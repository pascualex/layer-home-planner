use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{palette, point::Point};

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLineEvent>()
            .add_system(spawn_lines)
            .add_system(spawn_when_available);
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
}

fn spawn_lines(
    mut events: EventReader<SpawnLineEvent>,
    query: Query<&Transform, With<Point>>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let Ok(transform_a) = query.get(event.point_a) else {
            continue;
        };
        let Ok(transform_b) = query.get(event.point_b) else {
            continue;
        };

        let position_a = transform_a.translation.truncate();
        let position_b = transform_b.translation.truncate();
        let position = (position_a + position_b) / 2.0;

        let shape = shapes::Line(position_a - position, position_b - position);
        commands.spawn((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Stroke(StrokeMode::color(palette::DARK_WHITE)),
                Transform::from_translation(position.extend(0.0)),
            ),
            Line::new(event.point_a, event.point_b),
        ));
        println!("Built!");
    }
}

fn spawn_when_available(
    query: Query<Entity, With<Point>>,
    mut events: EventWriter<SpawnLineEvent>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let mut point_a = None;
    let mut point_b = None;
    for entity in &query {
        if point_a.is_none() {
            point_a = Some(entity);
        } else {
            point_b = Some(entity);
            break;
        }
    }
    if let Some(point_a) = point_a {
        if let Some(point_b) = point_b {
            events.send(SpawnLineEvent::new(point_a, point_b));
            *done = true;
        }
    }
}
