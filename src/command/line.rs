use bevy::prelude::*;

use crate::{
    command::{
        action::UncommittedCommands,
        point::CreatePoint,
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    plan::{
        line::{Line, LineAssets, LineBlueprint, LineBundle},
        point::{Point, PointBlueprint},
    },
};

pub struct LineCommandPlugin;

impl Plugin for LineCommandPlugin {
    fn build(&self, app: &mut App) {
        app
            // atomic commands
            .register_system_command(create_line)
            .register_system_command(delete_line)
            // composed commands
            .register_system_command(split_line);
    }
}

// atomic commands

#[derive(Debug)]
pub struct CreateLine(pub Entity, pub LineBlueprint);

fn create_line(
    In(CreateLine(entity, blueprint)): In<CreateLine>,
    line_assets: Res<LineAssets>,
    mut point_query: Query<&mut Point>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    mut commands: Commands,
) {
    // apply
    commands
        .get_or_spawn(entity)
        .insert(LineBundle::new(blueprint, &line_assets));
    let mut point_a = point_query.get_mut(blueprint.point_a).unwrap();
    point_a.lines.push(entity);
    let mut point_b = point_query.get_mut(blueprint.point_b).unwrap();
    point_b.lines.push(entity);
    // add undo
    uncommitted_commands.add(DeleteLine(entity));
}

#[derive(Debug)]
pub struct DeleteLine(pub Entity);

fn delete_line(
    In(DeleteLine(line_entity)): In<DeleteLine>,
    line_query: Query<&Line>,
    mut point_query: Query<&mut Point>,
    mut uncommitted_commands: ResMut<UncommittedCommands>,
    mut commands: Commands,
) {
    // get state
    let line = line_query.get(line_entity).unwrap();
    let point_a_entity = line.point_a;
    let point_b_entity = line.point_b;
    // apply
    let mut point_a = point_query.get_mut(line.point_a).unwrap();
    point_a.remove_line(line_entity);
    let mut point_b = point_query.get_mut(line.point_b).unwrap();
    point_b.remove_line(line_entity);
    commands.entity(line_entity).despawn_recursive();
    // add undo
    uncommitted_commands.add(CreateLine(
        line_entity,
        LineBlueprint::new(point_a_entity, point_b_entity),
    ));
}

// composed commands

#[derive(Debug)]
pub struct SplitLine(pub Entity, pub Entity);

fn split_line(
    In(SplitLine(old_line_entity, new_point_entity)): In<SplitLine>,
    line_query: Query<&Line>,
    point_query: Query<&Transform, With<Point>>,
    mut commands: Commands,
) {
    let line = line_query.get(old_line_entity).unwrap();
    // get center position
    let point_a_transform = point_query.get(line.point_a).unwrap();
    let position_a = point_a_transform.translation.truncate();
    let point_b_transform = point_query.get(line.point_b).unwrap();
    let position_b = point_b_transform.translation.truncate();
    let center_position = (position_a + position_b) / 2.0;
    // delete old line
    commands.add_system_command(DeleteLine(old_line_entity));
    // create new point and lines
    commands.add_system_command(CreatePoint(
        new_point_entity,
        PointBlueprint::new(center_position),
    ));
    let new_line_a_entity = commands.spawn_empty().id();
    commands.add_system_command(CreateLine(
        new_line_a_entity,
        LineBlueprint::new(line.point_a, new_point_entity),
    ));
    let new_line_b_entity = commands.spawn_empty().id();
    commands.add_system_command(CreateLine(
        new_line_b_entity,
        LineBlueprint::new(line.point_b, new_point_entity),
    ));
}
