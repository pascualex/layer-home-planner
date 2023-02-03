use bevy::prelude::*;

use crate::{
    input::{Cursor, InputUpdate},
    line::SpawnLineEvent,
    point::{Point, PointSpawn, SpawnPointWithEntityEvent, POINT_RADIUS},
};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .add_system(update_selection.after(InputUpdate).before(PointSpawn))
            .add_system(move_selection.after(update_selection));
    }
}

#[derive(Resource, Default)]
pub struct Selected {
    pub entity: Option<Entity>,
}

fn update_selection(
    cursor: Res<Cursor>,
    query: Query<(Entity, &Transform), With<Point>>,
    mut point_events: EventWriter<SpawnPointWithEntityEvent>,
    mut line_events: EventWriter<SpawnLineEvent>,
    mut selected: ResMut<Selected>,
    mut commands: Commands,
) {
    if cursor.primary {
        if selected.entity.is_none() {
            let Some(position) = cursor.position else {
                return;
            };
            let entity = match find_point_at(position, &query) {
                Some(entity) => entity,
                None => {
                    let entity = commands.spawn_empty().id();
                    point_events.send(SpawnPointWithEntityEvent::new(entity, position));
                    entity
                }
            };
            selected.entity = Some(entity);
        } else {
            selected.entity = None;
        }
    } else if cursor.secondary && selected.entity.is_none() {
        let Some(position) = cursor.position else {
            return;
        };
        let Some(point_a_entity) = find_point_at(position, &query) else {
            return;
        };
        let point_b_entity = commands.spawn_empty().id();
        point_events.send(SpawnPointWithEntityEvent::new(point_b_entity, position));
        line_events.send(SpawnLineEvent::new(point_a_entity, point_b_entity));
        selected.entity = Some(point_b_entity);
    }
}

fn find_point_at(
    position: Vec2,
    query: &Query<(Entity, &Transform), With<Point>>,
) -> Option<Entity> {
    let radius_squared = POINT_RADIUS * POINT_RADIUS;
    query
        .iter()
        .find(|(_, transform)| {
            Vec2::distance_squared(transform.translation.truncate(), position) <= radius_squared
        })
        .map(|(entity, _)| entity)
}

fn move_selection(
    cursor: Res<Cursor>,
    selected: Res<Selected>,
    mut query: Query<&mut Transform, With<Point>>,
) {
    let Some(entity) = selected.entity else {
        return;
    };
    let Ok(mut transform) = query.get_mut(entity) else {
        return;
    };
    if let Some(position) = cursor.position {
        let decimals = if cursor.alt { 2 } else { 1 };
        transform.translation.x = round(position.x, decimals);
        transform.translation.y = round(position.y, decimals);
    }
}

fn round(number: f32, decimals: u32) -> f32 {
    let offset = 10_i32.pow(decimals) as f32;
    (number * offset).round() / offset
}
