#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod action;
mod binding;
mod input;
mod palette;
mod plan;
mod ui;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};

use self::{
    action::ActionPlugin, binding::BindingPlugin, input::InputPlugin, plan::PlanPlugin,
    ui::UiPlugin,
};

const VIEWPORT_SIZE: f32 = 10.0;
const BASE_PRIORITY: f32 = 0.0;

#[derive(StageLabel)]
pub enum AppStage {
    Input,
    Binding,
    Action,
    Plan,
    Ui,
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            AppStage::Input,
            SystemStage::single_threaded(),
        )
        .add_stage_after(
            AppStage::Input,
            AppStage::Binding,
            SystemStage::single_threaded(),
        )
        .add_stage_after(
            AppStage::Binding,
            AppStage::Action,
            SystemStage::single_threaded(),
        )
        .add_stage_after(
            AppStage::Action,
            AppStage::Plan,
            SystemStage::single_threaded(),
        )
        .add_stage_after(AppStage::Plan, AppStage::Ui, SystemStage::single_threaded())
        .add_plugin(InputPlugin)
        .add_plugin(BindingPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(PlanPlugin)
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
