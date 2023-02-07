use bevy::prelude::*;

use crate::{
    action::Selection,
    plan::{
        line::{Line, SpawnLineInstruction},
        point::Point,
        point::SpawnPointInstruction,
    },
    AppStage,
};

pub struct PointCommandPlugin;

impl Plugin for PointCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreationCommand>()
            .add_event::<MergeCommand>()
            .add_event::<ExtensionCommand>()
            .add_system_set_to_stage(
                AppStage::Command,
                SystemSet::new()
                    .with_system(apply_creation_command)
                    .with_system(apply_merge_command)
                    .with_system(apply_extension_command),
            );
    }
}

pub struct CreationCommand;

pub struct MergeCommand {
    old: Entity,
    new: Entity,
}

impl MergeCommand {
    pub fn new(old: Entity, new: Entity) -> Self {
        Self { old, new }
    }
}

pub struct ExtensionCommand {
    point: Entity,
}

impl ExtensionCommand {
    pub fn new(point: Entity) -> Self {
        Self { point }
    }
}

fn apply_creation_command(
    mut command_events: EventReader<CreationCommand>,
    mut instructions: EventWriter<SpawnPointInstruction>,
    mut selection: ResMut<Selection>,
    mut commands: Commands,
) {
    for _ in command_events.iter() {
        let entity = commands.spawn_empty().id();
        instructions.send(SpawnPointInstruction::new(entity));
        selection.point = Some(entity);
    }
}

fn apply_merge_command(
    mut command_events: EventReader<MergeCommand>,
    mut point_query: Query<&mut Point>,
    mut line_query: Query<&mut Line>,
    mut selection: ResMut<Selection>,
    mut commands: Commands,
) {
    for command in command_events.iter() {
        let Ok(old_point) = point_query.get(command.old) else {
            continue;
        };
        let lines = old_point.lines.clone();
        let Ok(mut new_point) = point_query.get_mut(command.new) else {
            continue;
        };
        for &line_entity in &lines {
            let Ok(mut line) = line_query.get_mut(line_entity) else {
                continue;
            };
            line.reconnect(command.old, command.new);
            new_point.lines.push(line_entity);
        }
        commands.entity(command.old).despawn_recursive();
        selection.point = None;
    }
}

fn apply_extension_command(
    mut command_events: EventReader<ExtensionCommand>,
    mut query: Query<&mut Point>,
    mut point_instructions: EventWriter<SpawnPointInstruction>,
    mut line_instructions: EventWriter<SpawnLineInstruction>,
    mut selection: ResMut<Selection>,
    mut commands: Commands,
) {
    for command in command_events.iter() {
        let old_point_entity = command.point;
        let new_point_entity = commands.spawn_empty().id();
        let line_entity = commands.spawn_empty().id();

        let Ok(mut old_point) = query.get_mut(old_point_entity) else {
            continue;
        };
        old_point.lines.push(line_entity);

        point_instructions.send(SpawnPointInstruction::from_lines(
            new_point_entity,
            &[line_entity],
        ));
        line_instructions.send(SpawnLineInstruction::new(
            line_entity,
            old_point_entity,
            new_point_entity,
        ));

        selection.point = Some(new_point_entity);
    }
}
