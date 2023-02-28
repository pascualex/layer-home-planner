pub mod line;
pub mod point;

use bevy::prelude::*;

use crate::palette;

use crate::plan::{line::LinePlugin, point::PointPlugin};

const BASE_PRIORITY: f32 = 0.0;
const STANDARD_COLOR: Color = palette::LIGHT_WHITE;
const HOVERED_COLOR: Color = palette::LIGHT_GREEN;
const SELECTED_COLOR: Color = palette::LIGHT_BLUE;

pub struct PlanPlugin;

impl Plugin for PlanPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PointPlugin)
            .add_plugin(LinePlugin)
            .init_resource::<PlanMode>();
    }
}

#[derive(Resource, Default, Clone, Copy, Debug)]
pub enum PlanMode {
    #[default]
    Normal,
    Point(Entity, PointMode),
}

impl PlanMode {
    pub fn point(&self) -> Option<Entity> {
        match self {
            PlanMode::Point(point, _) => Some(*point),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PointMode {
    Normal,
    Track(TrackMode),
}

#[derive(Clone, Copy, Debug)]
pub enum TrackMode {
    Create,
    Move,
    Extend(Entity),
}
