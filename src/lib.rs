#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod palette;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};
use bevy_prototype_lyon::prelude::*;

const VIEWPORT_SIZE: f32 = 20.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(palette::DARK_BLACK),
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(VIEWPORT_SIZE),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 99.9),
        ..default()
    });
    // circle
    let shape = shapes::RegularPolygon {
        feature: RegularPolygonFeature::Radius(0.125),
        sides: 20,
        ..default()
    };
    commands.spawn(GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill(FillMode::color(palette::LIGHT_WHITE)),
        Transform::default(),
    ));
}
