use bevy::prelude::*;

use crate::{
    action::{ActionHandling, ActionState},
    command::{CreationCommand, ExtensionCommand, SelectionCommand},
    input::{Cursor, Hover, InputProcessing},
    plan::{point::Point, Selection},
};

pub struct PointActionPlugin;

impl Plugin for PointActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            move_selection_to_cursor
                .label(ActionHandling)
                .after(InputProcessing),
        )
        .add_system(
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

fn move_selection_to_cursor(
    cursor: Res<Cursor>,
    input: Res<Input<KeyCode>>,
    selection: Res<Selection>,
    mut query: Query<&mut Transform, With<Point>>,
) {
    let Some(entity) = selection.point else {
        return;
    };
    let Ok(mut transform) = query.get_mut(entity) else {
        return;
    };
    if let Some(position) = cursor.position {
        let decimals = if input.pressed(KeyCode::LAlt) { 2 } else { 1 };
        transform.translation.x = round(position.x, decimals);
        transform.translation.y = round(position.y, decimals);
    }
}

fn round(number: f32, decimals: u32) -> f32 {
    let offset = 10_i32.pow(decimals) as f32;
    (number * offset).round() / offset
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
