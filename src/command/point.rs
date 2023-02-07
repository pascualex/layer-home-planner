use bevy::prelude::*;

use crate::{
    plan::{
        line::SpawnLineInstruction,
        point::SpawnPointInstruction,
        point::{ConnectPointInstruction, Selection},
    },
    AppStage,
};

pub struct PointCommandPlugin;

impl Plugin for PointCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectionCommand>()
            .add_event::<CreationCommand>()
            .add_event::<ExtensionCommand>()
            .add_system_set_to_stage(
                AppStage::Command,
                SystemSet::new()
                    .with_system(apply_selection_command)
                    .with_system(apply_creation_command)
                    .with_system(apply_extension_command),
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
    for command in command_events.iter() {
        selection.point = command.point;
    }
}

fn apply_creation_command(
    mut command_events: EventReader<CreationCommand>,
    mut instructions: EventWriter<SpawnPointInstruction>,
    mut selection: ResMut<Selection>,
    mut commands: Commands,
) {
    for _ in command_events.iter() {
        let point = commands.spawn_empty().id();
        instructions.send(SpawnPointInstruction::new(point));
        selection.point = Some(point);
    }
}

fn apply_extension_command(
    mut command_events: EventReader<ExtensionCommand>,
    mut spawn_point_instructions: EventWriter<SpawnPointInstruction>,
    mut connect_point_instructions: EventWriter<ConnectPointInstruction>,
    mut line_instructions: EventWriter<SpawnLineInstruction>,
    mut selection: ResMut<Selection>,
    mut commands: Commands,
) {
    for command in command_events.iter() {
        let point_a = command.point;
        let point_b = commands.spawn_empty().id();
        let line = commands.spawn_empty().id();
        spawn_point_instructions.send(SpawnPointInstruction::from_lines(point_b, &[line]));
        connect_point_instructions.send(ConnectPointInstruction::new(point_a, line));
        line_instructions.send(SpawnLineInstruction::new(line, point_a, point_b));
        selection.point = Some(point_b);
    }
}
