mod normal;
mod select;
mod track;

use std::fmt::Debug;

use bevy::prelude::*;

use crate::{
    binding::{
        normal::{NormalBindings, NormalBindingsPlugin},
        select::{SelectBindings, SelectBindingsPlugin},
        track::{TrackBindings, TrackBindingsPlugin},
    },
    command::{
        action::{AddToCommands, CommitAsUndo, Redo, RedoActions, Undo, UndoActions},
        system_command::AddSystemCommand,
    },
    input::Hover,
    plan::{PlanMode, PointMode},
    AppSet,
};

pub struct BindingPlugin;

impl Plugin for BindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NormalBindingsPlugin)
            .add_plugin(SelectBindingsPlugin)
            .add_plugin(TrackBindingsPlugin)
            .init_resource::<Bindings>()
            .add_system(get_binding_hits.pipe(bind).in_set(AppSet::Binding));
    }
}

#[derive(Resource)]
pub struct Bindings {
    normal: NormalBindings,
    select: SelectBindings,
    track: TrackBindings,
    undo: Binding,
    redo: Binding,
}

impl Bindings {
    fn get_hits(
        &self,
        plan_mode: PlanMode,
        hover: &Hover,
        can_undo: bool,
        can_redo: bool,
        hits: &mut BindingHits,
    ) {
        match plan_mode {
            PlanMode::Normal => self.normal.get_hits(hover, hits),
            PlanMode::Point(selected_point, point_mode) => match point_mode {
                PointMode::Normal => self.select.get_hits(selected_point, hover, hits),
                PointMode::Track(cancel_point) => {
                    self.track
                        .get_hits(selected_point, cancel_point, hover, hits)
                }
            },
        }
        if can_undo {
            hits.no_commit(self.undo, Undo);
        }
        if can_redo {
            hits.no_commit(self.redo, Redo);
        }
    }
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            normal: NormalBindings::default(),
            select: SelectBindings::default(),
            track: TrackBindings::default(),
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
pub struct BindingHits(pub Vec<BindingHit>);

impl BindingHits {
    fn commit<T: 'static + Send + Debug>(&mut self, binding: Binding, command: T) {
        self.push(BindingHit {
            binding,
            command: Box::new(command),
            should_commit: true,
        });
    }

    fn no_commit<T: 'static + Send + Debug>(&mut self, binding: Binding, command: T) {
        self.push(BindingHit {
            binding,
            command: Box::new(command),
            should_commit: false,
        });
    }
}

pub struct BindingHit {
    pub binding: Binding,
    pub command: Box<dyn AddToCommands>,
    pub should_commit: bool,
}

pub fn get_binding_hits(
    bindings: Res<Bindings>,
    plan_mode: Res<PlanMode>,
    hover: Res<Hover>,
    undo_actions: Res<UndoActions>,
    redo_actions: Res<RedoActions>,
) -> BindingHits {
    let mut binding_hits = BindingHits::default();
    bindings.get_hits(
        *plan_mode,
        &hover,
        !undo_actions.is_empty(),
        !redo_actions.is_empty(),
        &mut binding_hits,
    );
    binding_hits
}

fn bind(
    In(binding_hits): In<BindingHits>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let binded_hit = binding_hits
        .0
        .into_iter()
        .find(|binding_hit| match binding_hit.binding {
            Binding::Mouse(mouse_button) => mouse_input.just_pressed(mouse_button),
            Binding::Keyboard(key_code) => keyboard_input.just_pressed(key_code),
        });
    if let Some(binded_hit) = binded_hit {
        binded_hit.command.add_to(&mut commands);
        if binded_hit.should_commit {
            commands.add_system_command(CommitAsUndo);
        }
    }
}
