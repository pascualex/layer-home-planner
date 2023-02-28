mod normal;
mod track;

use bevy::prelude::*;

use crate::{
    binding::{
        point::{
            normal::{NormalPointBindingPlugin, NormalPointBindings},
            track::{TrackPointBindingPlugin, TrackPointBindings},
        },
        BindingHits,
    },
    input::Hover,
    plan::PointMode,
};

pub struct PointBindingPlugin;

impl Plugin for PointBindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NormalPointBindingPlugin)
            .add_plugin(TrackPointBindingPlugin);
    }
}

#[derive(Resource, Default)]
pub struct PointBindings {
    normal: NormalPointBindings,
    track: TrackPointBindings,
}

impl PointBindings {
    pub fn get_hits(
        &self,
        selected_point: Entity,
        point_mode: PointMode,
        hover: &Hover,
        hits: &mut BindingHits,
    ) {
        match point_mode {
            PointMode::Normal => self.normal.get_hits(selected_point, hover, hits),
            PointMode::Track(cancel_point) => {
                self.track
                    .get_hits(selected_point, cancel_point, hover, hits)
            }
        }
    }
}
