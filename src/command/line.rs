use bevy::prelude::*;

use crate::{
    command::{
        system_command::{AddSystemCommand, RegisterSystemCommand},
        RegisterUndoableSystemCommand,
    },
    plan::{
        line::{Line, LineAssets, LineBlueprint, LineBundle},
        point::Point,
    },
};

pub struct LineCommandPlugin;

impl Plugin for LineCommandPlugin {
    fn build(&self, app: &mut App) {
        app.register_undoable_system_command(create_line)
            .register_undoable_system_command(delete_line)
            .register_system_command(delete_lines)
            .register_system_command(transfer_lines);
    }
}

pub struct CreateLine(pub Entity, pub LineBlueprint);

fn create_line(
    In(CreateLine(entity, blueprint)): In<CreateLine>,
    line_assets: Res<LineAssets>,
    mut point_query: Query<&mut Point>,
    mut commands: Commands,
) -> DeleteLine {
    // apply
    commands
        .get_or_spawn(entity)
        .insert(LineBundle::new(blueprint, &line_assets));
    let mut point_a = point_query.get_mut(blueprint.point_a).unwrap();
    point_a.lines.push(entity);
    let mut point_b = point_query.get_mut(blueprint.point_b).unwrap();
    point_b.lines.push(entity);
    // build undo
    DeleteLine(entity)
}

pub struct DeleteLine(pub Entity);

fn delete_line(
    In(DeleteLine(entity)): In<DeleteLine>,
    line_query: Query<&Line>,
    mut point_query: Query<&mut Point>,
    mut commands: Commands,
) -> CreateLine {
    // get state
    let line = line_query.get(entity).unwrap();
    let old_point_a_entity = line.point_a;
    let old_point_b_entity = line.point_b;
    // apply
    let mut point_a = point_query.get_mut(line.point_a).unwrap();
    point_a.remove_line(entity);
    let mut point_b = point_query.get_mut(line.point_b).unwrap();
    point_b.remove_line(entity);
    commands.entity(entity).despawn_recursive();
    // push undo
    CreateLine(
        entity,
        LineBlueprint::new(old_point_a_entity, old_point_b_entity),
    )
}

pub struct DeleteLines(pub Entity);

fn delete_lines(
    In(DeleteLines(point_entity)): In<DeleteLines>,
    point_query: Query<&Point>,
    mut commands: Commands,
) {
    let point = point_query.get(point_entity).unwrap();
    for &line_entity in &point.lines {
        commands.add_system_command(DeleteLine(line_entity));
    }
}

pub struct TransferLines(pub Entity, pub Entity);

fn transfer_lines(
    In(TransferLines(old_point_entity, new_point_entity)): In<TransferLines>,
    point_query: Query<&Point>,
    line_query: Query<&Line>,
    mut commands: Commands,
) {
    let new_point = point_query.get(new_point_entity).unwrap();
    let new_point_neighbours_entities: Vec<_> = new_point
        .lines
        .iter()
        .map(|&new_point_line_entity| {
            let new_point_line = line_query.get(new_point_line_entity).unwrap();
            new_point_line.neighbour(new_point_entity).unwrap()
        })
        .collect();
    let old_point = point_query.get(old_point_entity).unwrap();
    for &old_point_line_entity in &old_point.lines {
        let old_point_line = line_query.get(old_point_line_entity).unwrap();
        let old_point_neighbour_entity = old_point_line.neighbour(old_point_entity).unwrap();
        if old_point_neighbour_entity != new_point_entity
            && !new_point_neighbours_entities.contains(&old_point_neighbour_entity)
        {
            let new_line = commands.spawn_empty().id();
            commands.add_system_command(CreateLine(
                new_line,
                LineBlueprint::new(old_point_neighbour_entity, new_point_entity),
            ));
        }
        commands.add_system_command(DeleteLine(old_point_line_entity));
    }
}
