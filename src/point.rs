use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{palette, tool::Selected, ZOOM};

pub const POINT_RADIUS: f32 = 5.0;

#[derive(SystemLabel)]
pub struct PointUpdate;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPointEvent>()
            .add_system(spawn_points.label(PointUpdate))
            .add_system(update_points_transform.label(PointUpdate));
    }
}

pub struct SpawnPointEvent {
    pub position: Vec2,
}

impl SpawnPointEvent {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

#[derive(Component)]
pub struct Point {
    pub position: Vec2,
}

impl Point {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

fn spawn_points(
    mut events: EventReader<SpawnPointEvent>,
    mut selected: ResMut<Selected>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let shape = shapes::Circle {
            radius: POINT_RADIUS,
            ..default()
        };
        let entity = commands
            .spawn((
                GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(palette::LIGHT_WHITE)),
                    Transform::from_translation(Vec3::new(
                        event.position.x * ZOOM,
                        event.position.y * ZOOM,
                        0.0,
                    )),
                ),
                Point::new(event.position),
            ))
            .id();
        selected.entity = Some(entity);
    }
}

fn update_points_transform(mut query: Query<(&mut Transform, &Point), Changed<Point>>) {
    for (mut transform, point) in &mut query {
        transform.translation.x = point.position.x * ZOOM;
        transform.translation.y = point.position.y * ZOOM;
    }
}
