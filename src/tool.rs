use bevy::prelude::*;

use crate::{
    input::{Cursor, InputUpdate},
    point::{Point, PointUpdate, SpawnPointEvent},
};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .add_system(move_selected_point.after(InputUpdate).before(PointUpdate))
            .add_system(update_selection.after(InputUpdate));
    }
}

#[derive(Resource, Default)]
pub struct Selected {
    pub entity: Option<Entity>,
}

fn update_selection(
    cursor: Res<Cursor>,
    mut events: EventWriter<SpawnPointEvent>,
    mut selected: ResMut<Selected>,
) {
    if cursor.primary {
        if selected.entity.is_none() {
            let Some(position) = cursor.position else {
                return;
            };
            events.send(SpawnPointEvent::new(position));
        } else {
            selected.entity = None;
        }
    }
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
