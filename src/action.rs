use std::collections::VecDeque;

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
        app.init_resource::<ActionQueue>()
            .add_system(process_actions.in_set(AppSet::Action))
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

#[derive(Resource, Default)]
pub struct ActionQueue {
    actions: VecDeque<Action>,
}

impl ActionQueue {
    // pub fn push_front(&mut self, actions: Vec<Action>) {
    //     for action in actions.into_iter().rev() {
    //         self.actions.push_front(action);
    //     }
    // }

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
    CreateLine(Entity, Entity),
    CreatePoint(Entity),
    DeletePoint(Entity),
    TransferLines(Entity, Entity),
    MovePoint(Entity, Vec2),
    SelectPoint(Entity),
    TrackPoint(Entity, TrackReason),
    UnselectPoint,
}

#[derive(Clone, Copy)]
pub enum TrackReason {
    Move,
    Create,
    Extend(Entity),
}

fn process_actions(world: &mut World) {
    world.resource_scope(|world, mut action_queue: Mut<ActionQueue>| {
        while let Some(action) = action_queue.pop() {
            world.insert_resource(CurrentAction(action));
            world.run_schedule(ActionSchedule);
            world.remove_resource::<CurrentAction>();
        }
    });
}

fn handle_connect_action(
    action: Res<CurrentAction>,
    mut query: Query<&mut Point>,
    assets: Res<LineAssets>,
    mut commands: Commands,
) {
    if let Action::CreateLine(point_a_entity, point_b_entity) = **action {
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
    if let Action::CreatePoint(entity) = **action {
        commands.entity(entity).insert(PointBundle::new(&assets));
    }
}

fn handle_delete_action(
    action: Res<CurrentAction>,
    mut point_query: Query<&mut Point>,
    line_query: Query<&Line>,
    mut commands: Commands,
) {
    if let Action::DeletePoint(point_entity) = **action {
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
    if let Action::TransferLines(old_point_entity, new_point_entity) = **action {
        let old_point = point_query.get(old_point_entity).unwrap();
        let old_lines_entities = old_point.lines.clone();
        let mut new_point = point_query.get_mut(new_point_entity).unwrap();
        let new_lines_entities = &mut new_point.lines;
        for line_entity in old_lines_entities {
            let mut line = line_query.get_mut(line_entity).unwrap();
            if line.other(old_point_entity).unwrap() == new_point_entity {
                new_lines_entities.remove(
                    new_lines_entities
                        .iter()
                        .position(|e| *e == line_entity)
                        .unwrap(),
                );
                commands.entity(line_entity).despawn();
            } else {
                line.replace(old_point_entity, new_point_entity);
                new_lines_entities.push(line_entity);
            }
        }
        commands.entity(old_point_entity).despawn();
    }
}

fn handle_move_action(action: Res<CurrentAction>, mut query: Query<&mut Transform, With<Point>>) {
    if let Action::MovePoint(entity, position) = **action {
        let mut transform = query.get_mut(entity).unwrap();
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

fn handle_select_action(action: Res<CurrentAction>, mut mode: ResMut<PlanMode>) {
    if let Action::SelectPoint(entity) = **action {
        *mode = PlanMode::Select(entity);
    }
}

fn handle_track_action(
    action: Res<CurrentAction>,
    query: Query<&Transform, With<Point>>,
    mut mode: ResMut<PlanMode>,
) {
    if let Action::TrackPoint(entity, cancelation) = **action {
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
    if let Action::UnselectPoint = **action {
        *mode = PlanMode::Default;
    }
}
