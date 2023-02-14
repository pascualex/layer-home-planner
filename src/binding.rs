use bevy::prelude::*;

use crate::{
    action::{Action, ActionBundle, TrackReason},
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
        commands: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::E) {
            let entity = commands.spawn_empty().id();
            commands.insert_resource(ActionBundle::new(vec![
                Action::Create(entity),
                Action::Track(entity, TrackReason::Create),
            ]));
        } else if let Some(hover_entity) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                commands.insert_resource(ActionBundle::new(vec![Action::Select(hover_entity)]));
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
        commands: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::G) {
            commands.insert_resource(ActionBundle::new(vec![Action::Track(
                selection_entity,
                TrackReason::Move,
            )]));
        } else if keyboard_input.just_pressed(KeyCode::E) {
            let entity = commands.spawn_empty().id();
            commands.insert_resource(ActionBundle::new(vec![
                Action::Create(entity),
                Action::Connect(selection_entity, entity),
                Action::Track(entity, TrackReason::Extend(selection_entity)),
            ]));
        } else if keyboard_input.just_pressed(KeyCode::Delete) {
            commands.insert_resource(ActionBundle::new(vec![
                Action::Delete(selection_entity),
                Action::Unselect,
            ]));
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            commands.insert_resource(ActionBundle::new(vec![Action::Unselect]));
        } else if let Some(hover) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) && hover != selection_entity {
                commands.insert_resource(ActionBundle::new(vec![Action::Select(hover)]));
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                commands.insert_resource(ActionBundle::new(vec![Action::Unselect]));
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
        commands: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::Delete) {
            commands.insert_resource(ActionBundle::new(vec![
                Action::Delete(selection_entity),
                Action::Unselect,
            ]));
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            match cancelation_mode {
                CancelationMode::Move(old_position) => {
                    commands.insert_resource(ActionBundle::new(vec![
                        Action::Move(selection_entity, old_position),
                        Action::Select(selection_entity),
                    ]));
                }
                CancelationMode::Destroy => {
                    commands.insert_resource(ActionBundle::new(vec![
                        Action::Delete(selection_entity),
                        Action::Unselect,
                    ]));
                }
                CancelationMode::DestroyAndSelect(old_entity) => {
                    commands.insert_resource(ActionBundle::new(vec![
                        Action::Delete(selection_entity),
                        Action::Select(old_entity),
                    ]));
                }
            }
        } else if let Some(hover_entity) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                commands.insert_resource(ActionBundle::new(vec![
                    Action::Merge(selection_entity, hover_entity),
                    Action::Select(hover_entity),
                ]));
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                commands.insert_resource(ActionBundle::new(vec![Action::Select(selection_entity)]));
            }
        }
    }
}

fn process_bindings(
    plan_mode: Res<PlanMode>,
    hover: Res<Hover>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    match *plan_mode {
        PlanMode::Default => {
            DefaultBindings::bind(&hover, &mouse_input, &keyboard_input, &mut commands)
        }
        PlanMode::Select(selection) => SelectBindings::bind(
            selection,
            &hover,
            &mouse_input,
            &keyboard_input,
            &mut commands,
        ),
        PlanMode::Track(selection, cancelation_mode) => TrackBindings::bind(
            selection,
            cancelation_mode,
            &hover,
            &mouse_input,
            &keyboard_input,
            &mut commands,
        ),
    };
}
