use bevy::prelude::*;

use crate::ZOOM;

#[derive(SystemLabel)]
pub struct UpdateTransform;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_points_transform.label(UpdateTransform));
    }
}

#[derive(Component)]
pub struct Point {
    pub position: Vec2,
}

impl Point {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

fn update_points_transform(mut query: Query<(&mut Transform, &Point), Changed<Point>>) {
    for (mut transform, point) in &mut query {
        transform.translation.x = point.position.x * ZOOM;
        transform.translation.y = point.position.y * ZOOM;
    }
}
