use bevy::prelude::*;

use crate::{
    command::{
        action::UncommittedCommands,
        line::{CreateLine, DeleteLine},
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    plan::{
        line::{Line, LineBlueprint},
        point::{Point, PointAssets, PointBlueprint, PointBundle},
    },
};

pub struct PointCommandPlugin;

impl Plugin for PointCommandPlugin {
    fn build(&self, app: &mut App) {
        app
            // atomic commands
            .register_system_command(create_point)
            .register_system_command(move_point)
            .register_system_command(despawn_point)
            // composed commands
            .register_system_command(transfer_point_lines)
            .register_system_command(delete_point_lines)
            .register_system_command(delete_point);
    }
}

// atomic commands

#[derive(Debug)]
pub struct CreatePoint(pub Entity, pub PointBlueprint);

fn create_point(
    In(CreatePoint(point, blueprint)): In<CreatePoint>,
    point_assets: Res<PointAssets>,
    mut commands: Commands,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
) {
    // apply
    commands
        .get_or_spawn(point)
        .insert(PointBundle::new(blueprint, &point_assets));
    // add undo
    uncommitted_commands.add(DespawnPoint(point));
}

#[derive(Debug)]
pub struct MovePoint(pub Entity, pub Vec2);

fn move_point(
    In(MovePoint(point_entity, new_position)): In<MovePoint>,
    mut point_query: Query<&mut Transform, With<Point>>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
) {
    // get state
    let transform = point_query.get(point_entity).unwrap();
    let old_position = transform.translation.truncate();
    // apply
    let mut transform = point_query.get_mut(point_entity).unwrap();
    transform.translation.x = new_position.x;
    transform.translation.y = new_position.y;
    // add undo
    uncommitted_commands.add(MovePoint(point_entity, old_position));
}

#[derive(Debug)]
struct DespawnPoint(pub Entity);

fn despawn_point(
    In(DespawnPoint(point_entity)): In<DespawnPoint>,
    point_query: Query<&Transform, With<Point>>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    mut commands: Commands,
) {
    // get state
    let transform = point_query.get(point_entity).unwrap();
    let old_position = transform.translation.truncate();
    // apply
    commands.entity(point_entity).despawn_recursive();
    // add undo
    uncommitted_commands.add(CreatePoint(point_entity, PointBlueprint::new(old_position)));
}

// composed commands

#[derive(Debug)]
pub struct TransferPointLines(pub Entity, pub Entity);

fn transfer_point_lines(
    In(TransferPointLines(old_point_entity, new_point_entity)): In<TransferPointLines>,
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

#[derive(Debug)]
pub struct DeletePointLines(pub Entity);

fn delete_point_lines(
    In(DeletePointLines(point_entity)): In<DeletePointLines>,
    point_query: Query<&Point>,
    mut commands: Commands,
) {
    let point = point_query.get(point_entity).unwrap();
    for &line_entity in &point.lines {
        commands.add_system_command(DeleteLine(line_entity));
    }
}

#[derive(Debug)]
pub struct DeletePoint(pub Entity);

fn delete_point(In(DeletePoint(point)): In<DeletePoint>, mut commands: Commands) {
    commands.add_system_command(DeletePointLines(point));
    commands.add_system_command(DespawnPoint(point));
}
