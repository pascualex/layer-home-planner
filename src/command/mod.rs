pub mod action;
pub mod line;
pub mod plan_mode;
pub mod point;
pub mod system_command;

use bevy::prelude::*;

use crate::{
    binding::{bind, BindedCommands, Binding},
    command::{
        action::{ActionPlugin, Commit},
        line::LineCommandPlugin,
        plan_mode::PlanModeCommandPlugin,
        point::PointCommandPlugin,
        system_command::AddSystemCommand,
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
            .add_systems(
                (bind.pipe(process_binded_commands), apply_system_buffers)
                    .chain()
                    .in_set(AppSet::Command),
            );
    }
}

fn process_binded_commands(
    In(binded_commands): In<BindedCommands>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let triggered_command =
        binded_commands
            .0
            .into_iter()
            .find(|binding_hit| match binding_hit.binding {
                Binding::Mouse(mouse_button) => mouse_input.just_pressed(mouse_button),
                Binding::Keyboard(key_code) => keyboard_input.just_pressed(key_code),
            });
    if let Some(triggered_command) = triggered_command {
        triggered_command.command.add_to(&mut commands);
        if triggered_command.should_commit {
            commands.add_system_command(Commit);
        }
    }
}
