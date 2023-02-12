use bevy::prelude::*;

use crate::{
    action::{Action, BindedAction},
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
    mut binded: ResMut<BindedAction>,
) {
    binded.action = None;
    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    match *plan_mode {
        PlanMode::Default => {
            if keyboard_input.just_pressed(KeyCode::E) {
                binded.action = Some(Action::Create);
            } else if let Some(hover) = hover.point {
                if mouse_input.just_pressed(MouseButton::Left) {
                    binded.action = Some(Action::Select(hover));
                }
            }
        }
        PlanMode::Select(selection) => {
            if keyboard_input.just_pressed(KeyCode::G) {
                binded.action = Some(Action::Track(selection));
            } else if keyboard_input.just_pressed(KeyCode::E) {
                binded.action = Some(Action::Extend(selection));
            } else if keyboard_input.just_pressed(KeyCode::Delete) {
                binded.action = Some(Action::Delete(selection))
            } else if keyboard_input.just_pressed(KeyCode::Escape) {
                binded.action = Some(Action::Deselect);
            } else if let Some(hover) = hover.point {
                if mouse_input.just_pressed(MouseButton::Left) && hover != selection {
                    binded.action = Some(Action::Select(hover));
                }
            } else {
                if mouse_input.just_pressed(MouseButton::Left) {
                    binded.action = Some(Action::Deselect);
                }
            }
        }
        PlanMode::Track(selection, track_mode) => {
            if keyboard_input.just_pressed(KeyCode::Delete) {
                binded.action = Some(Action::Delete(selection));
            } else if keyboard_input.just_pressed(KeyCode::Escape) {
                match track_mode {
                    TrackMode::Move(old_position) => {
                        binded.action = Some(Action::Move(selection, old_position));
                    }
                    TrackMode::Place => {
                        binded.action = Some(Action::Delete(selection));
                    }
                }
            } else if let Some(hover) = hover.point {
                if mouse_input.just_pressed(MouseButton::Left) {
                    binded.action = Some(Action::Merge(selection, hover));
                }
            } else {
                if mouse_input.just_pressed(MouseButton::Left) {
                    binded.action = Some(Action::Select(selection));
                }
            }
        }
    }
}
