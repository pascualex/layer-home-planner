use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::{
    input::{Cursor, Hover},
    palette,
    plan::{line::LINE_PRIORITY, PlanMode},
    AppStage,
};

pub const POINT_RADIUS: f32 = 0.1;
pub const POINT_PRIORITY: f32 = LINE_PRIORITY + 1.0;
pub const NORMAL_COLOR: Color = palette::LIGHT_WHITE;
pub const HOVERED_COLOR: Color = palette::LIGHT_YELLOW;
pub const SELECTED_COLOR: Color = palette::LIGHT_ORANGE;

#[derive(SystemLabel)]
pub struct PointUpdate;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            AppStage::Plan,
            SystemSet::new()
                .label(PointUpdate)
                .with_system(track_cursor_with_selection)
                .with_system(highlight_points),
        );
    }
}

#[derive(Bundle)]
pub struct PointBundle {
    shape: ShapeBundle,
    point: Point,
}

impl PointBundle {
    pub fn from_line_entity(line_entity: Entity) -> Self {
        Self {
            point: Point::new(vec![line_entity]),
            ..default()
        }
    }
}

impl Default for PointBundle {
    fn default() -> Self {
        Self {
            shape: GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: Vec2::splat(POINT_RADIUS),
                    ..default()
                },
                DrawMode::Fill(FillMode::color(NORMAL_COLOR)),
                Transform {
                    translation: Vec2::ZERO.extend(POINT_PRIORITY),
                    rotation: Quat::from_rotation_z(PI / 4.0),
                    ..default()
                },
            ),
            point: Point::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct Point {
    pub lines: Vec<Entity>,
}

impl Point {
    pub fn new(lines: Vec<Entity>) -> Self {
        Self { lines }
    }
}

fn track_cursor_with_selection(
    mode: Res<PlanMode>,
    cursor: Res<Cursor>,
    mut query: Query<&mut Transform, With<Point>>,
) {
    if let PlanMode::Track(entity, _) = *mode {
        let Ok(mut transform) = query.get_mut(entity) else {
            return;
        };
        if let Some(position) = cursor.track_position() {
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}

fn highlight_points(
    mode: Res<PlanMode>,
    hover: Res<Hover>,
    mut query: Query<(Entity, &mut DrawMode), With<Point>>,
) {
    for (entity, mut draw_mode) in &mut query {
        let color = if Some(entity) == mode.selection() {
            SELECTED_COLOR
        } else if Some(entity) == hover.point {
            HOVERED_COLOR
        } else {
            NORMAL_COLOR
        };
        *draw_mode = DrawMode::Fill(FillMode::color(color));
    }
}
