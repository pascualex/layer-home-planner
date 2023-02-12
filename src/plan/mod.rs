pub mod line;
pub mod point;

use bevy::prelude::*;

use self::{line::LinePlugin, point::PointPlugin};

pub struct PlanPlugin;

impl Plugin for PlanPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LinePlugin)
            .add_plugin(PointPlugin)
            .init_resource::<PlanMode>();
    }
}

#[derive(Resource, Default, Debug)]
pub enum PlanMode {
    #[default]
    Default,
    Select(Entity),
    Track(Entity, TrackMode),
}

impl PlanMode {
    pub fn selection(&self) -> Option<Entity> {
        match *self {
            PlanMode::Select(selection) => Some(selection),
            PlanMode::Track(selection, _) => Some(selection),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TrackMode {
    Place,
    Move(Vec2),
}
