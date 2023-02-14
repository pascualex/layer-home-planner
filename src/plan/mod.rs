pub mod line;
pub mod point;

use bevy::prelude::*;

use crate::palette;

use self::{line::LinePlugin, point::PointPlugin};

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

#[derive(Resource, Default, Debug)]
pub enum PlanMode {
    #[default]
    Default,
    Select(Entity),
    Track(Entity, CancelationMode),
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
pub enum CancelationMode {
    Move(Vec2),
    Destroy,
    DestroyAndSelect(Entity),
}
