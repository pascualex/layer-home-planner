use bevy::prelude::*;

use crate::{
    input::{Cursor, InputUpdate},
    line::SpawnLineEvent,
    point::{
        HighlightLevel, HighlightPointEvent, Point, PointSpawn, SpawnPointWithEntityEvent,
        POINT_RADIUS,
    },
};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .init_resource::<Hovered>()
            .add_system(update_hovered.after(InputUpdate))
            .add_system(update_selected.after(update_hovered).before(PointSpawn))
            .add_system(move_selection.after(update_selected));
    }
}

#[derive(Resource, Default)]
pub struct Selected {
    pub entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct Hovered {
    pub entity: Option<Entity>,
}

fn update_selected(
    cursor: Res<Cursor>,
    hovered: Res<Hovered>,
    mut point_events: EventWriter<SpawnPointWithEntityEvent>,
    mut line_events: EventWriter<SpawnLineEvent>,
    mut selected: ResMut<Selected>,
    mut highlight_events: EventWriter<HighlightPointEvent>,
    mut commands: Commands,
) {
    let Some(position) = cursor.position else {
        return;
    };
    let new_entity = if cursor.primary {
        if selected.entity.is_none() {
            let entity = match hovered.entity {
                Some(entity) => entity,
                None => {
                    let entity = commands.spawn_empty().id();
                    point_events.send(SpawnPointWithEntityEvent::new(entity, position));
                    entity
                }
            };
            Some(entity)
        } else {
            None
        }
    } else if cursor.secondary && selected.entity.is_none() {
        let Some(position) = cursor.position else {
            return;
        };
        let Some(point_a_entity) = hovered.entity else {
            return;
        };
        let point_b_entity = commands.spawn_empty().id();
        point_events.send(SpawnPointWithEntityEvent::new(point_b_entity, position));
        line_events.send(SpawnLineEvent::new(point_a_entity, point_b_entity));
        Some(point_b_entity)
    } else {
        return;
    };
    if new_entity != selected.entity {
        if let Some(old_entity) = selected.entity {
            highlight_events.send(HighlightPointEvent::new(old_entity, HighlightLevel::Normal));
        }
        selected.entity = new_entity;
        if let Some(new_entity) = selected.entity {
            highlight_events.send(HighlightPointEvent::new(
                new_entity,
                HighlightLevel::Selected,
            ));
        }
    }
}

fn update_hovered(
    cursor: Res<Cursor>,
    query: Query<(Entity, &Transform), With<Point>>,
    selected: Res<Selected>,
    mut hovered: ResMut<Hovered>,
    mut highlight_events: EventWriter<HighlightPointEvent>,
) {
    let Some(cursor_position) = cursor.position else {
        return;
    };
    let radius_squared = POINT_RADIUS * POINT_RADIUS;
    let new_entity = query
        .iter()
        .find(|(entity, transform)| {
            let position = transform.translation.truncate();
            Vec2::distance_squared(position, cursor_position) <= radius_squared
                && match selected.entity {
                    Some(selected) => entity != &selected,
                    None => true,
                }
        })
        .map(|(entity, _)| entity);
    if new_entity != hovered.entity {
        if let Some(old_entity) = hovered.entity {
            highlight_events.send(HighlightPointEvent::new(old_entity, HighlightLevel::Normal));
        }
        hovered.entity = new_entity;
        if let Some(new_entity) = hovered.entity {
            highlight_events.send(HighlightPointEvent::new(
                new_entity,
                HighlightLevel::Hovered,
            ));
        }
    }
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
