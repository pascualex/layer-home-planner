use bevy::prelude::*;

use crate::{
    binding::{Binding, BindingHits},
    command::{
        action::UndoUncommitted,
        plan_mode::{SelectLine, SelectPoint, Unselect},
        point::{DeletePoint, TransferPointLines},
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    input::Hover,
    plan::{Element, TrackMode},
};

pub struct TrackPointBindingPlugin;

impl Plugin for TrackPointBindingPlugin {
    fn build(&self, app: &mut App) {
        app.register_system_command(custom_merge)
            .register_system_command(custom_cancel);
    }
}

pub struct TrackPointBindings {
    place: Binding,
    cancel: Binding,
}

impl TrackPointBindings {
    pub fn get_hits(
        &self,
        tracked_point: Entity,
        track_mode: TrackMode,
        hover: &Hover,
        hits: &mut BindingHits,
    ) {
        if let Element::Point(hovered_point) = **hover {
            match track_mode {
                TrackMode::Create => {
                    hits.no_commit(
                        "Cancel",
                        self.place,
                        CustomCancel(tracked_point, track_mode),
                    );
                }
                TrackMode::Move | TrackMode::Split(_) => {
                    hits.commit(
                        "Merge",
                        self.place,
                        CustomMerge(tracked_point, hovered_point),
                    );
                }
                TrackMode::Extend(extended_point) => {
                    if hovered_point == extended_point {
                        hits.no_commit(
                            "Cancel",
                            self.place,
                            CustomCancel(tracked_point, track_mode),
                        );
                    } else {
                        hits.commit(
                            "Merge",
                            self.place,
                            CustomMerge(tracked_point, hovered_point),
                        );
                    }
                }
            }
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

impl Default for TrackPointBindings {
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
        TrackMode::Create => {
            commands.add_system_command(Unselect);
        }
        TrackMode::Move => {
            commands.add_system_command(SelectPoint(tracked_point));
        }
        TrackMode::Extend(old_point) => {
            commands.add_system_command(SelectPoint(old_point));
        }
        TrackMode::Split(old_line) => {
            commands.add_system_command(SelectLine(old_line));
        }
    }
}
