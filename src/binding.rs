use bevy::prelude::*;

use crate::{
    action::{Action, ActionQueue},
    input::Hover,
    plan::{PlanMode, TrackMode},
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
    ) -> Vec<Action> {
        if keyboard_input.just_pressed(KeyCode::E) {
            vec![Action::Create]
        } else if let Some(hover) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                vec![Action::Select(hover)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
}

struct SelectBindings;

impl SelectBindings {
    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    fn bind(
        selection: Entity,
        hover: &Hover,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
    ) -> Vec<Action> {
        if keyboard_input.just_pressed(KeyCode::G) {
            vec![Action::Track(selection)]
        } else if keyboard_input.just_pressed(KeyCode::E) {
            vec![Action::Extend(selection)]
        } else if keyboard_input.just_pressed(KeyCode::Delete) {
            vec![Action::Delete(selection)]
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            vec![Action::Unselect]
        } else if let Some(hover) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) && hover != selection {
                vec![Action::Select(hover)]
            } else {
                vec![]
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                vec![Action::Unselect]
            } else {
                vec![]
            }
        }
    }
}

struct TrackBindings;

impl TrackBindings {
    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    fn bind(
        selection: Entity,
        mode: TrackMode,
        hover: &Hover,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
    ) -> Vec<Action> {
        if keyboard_input.just_pressed(KeyCode::Delete) {
            vec![Action::Delete(selection)]
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            match mode {
                TrackMode::Move(old_position) => vec![Action::Move(selection, old_position)],
                TrackMode::Place => vec![Action::Delete(selection)],
            }
        } else if let Some(hover) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                vec![Action::Merge(selection, hover)]
            } else {
                vec![]
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                vec![Action::Select(selection)]
            } else {
                vec![]
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
) {
    let actions = match *plan_mode {
        PlanMode::Default => DefaultBindings::bind(&hover, &mouse_input, &keyboard_input),
        PlanMode::Select(selection) => {
            SelectBindings::bind(selection, &hover, &mouse_input, &keyboard_input)
        }
        PlanMode::Track(selection, track_mode) => {
            TrackBindings::bind(selection, track_mode, &hover, &mouse_input, &keyboard_input)
        }
    };
    for action in actions {
        action_queue.push_back(action);
    }
}
