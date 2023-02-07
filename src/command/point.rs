use bevy::prelude::*;

use crate::{
    action::ActionHandling,
    command::CommandApplication,
    plan::{line::SpawnLineEvent, point::Selection, point::SpawnPointWithEntityEvent},
};

pub struct PointCommandPlugin;

impl Plugin for PointCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectionCommand>()
            .add_event::<CreationCommand>()
            .add_event::<ExtensionCommand>()
            .add_system(
                apply_selection_command
                    .label(CommandApplication)
                    .after(ActionHandling),
            )
            .add_system(
                apply_creation_command
                    .label(CommandApplication)
                    .after(ActionHandling),
            )
            .add_system(
                apply_extension_command
                    .label(CommandApplication)
                    .after(ActionHandling),
            );
    }
}

pub struct SelectionCommand {
    point: Option<Entity>,
}

impl SelectionCommand {
    pub fn select(point: Entity) -> Self {
        Self { point: Some(point) }
    }

    pub fn deselect() -> Self {
        Self { point: None }
    }
}

pub struct CreationCommand;

pub struct ExtensionCommand {
    point: Entity,
}

impl ExtensionCommand {
    pub fn new(point: Entity) -> Self {
        Self { point }
    }
}

fn apply_selection_command(
    mut command_events: EventReader<SelectionCommand>,
    mut selection: ResMut<Selection>,
) {
    for command_event in command_events.iter() {
        selection.point = command_event.point;
    }
}

fn apply_creation_command(
    mut command_events: EventReader<CreationCommand>,
    mut point_events: EventWriter<SpawnPointWithEntityEvent>,
    mut selection: ResMut<Selection>,
    mut commands: Commands,
) {
    for _ in command_events.iter() {
        let point = commands.spawn_empty().id();
        point_events.send(SpawnPointWithEntityEvent::new(point));
        selection.point = Some(point);
    }
}

fn apply_extension_command(
    mut command_events: EventReader<ExtensionCommand>,
    mut point_events: EventWriter<SpawnPointWithEntityEvent>,
    mut line_events: EventWriter<SpawnLineEvent>,
    mut selection: ResMut<Selection>,
    mut commands: Commands,
) {
    for command_event in command_events.iter() {
        let point_a = command_event.point;
        let point_b = commands.spawn_empty().id();
        point_events.send(SpawnPointWithEntityEvent::new(point_b));
        line_events.send(SpawnLineEvent::new(point_a, point_b));
        selection.point = Some(point_b);
    }
}
