use std::mem::take;

use bevy::prelude::*;

use crate::command::{
    system_command::{AddSystemCommand, RegisterSystemCommand},
    Action, UncommitedAction,
};

pub struct UndoCommandPlugin;

impl Plugin for UndoCommandPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UndoActions>()
            .init_resource::<RedoActions>()
            .register_system_command(commit_as_undo)
            .register_system_command(commit_as_undo_from_redo)
            .register_system_command(commit_as_redo)
            .register_system_command(discard_uncommited)
            .register_system_command(undo)
            .register_system_command(redo)
            .register_system_command(undo_uncommited);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct UndoActions(Vec<Action>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct RedoActions(Vec<Action>);

pub struct CommitAsUndo;

fn commit_as_undo(
    In(CommitAsUndo): In<CommitAsUndo>,
    mut uncommited_action: ResMut<UncommitedAction>,
    mut undo_actions: ResMut<UndoActions>,
    mut redo_actions: ResMut<RedoActions>,
) {
    if !uncommited_action.is_empty() {
        undo_actions.push(take(&mut **uncommited_action));
        redo_actions.clear();
    }
}

pub struct CommitAsUndoFromRedo;

fn commit_as_undo_from_redo(
    In(CommitAsUndoFromRedo): In<CommitAsUndoFromRedo>,
    mut uncommited_action: ResMut<UncommitedAction>,
    mut undo_actions: ResMut<UndoActions>,
) {
    if !uncommited_action.is_empty() {
        undo_actions.push(take(&mut **uncommited_action));
    }
}

pub struct CommitAsRedo;

fn commit_as_redo(
    In(CommitAsRedo): In<CommitAsRedo>,
    mut uncommited_action: ResMut<UncommitedAction>,
    mut redo_actions: ResMut<RedoActions>,
) {
    if !uncommited_action.is_empty() {
        redo_actions.push(take(&mut **uncommited_action));
    }
}

pub struct DiscardUncommitted;

fn discard_uncommited(
    In(DiscardUncommitted): In<DiscardUncommitted>,
    mut uncommited_action: ResMut<UncommitedAction>,
) {
    take(&mut **uncommited_action);
}

pub struct Undo;

fn undo(In(Undo): In<Undo>, mut undo_actions: ResMut<UndoActions>, mut commands: Commands) {
    if let Some(action) = undo_actions.pop() {
        for command in action.0.into_iter().rev() {
            command.add_to(&mut commands);
        }
        commands.add_system_command(CommitAsRedo);
    }
}

pub struct Redo;

fn redo(In(Redo): In<Redo>, mut redo_actions: ResMut<RedoActions>, mut commands: Commands) {
    if let Some(action) = redo_actions.pop() {
        for command in action.0.into_iter().rev() {
            command.add_to(&mut commands);
        }
        commands.add_system_command(CommitAsUndoFromRedo);
    }
}

pub struct UndoUncommitted;

fn undo_uncommited(
    In(UndoUncommitted): In<UndoUncommitted>,
    mut uncommited_action: ResMut<UncommitedAction>,
    mut commands: Commands,
) {
    let action = take(&mut **uncommited_action);
    for command in action.0.into_iter().rev() {
        command.add_to(&mut commands);
    }
}
