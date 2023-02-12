use bevy::{prelude::*, render::camera::RenderTarget};

use crate::{
    plan::{
        point::{Point, POINT_RADIUS},
        PlanMode,
    },
    AppStage,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .init_resource::<Hover>()
            .add_system_set_to_stage(
                AppStage::Binding,
                SystemSet::new()
                    .with_system(update_cursor_position)
                    .with_system(update_cursor_mode)
                    .with_system(update_hover),
            );
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Option<Vec2>,
    pub mode: CursorMode,
}

impl Cursor {
    pub fn track_position(&self) -> Option<Vec2> {
        self.position.map(|position| {
            Vec2::new(
                Self::round(position.x, self.mode.decimals()),
                Self::round(position.y, self.mode.decimals()),
            )
        })
    }

    fn round(number: f32, decimals: u32) -> f32 {
        let offset = 10_i32.pow(decimals) as f32;
        (number * offset).round() / offset
    }
}

#[derive(Resource, Default)]
pub enum CursorMode {
    #[default]
    Decimeters,
    Centimeters,
}

impl CursorMode {
    fn decimals(&self) -> u32 {
        match self {
            CursorMode::Decimeters => 1,
            CursorMode::Centimeters => 2,
        }
    }
}

#[derive(Resource, Default)]
pub struct Hover {
    pub point: Option<Entity>,
}

fn update_cursor_position(
    windows: Res<Windows>,
    query: Query<(&GlobalTransform, &Camera)>,
    mut cursor: ResMut<Cursor>,
) {
    let (transform, camera) = query.single();
    let window = match camera.target {
        RenderTarget::Window(id) => windows.get(id).unwrap(),
        RenderTarget::Image(_) => panic!(),
    };
    cursor.position = window.cursor_position().and_then(|screen_position| {
        let size = Vec2::new(window.width(), window.height());
        let ndc = (screen_position / size) * 2.0 - Vec2::ONE;
        camera
            .ndc_to_world(transform, ndc.extend(-1.0))
            .map(|p| p.truncate())
    });
}

fn update_cursor_mode(input: Res<Input<KeyCode>>, mut cursor: ResMut<Cursor>) {
    cursor.mode = match input.pressed(KeyCode::LAlt) || input.pressed(KeyCode::RAlt) {
        true => CursorMode::Centimeters,
        false => CursorMode::Decimeters,
    };
}

fn update_hover(
    cursor: Res<Cursor>,
    query: Query<(Entity, &Transform), With<Point>>,
    mode: Res<PlanMode>,
    mut hover: ResMut<Hover>,
) {
    let Some(cursor_position) = cursor.position else {
        return;
    };
    let radius_squared = POINT_RADIUS * POINT_RADIUS;
    let tracked_entity = match *mode {
        PlanMode::Track(entity, _) => Some(entity),
        _ => None,
    };
    hover.point = query
        .iter()
        .filter(|(entity, _)| Some(*entity) != tracked_entity)
        .find(|(_, transform)| {
            let position = transform.translation.truncate();
            Vec2::distance_squared(position, cursor_position) <= radius_squared
        })
        .map(|(entity, _)| entity);
}
