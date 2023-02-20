pub mod line;
pub mod plan_mode;
pub mod point;
pub mod system_command;
pub mod undo;

use std::borrow::Cow;

use bevy::{
    ecs::system::{Command, PipeSystem},
    prelude::*,
};

use self::{
    line::LineCommandPlugin,
    plan_mode::PlanModeCommandPlugin,
    point::PointCommandPlugin,
    system_command::{RegisterSystemCommand, SystemCommand},
    undo::UndoCommandPlugin,
};

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LineCommandPlugin)
            .add_plugin(PlanModeCommandPlugin)
            .add_plugin(PointCommandPlugin)
            .add_plugin(UndoCommandPlugin)
            .init_resource::<UncommitedAction>();
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct UncommitedAction(Action);

#[derive(Default, Deref, DerefMut)]
pub struct Action(Vec<Box<dyn AddToCommands + Send + Sync>>);

pub trait AddToCommands {
    fn add_to(self: Box<Self>, commands: &mut Commands);
}

impl<C: Command> AddToCommands for C {
    fn add_to(self: Box<Self>, commands: &mut Commands) {
        commands.add(*self);
    }
}

pub trait RegisterUndoableSystemCommand {
    fn register_undoable_system_command<T1: 'static + Send + Sync, T2: 'static + Send + Sync, P>(
        &mut self,
        system: impl IntoSystem<T1, T2, P>,
    ) -> &mut Self;
}

impl RegisterUndoableSystemCommand for App {
    fn register_undoable_system_command<T1: 'static + Send + Sync, T2: 'static + Send + Sync, P>(
        &mut self,
        system: impl IntoSystem<T1, T2, P>,
    ) -> &mut Self {
        let system_a = IntoSystem::into_system(system);
        let system_b = IntoSystem::into_system(push_command::<T2>);
        let name = format!("Pipe({}, {})", system_a.name(), system_b.name());
        let pipe_system = PipeSystem::new(system_a, system_b, Cow::Owned(name));
        self.register_system_command(pipe_system)
    }
}

fn push_command<T: 'static + Send + Sync>(
    In(command): In<T>,
    mut uncommited_action: ResMut<UncommitedAction>,
) {
    uncommited_action.push(Box::new(SystemCommand(command)));
}
