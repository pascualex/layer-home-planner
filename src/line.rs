use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    palette,
    point::{ConnectPointEvent, Point},
    BASE_PRIORITY,
};

pub const LINE_WIDTH: f32 = 0.025;
pub const LINE_PRIORITY: f32 = BASE_PRIORITY + 1.0;

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLineEvent>()
            .add_system(spawn_lines)
            .add_system(update_lines);
    }
}

pub struct SpawnLineEvent {
    pub point_a: Entity,
    pub point_b: Entity,
}

impl SpawnLineEvent {
    pub fn new(point_a: Entity, point_b: Entity) -> Self {
        Self { point_a, point_b }
    }
}

#[derive(Component)]
pub struct Line {
    pub point_a: Entity,
    pub point_b: Entity,
}

impl Line {
    pub fn new(point_a: Entity, point_b: Entity) -> Self {
        Self { point_a, point_b }
    }

    pub fn other(&self, point: Entity) -> Option<Entity> {
        if point == self.point_a {
            Some(self.point_b)
        } else if point == self.point_b {
            Some(self.point_a)
        } else {
            None
        }
    }
}

fn spawn_lines(
    mut line_events: EventReader<SpawnLineEvent>,
    query: Query<&Transform, With<Point>>,
    mut point_events: EventWriter<ConnectPointEvent>,
    mut commands: Commands,
) {
    for event in line_events.iter() {
        let Ok(transform_a) = query.get(event.point_a) else {
            continue;
        };
        let Ok(transform_b) = query.get(event.point_b) else {
            continue;
        };

        let (position, local_a, local_b) = calculate_line(
            transform_a.translation.truncate(),
            transform_b.translation.truncate(),
        );

        let entity = commands
            .spawn((
                GeometryBuilder::build_as(
                    &shapes::Line(local_a, local_b),
                    DrawMode::Stroke(StrokeMode::new(palette::DARK_WHITE, LINE_WIDTH)),
                    Transform::from_translation(position.extend(LINE_PRIORITY)),
                ),
                Line::new(event.point_a, event.point_b),
            ))
            .id();

        point_events.send(ConnectPointEvent::new(event.point_a, entity));
        point_events.send(ConnectPointEvent::new(event.point_b, entity));
    }
}

fn update_lines(
    point_query: Query<(Entity, &Transform, &Point), (With<Point>, Changed<Transform>)>,
    mut line_query: Query<(&mut Transform, &mut Path, &Line), Without<Point>>,
) {
    for (point_a_entity, transform_a, point_a) in &point_query {
        for &line_entity in &point_a.lines {
            let Ok((mut line_transform, mut path, line)) = line_query.get_mut(line_entity) else {
                continue;
            };
            let Some(point_b_entity) = line.other(point_a_entity) else {
                continue;
            };
            let Ok((_, transform_b, _)) = point_query.get(point_b_entity) else {
                continue;
            };
            let (position, local_a, local_b) = calculate_line(
                transform_a.translation.truncate(),
                transform_b.translation.truncate(),
            );
            line_transform.translation = position.extend(LINE_PRIORITY);
            *path = ShapePath::build_as(&shapes::Line(local_a, local_b));
        }
    }
}

fn calculate_line(position_a: Vec2, position_b: Vec2) -> (Vec2, Vec2, Vec2) {
    (
        (position_a + position_b) / 2.0,
        (position_a - position_b) / 2.0,
        (position_b - position_a) / 2.0,
    )
}
