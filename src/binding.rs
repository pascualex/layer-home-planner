use bevy::prelude::*;

use crate::{
    command::{
        line::{CreateLine, DeleteLines, TransferLines},
        plan_mode::ChangePlanMode,
        point::{CreatePoint, DeletePoint, MovePoint},
        system_command::AddSystemCommand,
        undo::{CommitAsUndo, DiscardUncommitted, Redo, Undo, UndoUncommitted},
    },
    input::{Cursor, Hover},
    plan::{line::LineBlueprint, point::PointBlueprint, PlanMode},
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
        cursor: &Cursor,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
        commands: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::U) {
            if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift) {
                commands.add_system_command(Redo);
            } else {
                commands.add_system_command(Undo);
            }
        } else if let Some(hovered_point) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                commands.add_system_command(ChangePlanMode(PlanMode::Select(hovered_point)));
                commands.add_system_command(DiscardUncommitted);
            }
        } else if let Some(cursor_position) = cursor.position {
            if keyboard_input.just_pressed(KeyCode::E) {
                let new_point = commands.spawn_empty().id();
                commands.add_system_command(CreatePoint(
                    new_point,
                    PointBlueprint::new(cursor_position),
                ));
                commands.add_system_command(ChangePlanMode(PlanMode::Track(new_point)));
            }
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
        cursor: &Cursor,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
        commands: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::U) {
            if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift) {
                commands.add_system_command(Redo);
            } else {
                commands.add_system_command(Undo);
            }
        } else if keyboard_input.just_pressed(KeyCode::Delete) {
            commands.add_system_command(ChangePlanMode(PlanMode::Default));
            commands.add_system_command(DeleteLines(selection));
            commands.add_system_command(DeletePoint(selection));
            commands.add_system_command(CommitAsUndo);
        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            commands.add_system_command(ChangePlanMode(PlanMode::Default));
            commands.add_system_command(DiscardUncommitted);
        } else if let Some(hovered_point) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) && hovered_point != selection {
                commands.add_system_command(ChangePlanMode(PlanMode::Select(hovered_point)));
                commands.add_system_command(DiscardUncommitted);
            }
        } else if let Some(cursor_position) = cursor.position {
            if keyboard_input.just_pressed(KeyCode::G) {
                commands.add_system_command(MovePoint(selection, cursor_position));
                commands.add_system_command(ChangePlanMode(PlanMode::Track(selection)));
            } else if keyboard_input.just_pressed(KeyCode::E) {
                let new_point = commands.spawn_empty().id();
                let new_line = commands.spawn_empty().id();
                commands.add_system_command(CreatePoint(
                    new_point,
                    PointBlueprint::new(cursor_position),
                ));
                commands.add_system_command(CreateLine(
                    new_line,
                    LineBlueprint::new(selection, new_point),
                ));
                commands.add_system_command(ChangePlanMode(PlanMode::Track(new_point)));
            } else if mouse_input.just_pressed(MouseButton::Left) {
                commands.add_system_command(ChangePlanMode(PlanMode::Default));
                commands.add_system_command(DiscardUncommitted);
            }
        }
    }
}

struct TrackBindings;

impl TrackBindings {
    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    fn bind(
        tracked_entity: Entity,
        hover: &Hover,
        mouse_input: &Input<MouseButton>,
        keyboard_input: &Input<KeyCode>,
        commands: &mut Commands,
    ) {
        if keyboard_input.just_pressed(KeyCode::Escape) {
            commands.add_system_command(UndoUncommitted);
        } else if keyboard_input.just_pressed(KeyCode::U) {
            if !keyboard_input.pressed(KeyCode::LShift) && !keyboard_input.pressed(KeyCode::RShift)
            {
                commands.add_system_command(UndoUncommitted);
            }
        } else if let Some(hovered_point) = hover.point {
            if mouse_input.just_pressed(MouseButton::Left) {
                commands.add_system_command(ChangePlanMode(PlanMode::Select(hovered_point)));
                commands.add_system_command(TransferLines(tracked_entity, hovered_point));
                commands.add_system_command(DeletePoint(tracked_entity));
                commands.add_system_command(CommitAsUndo);
            }
        } else {
            if mouse_input.just_pressed(MouseButton::Left) {
                commands.add_system_command(ChangePlanMode(PlanMode::Select(tracked_entity)));
                commands.add_system_command(CommitAsUndo);
            }
        }
    }
}

fn process_bindings(
    plan_mode: Res<PlanMode>,
    hover: Res<Hover>,
    cursor: Res<Cursor>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    match *plan_mode {
        PlanMode::Default => DefaultBindings::bind(
            &hover,
            &cursor,
            &mouse_input,
            &keyboard_input,
            &mut commands,
        ),
        PlanMode::Select(selection) => SelectBindings::bind(
            selection,
            &hover,
            &cursor,
            &mouse_input,
            &keyboard_input,
            &mut commands,
        ),
        PlanMode::Track(selection) => TrackBindings::bind(
            selection,
            &hover,
            &mouse_input,
            &keyboard_input,
            &mut commands,
        ),
    };
}
