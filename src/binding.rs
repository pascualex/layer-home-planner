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

fn process_bindings(
    plan_mode: Res<PlanMode>,
    hover: Res<Hover>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut binded_action: ResMut<BindedAction>,
) {
    *binded_action = BindedAction::None;
    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    match *plan_mode {
        PlanMode::Default => {
            if keyboard_input.just_pressed(KeyCode::E) {
                *binded_action = BindedAction::Create;
            } else if let Some(hover) = hover.point {
                if mouse_input.just_pressed(MouseButton::Left) {
                    *binded_action = BindedAction::Select(hover);
                }
            }
        }
        PlanMode::Select(selection) => {
            if keyboard_input.just_pressed(KeyCode::G) {
                *binded_action = BindedAction::Track(selection);
            } else if keyboard_input.just_pressed(KeyCode::E) {
                *binded_action = BindedAction::Extend(selection);
            } else if keyboard_input.just_pressed(KeyCode::Delete) {
                *binded_action = BindedAction::Delete(selection)
            } else if keyboard_input.just_pressed(KeyCode::Escape) {
                *binded_action = BindedAction::Unselect;
            } else if let Some(hover) = hover.point {
                if mouse_input.just_pressed(MouseButton::Left) && hover != selection {
                    *binded_action = BindedAction::Select(hover);
                }
            } else {
                if mouse_input.just_pressed(MouseButton::Left) {
                    *binded_action = BindedAction::Unselect;
                }
            }
        }
        PlanMode::Track(selection, track_mode) => {
            if keyboard_input.just_pressed(KeyCode::Delete) {
                *binded_action = BindedAction::Delete(selection);
            } else if keyboard_input.just_pressed(KeyCode::Escape) {
                match track_mode {
                    TrackMode::Move(old_position) => {
                        *binded_action = BindedAction::Move(selection, old_position);
                    }
                    TrackMode::Place => {
                        *binded_action = BindedAction::Delete(selection);
                    }
                }
            } else if let Some(hover) = hover.point {
                if mouse_input.just_pressed(MouseButton::Left) {
                    *binded_action = BindedAction::Merge(selection, hover);
                }
            } else {
                if mouse_input.just_pressed(MouseButton::Left) {
                    *binded_action = BindedAction::Select(selection);
                }
            }
        }
    }
}
