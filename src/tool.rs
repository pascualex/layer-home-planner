use bevy::prelude::*;

use crate::{
    input::{Cursor, InputUpdate},
    point::{Point, PointUpdate, SpawnPointEvent, POINT_RADIUS},
    ZOOM,
};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .add_system(move_selected_point.after(InputUpdate).before(PointUpdate))
            .add_system(update_selection.after(InputUpdate).before(PointUpdate));
    }
}

#[derive(Resource, Default)]
pub struct Selected {
    pub entity: Option<Entity>,
}

fn update_selection(
    cursor: Res<Cursor>,
    query: Query<(Entity, &Point)>,
    mut events: EventWriter<SpawnPointEvent>,
    mut selected: ResMut<Selected>,
) {
    if cursor.primary {
        if selected.entity.is_none() {
            let Some(position) = cursor.position else {
                return;
            };
            if let Some(entity) = find_point_at(position, &query) {
                selected.entity = Some(entity);
            } else {
                events.send(SpawnPointEvent::new(position));
            }
        } else {
            selected.entity = None;
        }
    }
}

fn find_point_at(position: Vec2, query: &Query<(Entity, &Point)>) -> Option<Entity> {
    let radius = POINT_RADIUS / ZOOM;
    let radius_squared = radius * radius;
    query
        .iter()
        .find(|(_, point)| Vec2::distance_squared(point.position, position) <= radius_squared)
        .map(|(entity, _)| entity)
}

fn move_selected_point(cursor: Res<Cursor>, selected: Res<Selected>, mut query: Query<&mut Point>) {
    let Some(entity) = selected.entity else {
        return;
    };
    let mut point = query.get_mut(entity).unwrap();
    if let Some(position) = cursor.position {
        let decimals = if cursor.alt { 2 } else { 1 };
        point.position.x = round(position.x, decimals);
        point.position.y = round(position.y, decimals);
    }
}

fn round(number: f32, decimals: u32) -> f32 {
    let offset = 10_i32.pow(decimals) as f32;
    (number * offset).round() / offset
}
