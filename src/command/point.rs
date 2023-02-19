use bevy::prelude::*;

use crate::{
    command::RegisterUndoSystemCommand,
    plan::point::{Point, PointAssets, PointBlueprint, PointBundle},
};

pub struct PointCommandPlugin;

impl Plugin for PointCommandPlugin {
    fn build(&self, app: &mut App) {
        app.register_undo_system_command(create_point)
            .register_undo_system_command(delete_point);
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

pub struct DeletePoint(pub Entity);

fn delete_point(
    In(DeletePoint(entity)): In<DeletePoint>,
    point_query: Query<&Transform, With<Point>>,
    mut commands: Commands,
) -> CreatePoint {
    // get state
    let transform = point_query.get(entity).unwrap();
    let position = transform.translation.truncate();
    // apply
    commands.entity(entity).despawn_recursive();
    // build undo
    CreatePoint(entity, PointBlueprint::new(position))
}

// fn apply_move_point_action(
//     action: Res<CurrentAction>,
//     mut point_query: Query<&mut Transform, With<Point>>,
//     mut undo_action_stack: ResMut<UndoActionStack>,
// ) {
//     if let Action::Point(PointAction::Move(entity, position)) = **action {
//         // push undo
//         let transform = point_query.get(entity).unwrap();
//         undo_action_stack.push(Action::Point(PointAction::Move(
//             entity,
//             transform.translation.truncate(),
//         )));
//         // apply
//         let mut transform = point_query.get_mut(entity).unwrap();
//         transform.translation.x = position.x;
//         transform.translation.y = position.y;
//     }
// }
