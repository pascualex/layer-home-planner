use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{palette, tool::Selected};

pub const POINT_RADIUS: f32 = 0.1;

#[derive(SystemLabel)]
pub struct PointUpdate;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPointEvent>()
            .add_system(spawn_points.label(PointUpdate));
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
pub struct Point;

fn spawn_points(
    mut events: EventReader<SpawnPointEvent>,
    mut selected: ResMut<Selected>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let shape = shapes::Rectangle {
            extents: Vec2::splat(POINT_RADIUS),
            ..default()
        };
        let entity = commands
            .spawn((
                GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill(FillMode::color(palette::LIGHT_WHITE)),
                    Transform {
                        translation: event.position.extend(0.0),
                        rotation: Quat::from_rotation_z(PI / 4.0),
                        ..default()
                    },
                ),
                Point,
            ))
            .id();
        selected.entity = Some(entity);
    }
}
