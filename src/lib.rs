#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod binding;
mod command;
mod input;
mod palette;
mod plan;
mod ui;
mod visuals;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};

use crate::{
    binding::BindingPlugin, command::CommandPlugin, input::InputPlugin, plan::PlanPlugin,
    ui::UiPlugin, visuals::VisualsPlugin,
};

const VIEWPORT_SIZE: f32 = 10.0;

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AppSet {
    Input,
    Command,
    Visuals,
    Ui,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets((AppSet::Input, AppSet::Command, AppSet::Visuals, AppSet::Ui).chain())
            .add_plugin(PlanPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(BindingPlugin)
            .add_plugin(CommandPlugin)
            .add_plugin(VisualsPlugin)
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
