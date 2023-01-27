use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{line::LINE_PRIORITY, palette};

pub const POINT_RADIUS: f32 = 0.1;
pub const POINT_PRIORITY: f32 = LINE_PRIORITY + 1.0;

#[derive(SystemLabel)]
pub struct PointSpawn;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPointWithEntityEvent>()
            .add_system(spawn_points.label(PointSpawn));
    }
}

pub struct SpawnPointWithEntityEvent {
    pub entity: Entity,
    pub position: Vec2,
}

impl SpawnPointWithEntityEvent {
    pub fn new(entity: Entity, position: Vec2) -> Self {
        Self { entity, position }
    }
}

#[derive(Component)]
pub struct Point;

fn spawn_points(mut events: EventReader<SpawnPointWithEntityEvent>, mut commands: Commands) {
    for event in events.iter() {
        let shape = shapes::Rectangle {
            extents: Vec2::splat(POINT_RADIUS),
            ..default()
        };
        commands.entity(event.entity).insert((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(palette::LIGHT_WHITE)),
                Transform {
                    translation: event.position.extend(POINT_PRIORITY),
                    rotation: Quat::from_rotation_z(PI / 4.0),
                    ..default()
                },
            ),
            Point,
        ));
    }
}
