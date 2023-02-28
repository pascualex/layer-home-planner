pub mod action;
pub mod line;
pub mod plan_mode;
pub mod point;
pub mod system_command;

use bevy::prelude::*;

use crate::{
    command::{
        action::ActionPlugin, line::LineCommandPlugin, plan_mode::PlanModeCommandPlugin,
        point::PointCommandPlugin,
    },
    AppSet,
};

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LineCommandPlugin)
            .add_plugin(PlanModeCommandPlugin)
            .add_plugin(PointCommandPlugin)
            .add_plugin(ActionPlugin)
            .add_system(apply_system_buffers.in_set(AppSet::Command));
    }
}
