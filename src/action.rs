use std::collections::VecDeque;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::{
    plan::{
        line::{Line, LineAssets, LineBundle},
        point::{Point, PointAssets, PointBundle},
        PlanMode, TrackMode,
    },
    AppSet,
};

#[derive(ScheduleLabel, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ActionSchedule;

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
struct ActionSet;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionQueue>()
            .add_system(process_actions.in_set(AppSet::Action))
            .init_schedule(ActionSchedule)
            .add_systems_to_schedule(
                ActionSchedule,
                (
                    handle_create_action,
                    handle_delete_action,
                    handle_extend_action,
                    handle_merge_action,
                    handle_move_action,
                    handle_select_action,
                    handle_track_action,
                    handle_unselect_action,
                )
                    .in_set(ActionSet),
            )
            .add_system_to_schedule(ActionSchedule, apply_system_buffers.after(ActionSet));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ActionQueue(VecDeque<Action>);

#[derive(Resource, Deref, DerefMut)]
struct CurrentAction(Action);

pub enum Action {
    Create,
    Delete(Entity),
    Extend(Entity),
    Merge(Entity, Entity),
    Move(Entity, Vec2),
    Select(Entity),
    Track(Entity),
    Unselect,
}

fn process_actions(world: &mut World) {
    world.resource_scope(|world, mut action_queue: Mut<ActionQueue>| {
        while let Some(action) = action_queue.pop_front() {
            world.insert_resource(CurrentAction(action));
            world.run_schedule(ActionSchedule);
            world.remove_resource::<CurrentAction>();
        }
    });
}

fn handle_create_action(
    action: Res<CurrentAction>,
    mut mode: ResMut<PlanMode>,
    assets: Res<PointAssets>,
    mut commands: Commands,
) {
    if let Action::Create = **action {
        let entity = commands.spawn(PointBundle::empty(&assets)).id();
        *mode = PlanMode::Track(entity, TrackMode::Place);
    }
}

fn handle_delete_action(
    action: Res<CurrentAction>,
    mut point_query: Query<&mut Point>,
    line_query: Query<&Line>,
    mut mode: ResMut<PlanMode>,
    mut commands: Commands,
) {
    if let Action::Delete(point_entity) = **action {
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
        *mode = PlanMode::Default;
    }
}

fn handle_extend_action(
    action: Res<CurrentAction>,
    mut query: Query<&mut Point>,
    mut mode: ResMut<PlanMode>,
    point_assets: Res<PointAssets>,
    line_assets: Res<LineAssets>,
    mut commands: Commands,
) {
    if let Action::Extend(old_point_entity) = **action {
        let new_point_entity = commands.spawn_empty().id();
        let line_entity = commands
            .spawn(LineBundle::new(
                old_point_entity,
                new_point_entity,
                &line_assets,
            ))
            .id();
        commands
            .entity(new_point_entity)
            .insert(PointBundle::from_line(line_entity, &point_assets));
        let mut old_point = query.get_mut(old_point_entity).unwrap();
        old_point.lines.push(line_entity);
        *mode = PlanMode::Track(new_point_entity, TrackMode::Place);
    }
}

fn handle_merge_action(
    action: Res<CurrentAction>,
    mut point_query: Query<&mut Point>,
    mut line_query: Query<&mut Line>,
    mut mode: ResMut<PlanMode>,
    mut commands: Commands,
) {
    if let Action::Merge(old_point_entity, new_point_entity) = **action {
        let old_point = point_query.get(old_point_entity).unwrap();
        let lines_entities = old_point.lines.clone();
        let mut new_point = point_query.get_mut(new_point_entity).unwrap();
        for line_entity in lines_entities {
            let mut line = line_query.get_mut(line_entity).unwrap();
            line.replace(old_point_entity, new_point_entity);
            new_point.lines.push(line_entity);
        }
        commands.entity(old_point_entity).despawn();
        *mode = PlanMode::Select(new_point_entity);
    }
}

fn handle_move_action(
    action: Res<CurrentAction>,
    mut query: Query<&mut Transform, With<Point>>,
    mut mode: ResMut<PlanMode>,
) {
    if let Action::Move(entity, position) = **action {
        let mut transform = query.get_mut(entity).unwrap();
        transform.translation.x = position.x;
        transform.translation.y = position.y;
        *mode = PlanMode::Select(entity);
    }
}

fn handle_select_action(action: Res<CurrentAction>, mut mode: ResMut<PlanMode>) {
    if let Action::Select(entity) = **action {
        *mode = PlanMode::Select(entity);
    }
}

fn handle_track_action(
    action: Res<CurrentAction>,
    query: Query<&Transform, With<Point>>,
    mut mode: ResMut<PlanMode>,
) {
    if let Action::Track(entity) = **action {
        let transform = query.get(entity).unwrap();
        let position = transform.translation.truncate();
        *mode = PlanMode::Track(entity, TrackMode::Move(position))
    }
}

fn handle_unselect_action(action: Res<CurrentAction>, mut mode: ResMut<PlanMode>) {
    if let Action::Unselect = **action {
        *mode = PlanMode::Default;
    }
}
