use std::collections::VecDeque;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::{
    plan::{
        line::{Line, LineAssets, LineBundle},
        point::{Point, PointAssets, PointBundle},
        PlanMode, TrackModeCancelation,
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
                    handle_disconnect_action,
                    handle_connect_action,
                    handle_move_action,
                    handle_select_action,
                    handle_track_action,
                    handle_transfer_action,
                    handle_unselect_action,
                )
                    .in_set(ActionSet),
            )
            .add_system_to_schedule(ActionSchedule, apply_system_buffers.after(ActionSet));
    }
}

#[derive(Resource, Default)]
pub struct ActionQueue {
    actions: VecDeque<Action>,
}

impl ActionQueue {
    pub fn push_front(&mut self, action: Action) {
        self.actions.push_front(action);
    }

    pub fn push_back(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.actions.push_back(action);
        }
    }

    fn pop(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }
}

#[derive(Resource, Deref, DerefMut)]
struct CurrentAction(Action);

#[derive(Clone, Copy)]
pub enum Action {
    Connect(Entity, Entity),
    Create(Entity),
    Delete(Entity),
    Disconnect(Entity),
    Transfer(Entity, Entity),
    Move(Entity, Vec2),
    Select(Entity),
    Track(Entity, TrackCancelation),
    Unselect,
}

#[derive(Clone, Copy)]
pub enum TrackCancelation {
    RestorePosition,
    Destroy,
    DestroyAndSelect(Entity),
}

fn process_actions(world: &mut World) {
    while let Some(action) = world.resource_mut::<ActionQueue>().pop() {
        world.insert_resource(CurrentAction(action));
        world.run_schedule(ActionSchedule);
        world.remove_resource::<CurrentAction>();
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
    query: Query<&Point>,
    mut action_queue: ResMut<ActionQueue>,
    mut commands: Commands,
) {
    if let Action::Delete(point_entity) = **action {
        let point = query.get(point_entity).unwrap();
        for &line_entity in &point.lines {
            action_queue.push_front(Action::Disconnect(line_entity));
        }
        commands.entity(point_entity).despawn();
    }
}

fn handle_disconnect_action(
    action: Res<CurrentAction>,
    line_query: Query<&Line>,
    mut point_query: Query<&mut Point>,
    mut commands: Commands,
) {
    if let Action::Disconnect(line_entity) = **action {
        let line = line_query.get(line_entity).unwrap();
        if let Ok(mut point_a) = point_query.get_mut(line.point_a) {
            point_a.remove_line(line_entity);
        }
        if let Ok(mut point_b) = point_query.get_mut(line.point_b) {
            point_b.remove_line(line_entity);
        }
        commands.entity(line_entity).despawn();
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
            TrackCancelation::RestorePosition => {
                let transform = query.get(entity).unwrap();
                let position = transform.translation.truncate();
                TrackModeCancelation::Move(position)
            }
            TrackCancelation::Destroy => TrackModeCancelation::Destroy,
            TrackCancelation::DestroyAndSelect(entity) => {
                TrackModeCancelation::DestroyAndSelect(entity)
            }
        };
        *mode = PlanMode::Track(entity, cancelation_mode)
    }
}

fn handle_transfer_action(
    action: Res<CurrentAction>,
    point_query: Query<&Point>,
    line_query: Query<&Line>,
    mut action_queue: ResMut<ActionQueue>,
) {
    if let Action::Transfer(old_point_entity, new_point_entity) = **action {
        let old_point = point_query.get(old_point_entity).unwrap();
        let new_point = point_query.get(new_point_entity).unwrap();
        let new_point_neighbours: Vec<_> = new_point
            .lines
            .iter()
            .map(|&line_entity| {
                let line = line_query.get(line_entity).unwrap();
                line.other(new_point_entity).unwrap()
            })
            .collect();
        for &line_entity in &old_point.lines {
            let line = line_query.get(line_entity).unwrap();
            let other_point_entity = line.other(old_point_entity).unwrap();
            action_queue.push_front(Action::Disconnect(line_entity));
            if other_point_entity != new_point_entity
                && !new_point_neighbours.contains(&other_point_entity)
            {
                action_queue.push_front(Action::Connect(other_point_entity, new_point_entity));
            }
        }
    }
}

fn handle_unselect_action(action: Res<CurrentAction>, mut mode: ResMut<PlanMode>) {
    if let Action::Unselect = **action {
        *mode = PlanMode::Default;
    }
}
