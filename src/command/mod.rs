pub mod line;
pub mod plan_mode;
pub mod point;
pub mod system_command;

use std::borrow::Cow;

use bevy::{ecs::system::PipeSystem, prelude::*};

use self::{
    line::LineCommandPlugin, plan_mode::PlanModeCommandPlugin, point::PointCommandPlugin,
    system_command::RegisterSystemCommand,
};

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LineCommandPlugin)
            .add_plugin(PlanModeCommandPlugin)
            .add_plugin(PointCommandPlugin)
            .init_resource::<UndoCommands>();
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct UndoCommands(Vec<Box<dyn 'static + Send + Sync>>);

pub trait RegisterUndoSystemCommand {
    fn register_undo_system_command<T1: 'static + Send + Sync, T2: 'static + Send + Sync, P>(
        &mut self,
        system: impl IntoSystem<T1, T2, P>,
    ) -> &mut Self;
}

impl RegisterUndoSystemCommand for App {
    fn register_undo_system_command<T1: 'static + Send + Sync, T2: 'static + Send + Sync, P>(
        &mut self,
        system: impl IntoSystem<T1, T2, P>,
    ) -> &mut Self {
        let system_a = IntoSystem::into_system(system);
        let system_b = IntoSystem::into_system(add_undo_command::<T2>);
        let name = format!("Pipe({}, {})", system_a.name(), system_b.name());
        let pipe_system = PipeSystem::new(system_a, system_b, Cow::Owned(name));
        self.register_system_command(pipe_system)
    }
}

fn add_undo_command<T: 'static + Send + Sync>(
    In(command): In<T>,
    mut undo_commands: ResMut<UndoCommands>,
) {
    undo_commands.push(Box::new(command));
}
