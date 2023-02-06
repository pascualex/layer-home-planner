#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod action;
mod command;
mod input;
mod inspector;
mod palette;
mod plan;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};

use self::{
    action::ActionPlugin, command::CommandPlugin, input::InputPlugin, inspector::InspectorPlugin,
    plan::PlanPlugin,
};

const VIEWPORT_SIZE: f32 = 10.0;
const BASE_PRIORITY: f32 = 0.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionPlugin)
            .add_plugin(CommandPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(InspectorPlugin)
            .add_plugin(PlanPlugin)
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
