#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod input;
mod palette;
mod ui;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};
use bevy_prototype_lyon::prelude::*;

use self::{
    input::{Cursor, InputPlugin, InputUpdate},
    ui::UiPlugin,
};

const VIEWPORT_SIZE: f32 = 5.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputPlugin)
            .add_plugin(UiPlugin)
            .add_startup_system(setup)
            .add_system(follow.after(InputUpdate));
    }
}

#[derive(Component)]
struct Follow;

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
        feature: RegularPolygonFeature::Radius(0.05),
        sides: 20,
        ..default()
    };
    commands.spawn((
        GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(palette::LIGHT_WHITE)),
            Transform::default(),
        ),
        Follow,
    ));
}

fn follow(mut query: Query<&mut Transform, With<Follow>>, cursor: Res<Cursor>) {
    for mut transform in &mut query {
        if let Some(position) = cursor.position {
            let decimals = if cursor.alt { 2 } else { 1 };
            transform.translation.x = round(position.x, decimals);
            transform.translation.y = round(position.y, decimals);
        }
    }
}

fn round(number: f32, decimals: u32) -> f32 {
    let offset = 10_i32.pow(decimals) as f32;
    (number * offset).round() / offset
}
