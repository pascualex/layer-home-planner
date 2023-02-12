use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::{
    palette,
    plan::point::{Point, PointUpdate},
    AppStage, BASE_PRIORITY,
};

pub const LINE_WIDTH: f32 = 0.025;
pub const LINE_PRIORITY: f32 = BASE_PRIORITY + 1.0;

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            AppStage::Plan,
            SystemSet::new()
                .after(PointUpdate)
                .with_system(update_lines),
        );
    }
}

#[derive(Bundle)]
pub struct LineBundle {
    shape: ShapeBundle,
    line: Line,
}

impl LineBundle {
    pub fn new(point_a: Entity, point_b: Entity) -> Self {
        Self {
            shape: GeometryBuilder::build_as(
                &shapes::Line(Vec2::ZERO, Vec2::ZERO),
                DrawMode::Stroke(StrokeMode::new(palette::DARK_WHITE, LINE_WIDTH)),
                Transform::from_translation(Vec2::ZERO.extend(LINE_PRIORITY)),
            ),
            line: Line::new(point_a, point_b),
        }
    }
}

#[derive(Component)]
pub struct Line {
    pub point_a: Entity,
    pub point_b: Entity,
}

impl Line {
    pub fn new(point_a: Entity, point_b: Entity) -> Self {
        Self { point_a, point_b }
    }

    pub fn other(&self, point: Entity) -> Option<Entity> {
        if point == self.point_a {
            Some(self.point_b)
        } else if point == self.point_b {
            Some(self.point_a)
        } else {
            None
        }
    }

    pub fn replace(&mut self, old: Entity, new: Entity) {
        if old == self.point_a {
            self.point_a = new;
        } else if old == self.point_b {
            self.point_b = new;
        }
    }
}

fn update_lines(
    changed_point_query: Query<
        (Entity, &Transform, &Point),
        Or<(Changed<Transform>, Changed<Point>)>,
    >,
    other_point_query: Query<&Transform, With<Point>>,
    mut line_query: Query<(&mut Transform, &mut Path, &Line), Without<Point>>,
) {
    for (point_a_entity, transform_a, point_a) in &changed_point_query {
        for &line_entity in &point_a.lines {
            let Ok((mut line_transform, mut path, line)) = line_query.get_mut(line_entity) else {
                continue;
            };
            let Some(point_b_entity) = line.other(point_a_entity) else {
                continue;
            };
            let Ok(transform_b) = other_point_query.get(point_b_entity) else {
                continue;
            };
            let (position, local_a, local_b) = calculate_line(
                transform_a.translation.truncate(),
                transform_b.translation.truncate(),
            );
            line_transform.translation = position.extend(LINE_PRIORITY);
            *path = ShapePath::build_as(&shapes::Line(local_a, local_b));
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
