use bevy::prelude::*;

use crate::{
    action::{ActionHandling, ActionState},
    command::{CreationCommand, ExtensionCommand, SelectionCommand},
    input::{Hover, InputProcessing},
};

pub struct PointActionPlugin;

impl Plugin for PointActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            handle_select_action
                .label(ActionHandling)
                .after(InputProcessing),
        )
        .add_system(
            handle_deselect_action
                .label(ActionHandling)
                .after(InputProcessing),
        )
        .add_system(
            handle_create_action
                .label(ActionHandling)
                .after(InputProcessing),
        )
        .add_system(
            handle_extend_action
                .label(ActionHandling)
                .after(InputProcessing),
        );
    }
}

fn handle_select_action(
    action_state: Res<ActionState>,
    hover: Res<Hover>,
    mut events: EventWriter<SelectionCommand>,
) {
    if matches!(*action_state, ActionState::Select) {
        let point = hover.point.unwrap();
        events.send(SelectionCommand::select(point));
    }
}

fn handle_deselect_action(
    action_state: Res<ActionState>,
    mut events: EventWriter<SelectionCommand>,
) {
    if matches!(*action_state, ActionState::Deselect) {
        events.send(SelectionCommand::deselect());
    }
}

fn handle_create_action(action_state: Res<ActionState>, mut events: EventWriter<CreationCommand>) {
    if matches!(*action_state, ActionState::Create) {
        events.send(CreationCommand);
    }
}

fn handle_extend_action(
    action_state: Res<ActionState>,
    hover: Res<Hover>,
    mut events: EventWriter<ExtensionCommand>,
) {
    if matches!(*action_state, ActionState::Extend) {
        let point = hover.point.unwrap();
        events.send(ExtensionCommand::new(point));
    }
}
