use bevy::prelude::*;

use crate::{
    binding::{Binding, BindingHits},
    command::{
        action::UndoUncommitted,
        plan_mode::{SelectPoint, Unselect},
        point::{DeletePoint, TransferPointLines},
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    input::Hover,
    plan::TrackMode,
};

pub struct TrackBindingsPlugin;

impl Plugin for TrackBindingsPlugin {
    fn build(&self, app: &mut App) {
        app.register_system_command(custom_merge)
            .register_system_command(custom_cancel);
    }
}

pub struct TrackBindings {
    place: Binding,
    cancel: Binding,
}

impl TrackBindings {
    pub fn get_hits(
        &self,
        tracked_point: Entity,
        track_mode: TrackMode,
        hover: &Hover,
        hits: &mut BindingHits,
    ) {
        if let Some(hovered_point) = hover.point {
            hits.commit(
                "Merge",
                self.place,
                CustomMerge(tracked_point, hovered_point),
            );
        } else {
            hits.commit("Place", self.place, SelectPoint(tracked_point));
        }
        hits.no_commit(
            "Cancel",
            self.cancel,
            CustomCancel(tracked_point, track_mode),
        );
    }
}

impl Default for TrackBindings {
    fn default() -> Self {
        Self {
            place: Binding::Mouse(MouseButton::Left),
            cancel: Binding::Keyboard(KeyCode::Escape),
        }
    }
}

#[derive(Debug)]
struct CustomMerge(Entity, Entity);

fn custom_merge(In(CustomMerge(old_point, new_point)): In<CustomMerge>, mut commands: Commands) {
    commands.add_system_command(SelectPoint(new_point));
    commands.add_system_command(TransferPointLines(old_point, new_point));
    commands.add_system_command(DeletePoint(old_point));
}

#[derive(Debug)]
struct CustomCancel(Entity, TrackMode);

fn custom_cancel(
    In(CustomCancel(tracked_point, track_mode)): In<CustomCancel>,
    mut commands: Commands,
) {
    commands.add_system_command(UndoUncommitted);
    match track_mode {
        TrackMode::Create => commands.add_system_command(Unselect),
        TrackMode::Move => commands.add_system_command(SelectPoint(tracked_point)),
        TrackMode::Extend(old_point) => commands.add_system_command(SelectPoint(old_point)),
    }
}
