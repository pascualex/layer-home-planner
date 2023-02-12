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
            SystemSet::new().with_system(handle_binded_action),
        );
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct BindedAction {
    pub action: Option<Action>,
}

pub enum Action {
    Create,
    Delete(Entity),
    Deselect,
    Extend(Entity),
    Move(Entity, Vec2),
    Select(Entity),
    Track(Entity),
    Merge(Entity, Entity),
}

fn handle_binded_action(
    binded_bundle: Res<BindedAction>,
    mut plan_mode: ResMut<PlanMode>,
    mut point_query: Query<(&mut Transform, &mut Point)>,
    mut line_query: Query<&mut Line>,
    mut commands: Commands,
) {
    if let Some(action) = &binded_bundle.action {
        *plan_mode = match *action {
            Action::Create => {
                let entity = commands.spawn(PointBundle::default()).id();
                PlanMode::Track(entity, TrackMode::Place)
            }
            Action::Delete(point_entity) => {
                let (_, point) = point_query.get(point_entity).unwrap();
                for &line_entity in &point.lines {
                    commands.entity(line_entity).despawn();
                }
                commands.entity(point_entity).despawn();
                PlanMode::Default
            }
            Action::Deselect => PlanMode::Default,
            Action::Extend(old_point_entity) => {
                let new_point_entity = commands.spawn_empty().id();
                let line_entity = commands
                    .spawn(LineBundle::new(old_point_entity, new_point_entity))
                    .id();
                commands
                    .entity(new_point_entity)
                    .insert(PointBundle::from_line_entity(line_entity));
                let (_, mut old_point) = point_query.get_mut(old_point_entity).unwrap();
                old_point.lines.push(line_entity);
                PlanMode::Track(new_point_entity, TrackMode::Place)
            }
            Action::Merge(old_point_entity, new_point_entity) => {
                let (_, old_point) = point_query.get(old_point_entity).unwrap();
                let lines_entities = old_point.lines.clone();
                let (_, mut new_point) = point_query.get_mut(new_point_entity).unwrap();
                for line_entity in lines_entities {
                    let mut line = line_query.get_mut(line_entity).unwrap();
                    line.replace(old_point_entity, new_point_entity);
                    new_point.lines.push(line_entity);
                }
                commands.entity(old_point_entity).despawn();
                PlanMode::Select(new_point_entity)
            }
            Action::Move(entity, position) => {
                let (mut transform, _) = point_query.get_mut(entity).unwrap();
                transform.translation.x = position.x;
                transform.translation.y = position.y;
                PlanMode::Select(entity)
            }
            Action::Select(entity) => PlanMode::Select(entity),
            Action::Track(entity) => {
                let (transform, _) = point_query.get(entity).unwrap();
                let position = transform.translation.truncate();
                PlanMode::Track(entity, TrackMode::Move(position))
            }
        }
    }
}
