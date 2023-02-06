pub mod line;
pub mod point;

use bevy::prelude::*;

use self::{line::LinePlugin, point::PointPlugin};

#[derive(SystemLabel)]
pub struct PlanUpdate;

pub struct PlanPlugin;

impl Plugin for PlanPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LinePlugin)
            .add_plugin(PointPlugin)
            .init_resource::<Selection>();
    }
}

#[derive(Resource, Default)]
pub struct Selection {
    pub point: Option<Entity>,
}
