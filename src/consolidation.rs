use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{
    input::{Cursor, Hover},
    plan::{
        line::{Line, LineShape, LINE_WIDTH},
        point::{
            Point, PointAssets, HOVERED_POINT_PRIORITY, SELECTED_POINT_PRIORITY,
            STANDARD_POINT_PRIORITY,
        },
        PlanMode, PointMode,
    },
    AppSet,
};

pub struct ConsolidationPlugin;

impl Plugin for ConsolidationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                highlight_points,
                track_cursor_with_selection,
                update_lines.after(track_cursor_with_selection),
            )
                .in_set(AppSet::Consolidation),
        );
    }
}

fn track_cursor_with_selection(
    plan_mode: Res<PlanMode>,
    cursor: Res<Cursor>,
    mut query: Query<&mut Transform, With<Point>>,
) {
    if let PlanMode::Point(selected_point_entity, PointMode::Track(_)) = *plan_mode {
        let mut transform = query.get_mut(selected_point_entity).unwrap();
        if let Some(position) = cursor.track_position() {
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}

fn highlight_points(
    plan_mode: Res<PlanMode>,
    hover: Res<Hover>,
    mut query: Query<(Entity, &mut Transform, &mut Handle<ColorMaterial>), With<Point>>,
    assets: Res<PointAssets>,
) {
    for (entity, mut transform, mut material) in &mut query {
        let (mode_material, mode_priority) = if Some(entity) == plan_mode.point() {
            (&assets.selected_material, SELECTED_POINT_PRIORITY)
        } else if Some(entity) == hover.point {
            (&assets.hovered_material, HOVERED_POINT_PRIORITY)
        } else {
            (&assets.standard_material, STANDARD_POINT_PRIORITY)
        };
        transform.translation.z = mode_priority;
        *material = mode_material.clone();
    }
}

fn update_lines(
    changed_point_query: Query<
        (Entity, &Transform, &Point),
        Or<(Changed<Transform>, Changed<Point>)>,
    >,
    other_point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<(&mut Transform, &mut Mesh2dHandle, &Line), Without<Point>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (point_a_entity, transform_a, point_a) in &changed_point_query {
        for &line_entity in &point_a.lines {
            let (mut line_transform, mut mesh, line) = line_query.get_mut(line_entity).unwrap();
            let point_b_entity = line.neighbour(point_a_entity).unwrap();
            let transform_b = other_point_query.get(point_b_entity).unwrap();
            let (position, local_a, local_b) = calculate_line(
                transform_a.translation.truncate(),
                transform_b.translation.truncate(),
            );
            line_transform.translation.x = position.x;
            line_transform.translation.y = position.y;
            *mesh = meshes
                .add(LineShape::new(local_a, local_b, LINE_WIDTH).into())
                .into();
        }
    }
}

fn calculate_line(position_a: Vec2, position_b: Vec2) -> (Vec2, Vec2, Vec2) {
    (
        (position_a + position_b) / 2.0,
        (position_a - position_b) / 2.0,
        (position_b - position_a) / 2.0,
    )
}
