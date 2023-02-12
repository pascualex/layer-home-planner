use bevy::prelude::*;

use crate::{
    plan::{
        line::{Line, LineBundle},
        point::{Point, PointBundle},
        PlanMode, TrackMode,
    },
    AppStage,
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BindedAction>().add_system_set_to_stage(
            AppStage::Action,
            SystemSet::new()
                .with_system(handle_create_action)
                .with_system(handle_delete_action)
                .with_system(handle_extend_action)
                .with_system(handle_merge_action)
                .with_system(handle_move_action)
                .with_system(handle_select_action)
                .with_system(handle_track_action)
                .with_system(handle_unselect_action),
        );
    }
}

#[derive(Resource, Default)]
pub enum BindedAction {
    #[default]
    None,
    Create,
    Delete(Entity),
    Extend(Entity),
    Merge(Entity, Entity),
    Move(Entity, Vec2),
    Select(Entity),
    Track(Entity),
    Unselect,
}

fn handle_create_action(
    binded_action: Res<BindedAction>,
    mut plan_mode: ResMut<PlanMode>,
    mut commands: Commands,
) {
    if let BindedAction::Create = *binded_action {
        let entity = commands.spawn(PointBundle::default()).id();
        *plan_mode = PlanMode::Track(entity, TrackMode::Place);
    }
}

fn handle_delete_action(
    binded_action: Res<BindedAction>,
    mut point_query: Query<&mut Point>,
    line_query: Query<&Line>,
    mut plan_mode: ResMut<PlanMode>,
    mut commands: Commands,
) {
    if let BindedAction::Delete(point_entity) = *binded_action {
        let point = point_query.get(point_entity).unwrap();
        let lines_entities = point.lines.clone();
        for line_entity in lines_entities {
            let line = line_query.get(line_entity).unwrap();
            let other_point_entity = line.other(point_entity).unwrap();
            let mut other_point = point_query.get_mut(other_point_entity).unwrap();
            let lines = &mut other_point.lines;
            lines.remove(lines.iter().position(|e| *e == line_entity).unwrap());
            commands.entity(line_entity).despawn();
        }
        commands.entity(point_entity).despawn();
        *plan_mode = PlanMode::Default;
    }
}

fn handle_extend_action(
    binded_action: Res<BindedAction>,
    mut query: Query<&mut Point>,
    mut plan_mode: ResMut<PlanMode>,
    mut commands: Commands,
) {
    if let BindedAction::Extend(old_point_entity) = *binded_action {
        let new_point_entity = commands.spawn(PointBundle::default()).id();
        let line_entity = commands
            .spawn(LineBundle::new(old_point_entity, new_point_entity))
            .id();
        commands
            .entity(new_point_entity)
            .insert(PointBundle::from_line_entity(line_entity));
        let mut old_point = query.get_mut(old_point_entity).unwrap();
        old_point.lines.push(line_entity);
        *plan_mode = PlanMode::Track(new_point_entity, TrackMode::Place);
    }
}

fn handle_merge_action(
    binded_action: Res<BindedAction>,
    mut point_query: Query<&mut Point>,
    mut line_query: Query<&mut Line>,
    mut plan_mode: ResMut<PlanMode>,
    mut commands: Commands,
) {
    if let BindedAction::Merge(old_point_entity, new_point_entity) = *binded_action {
        let old_point = point_query.get(old_point_entity).unwrap();
        let lines_entities = old_point.lines.clone();
        let mut new_point = point_query.get_mut(new_point_entity).unwrap();
        for line_entity in lines_entities {
            let mut line = line_query.get_mut(line_entity).unwrap();
            line.replace(old_point_entity, new_point_entity);
            new_point.lines.push(line_entity);
        }
        commands.entity(old_point_entity).despawn();
        *plan_mode = PlanMode::Select(new_point_entity);
    }
}

fn handle_move_action(
    binded_action: Res<BindedAction>,
    mut query: Query<&mut Transform, With<Point>>,
    mut plan_mode: ResMut<PlanMode>,
) {
    if let BindedAction::Move(entity, position) = *binded_action {
        let mut transform = query.get_mut(entity).unwrap();
        transform.translation.x = position.x;
        transform.translation.y = position.y;
        *plan_mode = PlanMode::Select(entity);
    }
}

fn handle_select_action(binded_action: Res<BindedAction>, mut plan_mode: ResMut<PlanMode>) {
    if let BindedAction::Select(entity) = *binded_action {
        *plan_mode = PlanMode::Select(entity);
    }
}

fn handle_track_action(
    binded_action: Res<BindedAction>,
    query: Query<&Transform, With<Point>>,
    mut plan_mode: ResMut<PlanMode>,
) {
    if let BindedAction::Track(entity) = *binded_action {
        let transform = query.get(entity).unwrap();
        let position = transform.translation.truncate();
        *plan_mode = PlanMode::Track(entity, TrackMode::Move(position))
    }
}

fn handle_unselect_action(binded_action: Res<BindedAction>, mut plan_mode: ResMut<PlanMode>) {
    if let BindedAction::Unselect = *binded_action {
        *plan_mode = PlanMode::Default;
    }
}
