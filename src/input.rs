use bevy::prelude::*;

use crate::{
    plan::{
        line::{Line, LINE_WIDTH},
        point::{Point, POINT_RADIUS},
        Element, PlanMode, PointMode,
    },
    AppSet,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .init_resource::<Hover>()
            .add_systems(
                (update_cursor_position, update_cursor_mode, update_hover).in_set(AppSet::Input),
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

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Hover(Element);

fn update_cursor_position(
    window_query: Query<&Window>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut cursor: ResMut<Cursor>,
) {
    let window = window_query.single();
    let (transform, camera) = camera_query.single();
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
    point_query: Query<(Entity, &Transform), With<Point>>,
    line_query: Query<(Entity, &Line)>,
    mode: Res<PlanMode>,
    mut hover: ResMut<Hover>,
) {
    let Some(cursor_position) = cursor.position else {
        return;
    };
    let tracked_point_entity = match *mode {
        PlanMode::Point(point_entity, PointMode::Track(_)) => Some(point_entity),
        _ => None,
    };
    let hovered_point_entity = point_query
        .iter()
        // don't hover the tracked point
        .filter(|(point_entity, _)| Some(*point_entity) != tracked_point_entity)
        // calculate distances from cursor
        .map(|(point_entity, transform)| {
            let position = transform.translation.truncate();
            (point_entity, Vec2::distance(position, cursor_position))
        })
        // filter minimum distance from cursor
        .filter(|(_, distance)| *distance <= 2.0 * POINT_RADIUS)
        // get closest point
        .min_by(|(_, distance_a), (_, distance_b)| {
            distance_a
                .partial_cmp(distance_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(point_entity, _)| point_entity);
    if let Some(hovered_point_entity) = hovered_point_entity {
        **hover = Element::Point(hovered_point_entity);
        return;
    }
    let hovered_line_entity = line_query
        .iter()
        .map(|(line_entity, line)| {
            (
                line_entity,
                point_query
                    .get(line.point_a)
                    .unwrap()
                    .1
                    .translation
                    .truncate(),
                point_query
                    .get(line.point_b)
                    .unwrap()
                    .1
                    .translation
                    .truncate(),
            )
        })
        .map(|(line_entity, point_a, point_b)| {
            let squared_len = Vec2::distance_squared(point_a, point_b);
            if squared_len == 0.0 {
                return (line_entity, Vec2::distance(point_a, cursor_position));
            }
            let t = f32::clamp(
                Vec2::dot(cursor_position - point_a, point_b - point_a) / squared_len,
                0.0,
                1.0,
            );
            let projection = point_a + t * (point_b - point_a);
            (line_entity, Vec2::distance(projection, cursor_position))
        })
        .filter(|(_, distance)| *distance <= 2.0 * LINE_WIDTH)
        .min_by(|(_, distance_a), (_, distance_b)| {
            distance_a
                .partial_cmp(distance_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(line_entity, _)| line_entity);
    **hover = match hovered_line_entity {
        Some(hovered_line_entity) => Element::Line(hovered_line_entity),
        None => Element::None,
    }
}
