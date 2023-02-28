use std::{fmt::Debug, mem::take};

use bevy::prelude::*;

use crate::{
    command::{
        plan_mode::ChangeSelection,
        system_command::{AddSystemCommand, RegisterSystemCommand, SystemCommand},
    },
    plan::PlanMode,
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UndoActions>()
            .init_resource::<RedoActions>()
            .init_resource::<UncommittedCommands>()
            .register_system_command(undo)
            .register_system_command(redo)
            .register_system_command(undo_uncommitted)
            .register_system_command(discard_uncommitted)
            .register_system_command(commit_as_undo)
            .register_system_command(commit_as_undo_from_redo)
            .register_system_command(commit_as_redo);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct UndoActions(Vec<Action>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct RedoActions(Vec<Action>);

#[derive(Debug)]
pub struct Action {
    commands: Vec<Box<dyn AddToCommands + Send + Sync>>,
    selection: Selection,
}

impl Action {
    pub fn new(commands: Vec<Box<dyn AddToCommands + Send + Sync>>, selection: Selection) -> Self {
        Self {
            commands,
            selection,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Selection {
    Point(Entity),
    None,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct UncommittedCommands(Vec<Box<dyn AddToCommands + Send + Sync>>);

impl UncommittedCommands {
    pub fn add<T>(&mut self, command: T)
    where
        T: AddToCommands + 'static + Send + Sync,
    {
        self.push(Box::new(command));
    }
}

#[derive(Debug)]
pub struct Undo;

fn undo(In(Undo): In<Undo>, mut undo_actions: ResMut<UndoActions>, mut commands: Commands) {
    commands.add_system_command(UndoUncommitted);
    if let Some(mut action) = undo_actions.pop() {
        while let Some(command) = action.commands.pop() {
            command.add_to(&mut commands);
        }
        commands.add_system_command(CommitAsRedo(action.selection));
    }
    let new_selection = match undo_actions.last() {
        Some(action) => action.selection,
        None => Selection::None,
    };
    commands.add_system_command(ChangeSelection(new_selection));
}

#[derive(Debug)]
pub struct Redo;

fn redo(In(Redo): In<Redo>, mut redo_actions: ResMut<RedoActions>, mut commands: Commands) {
    if let Some(mut action) = redo_actions.pop() {
        commands.add_system_command(UndoUncommitted);
        while let Some(command) = action.commands.pop() {
            command.add_to(&mut commands);
        }
        commands.add_system_command(CommitAsUndoFromRedo(action.selection));
        commands.add_system_command(ChangeSelection(action.selection));
    }
}

#[derive(Debug)]
pub struct UndoUncommitted;

fn undo_uncommitted(
    In(UndoUncommitted): In<UndoUncommitted>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    mut commands: Commands,
) {
    while let Some(command) = uncommitted_commands.pop() {
        command.add_to(&mut commands);
    }
    commands.add_system_command(DiscardUncommitted);
}

#[derive(Debug)]
struct DiscardUncommitted;

fn discard_uncommitted(
    In(DiscardUncommitted): In<DiscardUncommitted>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    undo_actions: Res<UndoActions>,
    mut commands: Commands,
) {
    uncommitted_commands.clear();
    let new_selection = match undo_actions.last() {
        Some(action) => action.selection,
        None => Selection::None,
    };
    commands.add_system_command(ChangeSelection(new_selection));
}

#[derive(Debug)]
pub struct CommitAsUndo;

fn commit_as_undo(
    In(CommitAsUndo): In<CommitAsUndo>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    plan_mode: Res<PlanMode>,
    mut undo_actions: ResMut<UndoActions>,
    mut redo_actions: ResMut<RedoActions>,
) {
    let commands = take(&mut **uncommitted_commands);
    let selection = match *plan_mode {
        PlanMode::Normal => Selection::None,
        PlanMode::Point(point, _) => Selection::Point(point),
    };
    undo_actions.push(Action::new(commands, selection));
    redo_actions.clear();
}

#[derive(Debug)]
struct CommitAsUndoFromRedo(Selection);

fn commit_as_undo_from_redo(
    In(CommitAsUndoFromRedo(selection)): In<CommitAsUndoFromRedo>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    mut undo_actions: ResMut<UndoActions>,
) {
    let commands = take(&mut **uncommitted_commands);
    undo_actions.push(Action::new(commands, selection));
}

#[derive(Debug)]
struct CommitAsRedo(Selection);

fn commit_as_redo(
    In(CommitAsRedo(selection)): In<CommitAsRedo>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    mut redo_actions: ResMut<RedoActions>,
) {
    let commands = take(&mut **uncommitted_commands);
    redo_actions.push(Action::new(commands, selection));
}

pub trait AddToCommands: Debug {
    fn add_to(self: Box<Self>, commands: &mut Commands);
}

impl<T: 'static + Send + Debug> AddToCommands for T {
    fn add_to(self: Box<Self>, commands: &mut Commands) {
        commands.add(SystemCommand(*self));
    }
}
