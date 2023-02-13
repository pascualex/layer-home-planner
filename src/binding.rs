use bevy::prelude::*;

use crate::{
    action::BindedAction,
    input::Hover,
    plan::{PlanMode, TrackMode},
    AppStage,
};

pub struct BindingPlugin;

impl Plugin for BindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            AppStage::Binding,
            SystemSet::new().with_system(process_bindings),
        );
    }
}

struct DefaultBindings;

impl DefaultBindings {
    fn bind(
        hover: &Hover,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
    ) -> BindedAction {
        if keyboard_input.just_pressed(KeyCode::E) {
            BindedAction::Create
        } else if let Some(hover) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                BindedAction::Select(hover)
            } else {
                BindedAction::None
            }
        } else {
            BindedAction::None
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
    ) -> BindedAction {
        if keyboard_input.just_pressed(KeyCode::G) {
            BindedAction::Track(selection)
        } else if keyboard_input.just_pressed(KeyCode::E) {
            BindedAction::Extend(selection)
        } else if keyboard_input.just_pressed(KeyCode::Delete) {
            BindedAction::Delete(selection)
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            BindedAction::Unselect
        } else if let Some(hover) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) && hover != selection {
                BindedAction::Select(hover)
            } else {
                BindedAction::None
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                BindedAction::Unselect
            } else {
                BindedAction::None
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
    ) -> BindedAction {
        if keyboard_input.just_pressed(KeyCode::Delete) {
            BindedAction::Delete(selection)
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            match mode {
                TrackMode::Move(old_position) => BindedAction::Move(selection, old_position),
                TrackMode::Place => BindedAction::Delete(selection),
            }
        } else if let Some(hover) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                BindedAction::Merge(selection, hover)
            } else {
                BindedAction::None
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                BindedAction::Select(selection)
            } else {
                BindedAction::None
            }
        }
    }
}

fn process_bindings(
    plan_mode: Res<PlanMode>,
    hover: Res<Hover>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut binded_action: ResMut<BindedAction>,
) {
    *binded_action = match *plan_mode {
        PlanMode::Default => DefaultBindings::bind(&hover, &mouse_input, &keyboard_input),
        PlanMode::Select(selection) => {
            SelectBindings::bind(selection, &hover, &mouse_input, &keyboard_input)
        }
        PlanMode::Track(selection, track_mode) => {
            TrackBindings::bind(selection, track_mode, &hover, &mouse_input, &keyboard_input)
        }
    }
}
