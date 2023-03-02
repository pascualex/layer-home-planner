use std::{
    fmt::Debug,
    mem::{replace, take},
};

use bevy::prelude::*;

use crate::{
    command::{
        plan_mode::ChangeSelection,
        system_command::{AddSystemCommand, RegisterSystemCommand, SystemCommand},
    },
    plan::{Element, PlanMode},
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UndoActions>()
            .init_resource::<RedoActions>()
            .init_resource::<UncommittedCommands>()
            .init_resource::<UncommittedSelection>()
            .register_system_command(undo)
            .register_system_command(redo)
            .register_system_command(undo_uncommitted)
            .register_system_command(discard_redo)
            .register_system_command(discard_uncommitted)
            .register_system_command(commit)
            .register_system_command(commit_as_undo)
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
    old_selection: Element,
    new_selection: Element,
}

impl Action {
    pub fn new(
        commands: Vec<Box<dyn AddToCommands + Send + Sync>>,
        old_selection: Element,
        new_selection: Element,
    ) -> Self {
        Self {
            commands,
            old_selection,
            new_selection,
        }
    }
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

#[derive(Resource, Default, Deref, DerefMut)]
pub struct UncommittedSelection(Element);

#[derive(Debug)]
pub struct Undo;

fn undo(In(Undo): In<Undo>, mut undo_actions: ResMut<UndoActions>, mut commands: Commands) {
    if let Some(mut action) = undo_actions.pop() {
        commands.add_system_command(UndoUncommitted);
        while let Some(command) = action.commands.pop() {
            command.add_to(&mut commands);
        }
        commands.add_system_command(CommitAsRedo(action.old_selection, action.new_selection));
        commands.add_system_command(ChangeSelection(action.old_selection));
    }
}

#[derive(Debug)]
pub struct Redo;

fn redo(In(Redo): In<Redo>, mut redo_actions: ResMut<RedoActions>, mut commands: Commands) {
    if let Some(mut action) = redo_actions.pop() {
        commands.add_system_command(UndoUncommitted);
        while let Some(command) = action.commands.pop() {
            command.add_to(&mut commands);
        }
        commands.add_system_command(CommitAsUndo(action.old_selection, action.new_selection));
        commands.add_system_command(ChangeSelection(action.new_selection));
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
struct DiscardRedo;

fn discard_redo(In(DiscardRedo): In<DiscardRedo>, mut redo_actions: ResMut<RedoActions>) {
    redo_actions.clear();
}

#[derive(Debug)]
struct DiscardUncommitted;

fn discard_uncommitted(
    In(DiscardUncommitted): In<DiscardUncommitted>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
) {
    uncommitted_commands.clear();
}

#[derive(Debug)]
pub struct Commit;

fn commit(
    In(Commit): In<Commit>,
    mut uncommitted_selection: ResMut<UncommittedSelection>,
    plan_mode: Res<PlanMode>,
    mut commands: Commands,
) {
    let new_selection = plan_mode.selection();
    let old_selection = replace(&mut **uncommitted_selection, new_selection);
    commands.add_system_command(CommitAsUndo(old_selection, new_selection));
    commands.add_system_command(DiscardRedo);
}

#[derive(Debug)]
struct CommitAsUndo(Element, Element);

fn commit_as_undo(
    In(CommitAsUndo(old_selection, new_selection)): In<CommitAsUndo>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    mut undo_actions: ResMut<UndoActions>,
) {
    let commands = take(&mut **uncommitted_commands);
    undo_actions.push(Action::new(commands, old_selection, new_selection));
}

#[derive(Debug)]
struct CommitAsRedo(Element, Element);

fn commit_as_redo(
    In(CommitAsRedo(old_selection, new_selection)): In<CommitAsRedo>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    mut redo_actions: ResMut<RedoActions>,
) {
    let commands = take(&mut **uncommitted_commands);
    redo_actions.push(Action::new(commands, old_selection, new_selection));
}

pub trait AddToCommands: Debug {
    fn add_to(self: Box<Self>, commands: &mut Commands);
}

impl<T: 'static + Send + Debug> AddToCommands for T {
    fn add_to(self: Box<Self>, commands: &mut Commands) {
        commands.add(SystemCommand(*self));
    }
}
