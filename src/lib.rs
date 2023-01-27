#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod input;
mod line;
mod palette;
mod point;
mod tool;
mod ui;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};

use self::{
    input::InputPlugin, line::LinePlugin, point::PointPlugin, tool::ToolPlugin, ui::UiPlugin,
};

const VIEWPORT_SIZE: f32 = 10.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputPlugin)
            .add_plugin(LinePlugin)
            .add_plugin(PointPlugin)
            .add_plugin(ToolPlugin)
            .add_plugin(UiPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
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
}
