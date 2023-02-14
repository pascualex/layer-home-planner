use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::{
    plan::{
        line::{Line, LineAssets, LineBundle},
        point::{Point, PointAssets, PointBundle},
        CancelationMode, PlanMode,
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
        app.add_system(process_actions.in_set(AppSet::Action))
            .init_schedule(ActionSchedule)
            .add_systems_to_schedule(
                ActionSchedule,
                (
                    handle_create_action,
                    handle_delete_action,
                    handle_connect_action,
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

#[derive(Resource)]
pub struct ActionBundle {
    actions: Vec<Action>,
}

impl ActionBundle {
    pub fn new(actions: Vec<Action>) -> Self {
        Self { actions }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct CurrentAction(Action);

#[derive(Clone, Copy)]
pub enum Action {
    Connect(Entity, Entity),
    Create(Entity),
    Delete(Entity),
    Merge(Entity, Entity),
    Move(Entity, Vec2),
    Select(Entity),
    Track(Entity, TrackReason),
    Unselect,
}

#[derive(Clone, Copy)]
pub enum TrackReason {
    Move,
    Create,
    Extend(Entity),
}

fn process_actions(world: &mut World) {
    if let Some(action_bundle) = world.remove_resource::<ActionBundle>() {
        for &action in &action_bundle.actions {
            world.insert_resource(CurrentAction(action));
            world.run_schedule(ActionSchedule);
            world.remove_resource::<CurrentAction>();
        }
    }
}

fn handle_connect_action(
    action: Res<CurrentAction>,
    mut query: Query<&mut Point>,
    assets: Res<LineAssets>,
    mut commands: Commands,
) {
    if let Action::Connect(point_a_entity, point_b_entity) = **action {
        let line_entity = commands
            .spawn(LineBundle::new(point_a_entity, point_b_entity, &assets))
            .id();
        let mut point_a = query.get_mut(point_a_entity).unwrap();
        point_a.lines.push(line_entity);
        let mut point_b = query.get_mut(point_b_entity).unwrap();
        point_b.lines.push(line_entity);
    }
}

fn handle_create_action(
    action: Res<CurrentAction>,
    assets: Res<PointAssets>,
    mut commands: Commands,
) {
    if let Action::Create(entity) = **action {
        commands.entity(entity).insert(PointBundle::new(&assets));
    }
}

fn handle_delete_action(
    action: Res<CurrentAction>,
    mut point_query: Query<&mut Point>,
    line_query: Query<&Line>,
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
    }
}

fn handle_merge_action(
    action: Res<CurrentAction>,
    mut point_query: Query<&mut Point>,
    mut line_query: Query<&mut Line>,
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
    }
}

fn handle_move_action(action: Res<CurrentAction>, mut query: Query<&mut Transform, With<Point>>) {
    if let Action::Move(entity, position) = **action {
        let mut transform = query.get_mut(entity).unwrap();
        transform.translation.x = position.x;
        transform.translation.y = position.y;
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
    if let Action::Track(entity, cancelation) = **action {
        let cancelation_mode = match cancelation {
            TrackReason::Move => {
                let transform = query.get(entity).unwrap();
                let position = transform.translation.truncate();
                CancelationMode::Move(position)
            }
            TrackReason::Create => CancelationMode::Destroy,
            TrackReason::Extend(entity) => CancelationMode::DestroyAndSelect(entity),
        };
        *mode = PlanMode::Track(entity, cancelation_mode)
    }
}

fn handle_unselect_action(action: Res<CurrentAction>, mut mode: ResMut<PlanMode>) {
    if let Action::Unselect = **action {
        *mode = PlanMode::Default;
    }
}
