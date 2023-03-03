mod line;
mod normal;
mod point;

use std::fmt::Debug;

use bevy::prelude::*;

use crate::{
    binding::{
        line::{LineBindingPlugin, LineBindings},
        normal::{NormalBindingPlugin, NormalBindings},
        point::{PointBindingPlugin, PointBindings},
    },
    command::action::{AddToCommands, Redo, RedoActions, Undo, UndoActions},
    input::Hover,
    plan::PlanMode,
};

pub struct BindingPlugin;

impl Plugin for BindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NormalBindingPlugin)
            .add_plugin(PointBindingPlugin)
            .add_plugin(LineBindingPlugin)
            .init_resource::<Bindings>();
    }
}

#[derive(Resource)]
pub struct Bindings {
    normal: NormalBindings,
    point: PointBindings,
    line: LineBindings,
    undo: Binding,
    redo: Binding,
}

impl Bindings {
    fn bind(
        &self,
        plan_mode: PlanMode,
        hover: &Hover,
        can_undo: bool,
        can_redo: bool,
        commands: &mut BindedCommands,
    ) {
        match plan_mode {
            PlanMode::Normal => {
                self.normal.bind(hover, commands);
            }
            PlanMode::Point(selected_point, point_mode) => {
                self.point.bind(selected_point, point_mode, hover, commands);
            }
            PlanMode::Line(selected_line) => {
                self.line.bind(selected_line, hover, commands);
            }
        }
        if can_undo {
            commands.no_commit("Undo", self.undo, Undo);
        }
        if can_redo {
            commands.no_commit("Redo", self.redo, Redo);
        }
    }
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            normal: NormalBindings::default(),
            point: PointBindings::default(),
            line: LineBindings::default(),
            undo: Binding::Keyboard(KeyCode::U),
            redo: Binding::Keyboard(KeyCode::R),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Binding {
    Mouse(MouseButton),
    Keyboard(KeyCode),
}

#[derive(Default, Deref, DerefMut)]
pub struct BindedCommands(pub Vec<BindedCommand>);

impl BindedCommands {
    fn commit<T: 'static + Send + Debug>(
        &mut self,
        name: &'static str,
        binding: Binding,
        command: T,
    ) {
        self.push(BindedCommand {
            name,
            binding,
            command: Box::new(command),
            should_commit: true,
        });
    }

    fn no_commit<T: 'static + Send + Debug>(
        &mut self,
        name: &'static str,
        binding: Binding,
        command: T,
    ) {
        self.push(BindedCommand {
            name,
            binding,
            command: Box::new(command),
            should_commit: false,
        });
    }
}

pub struct BindedCommand {
    pub name: &'static str,
    pub binding: Binding,
    pub command: Box<dyn AddToCommands>,
    pub should_commit: bool,
}

pub fn bind(
    bindings: Res<Bindings>,
    plan_mode: Res<PlanMode>,
    hover: Res<Hover>,
    undo_actions: Res<UndoActions>,
    redo_actions: Res<RedoActions>,
) -> BindedCommands {
    let mut binding_hits = BindedCommands::default();
    bindings.bind(
        *plan_mode,
        &hover,
        !undo_actions.is_empty(),
        !redo_actions.is_empty(),
        &mut binding_hits,
    );
    binding_hits
}
