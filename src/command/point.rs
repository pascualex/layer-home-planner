use bevy::prelude::*;

use crate::{
    command::RegisterUndoableSystemCommand,
    plan::point::{Point, PointAssets, PointBlueprint, PointBundle},
};

pub struct PointCommandPlugin;

impl Plugin for PointCommandPlugin {
    fn build(&self, app: &mut App) {
        app.register_undoable_system_command(create_point)
            .register_undoable_system_command(move_point)
            .register_undoable_system_command(delete_point);
    }
}

pub struct CreatePoint(pub Entity, pub PointBlueprint);

fn create_point(
    In(CreatePoint(entity, blueprint)): In<CreatePoint>,
    point_assets: Res<PointAssets>,
    mut commands: Commands,
) -> DeletePoint {
    // apply
    commands
        .get_or_spawn(entity)
        .insert(PointBundle::new(blueprint, &point_assets));
    // build undo
    DeletePoint(entity)
}

pub struct MovePoint(pub Entity, pub Vec2);

fn move_point(
    In(MovePoint(entity, new_position)): In<MovePoint>,
    mut point_query: Query<&mut Transform, With<Point>>,
) -> MovePoint {
    // get state
    let transform = point_query.get(entity).unwrap();
    let old_position = transform.translation.truncate();
    // apply
    let mut transform = point_query.get_mut(entity).unwrap();
    transform.translation.x = new_position.x;
    transform.translation.y = new_position.y;
    // build undo
    MovePoint(entity, old_position)
}

pub struct DeletePoint(pub Entity);

fn delete_point(
    In(DeletePoint(entity)): In<DeletePoint>,
    point_query: Query<&Transform, With<Point>>,
    mut commands: Commands,
) -> CreatePoint {
    // get state
    let transform = point_query.get(entity).unwrap();
    let old_position = transform.translation.truncate();
    // apply
    commands.entity(entity).despawn_recursive();
    // build undo
    CreatePoint(entity, PointBlueprint::new(old_position))
}
