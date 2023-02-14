use bevy::prelude::*;

use crate::{
    action::{Action, ActionQueue, TrackReason},
    input::Hover,
    plan::{CancelationMode, PlanMode},
    AppSet,
};

pub struct BindingPlugin;

impl Plugin for BindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(process_bindings.in_set(AppSet::Binding));
    }
}

struct DefaultBindings;

impl DefaultBindings {
    fn bind(
        hover: &Hover,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
        action_queue: &mut ActionQueue,
        commands: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::E) {
            let entity = commands.spawn_empty().id();
            action_queue.push_back(vec![
                Action::CreatePoint(entity),
                Action::TrackPoint(entity, TrackReason::Create),
            ]);
        } else if let Some(hover_entity) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                action_queue.push_back(vec![Action::SelectPoint(hover_entity)]);
            }
        }
    }
}

struct SelectBindings;

impl SelectBindings {
    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    fn bind(
        selection_entity: Entity,
        hover: &Hover,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
        action_queue: &mut ActionQueue,
        commands: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::G) {
            action_queue.push_back(vec![Action::TrackPoint(
                selection_entity,
                TrackReason::Move,
            )]);
        } else if keyboard_input.just_pressed(KeyCode::E) {
            let entity = commands.spawn_empty().id();
            action_queue.push_back(vec![
                Action::CreatePoint(entity),
                Action::CreateLine(selection_entity, entity),
                Action::TrackPoint(entity, TrackReason::Extend(selection_entity)),
            ]);
        } else if keyboard_input.just_pressed(KeyCode::Delete) {
            action_queue.push_back(vec![
                Action::DeletePoint(selection_entity),
                Action::UnselectPoint,
            ]);
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            action_queue.push_back(vec![Action::UnselectPoint]);
        } else if let Some(hover) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) && hover != selection_entity {
                action_queue.push_back(vec![Action::SelectPoint(hover)]);
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                action_queue.push_back(vec![Action::UnselectPoint]);
            }
        }
    }
}

struct TrackBindings;

impl TrackBindings {
    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    fn bind(
        selection_entity: Entity,
        cancelation_mode: CancelationMode,
        hover: &Hover,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
        action_queue: &mut ActionQueue,
        _: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::Delete) {
            action_queue.push_back(vec![
                Action::DeletePoint(selection_entity),
                Action::UnselectPoint,
            ]);
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            match cancelation_mode {
                CancelationMode::Move(old_position) => {
                    action_queue.push_back(vec![
                        Action::MovePoint(selection_entity, old_position),
                        Action::SelectPoint(selection_entity),
                    ]);
                }
                CancelationMode::Destroy => {
                    action_queue.push_back(vec![
                        Action::DeletePoint(selection_entity),
                        Action::UnselectPoint,
                    ]);
                }
                CancelationMode::DestroyAndSelect(old_entity) => {
                    action_queue.push_back(vec![
                        Action::DeletePoint(selection_entity),
                        Action::SelectPoint(old_entity),
                    ]);
                }
            }
        } else if let Some(hover_entity) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                action_queue.push_back(vec![
                    Action::TransferLines(selection_entity, hover_entity),
                    Action::SelectPoint(hover_entity),
                ]);
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                action_queue.push_back(vec![Action::SelectPoint(selection_entity)]);
            }
        }
    }
}

fn process_bindings(
    plan_mode: Res<PlanMode>,
    hover: Res<Hover>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut action_queue: ResMut<ActionQueue>,
    mut commands: Commands,
) {
    match *plan_mode {
        PlanMode::Default => DefaultBindings::bind(
            &hover,
            &mouse_input,
            &keyboard_input,
            &mut action_queue,
            &mut commands,
        ),
        PlanMode::Select(selection) => SelectBindings::bind(
            selection,
            &hover,
            &mouse_input,
            &keyboard_input,
            &mut action_queue,
            &mut commands,
        ),
        PlanMode::Track(selection, cancelation_mode) => TrackBindings::bind(
            selection,
            cancelation_mode,
            &hover,
            &mouse_input,
            &keyboard_input,
            &mut action_queue,
            &mut commands,
        ),
    };
}
