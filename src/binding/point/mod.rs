mod normal;
mod track;

use bevy::prelude::*;

use crate::{
    binding::{
        point::{
            normal::{NormalPointBindingPlugin, NormalPointBindings},
            track::{TrackPointBindingPlugin, TrackPointBindings},
        },
        BindedCommands,
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
    pub fn bind(
        &self,
        selected_point: Entity,
        point_mode: PointMode,
        hover: &Hover,
        commands: &mut BindedCommands,
    ) {
        match point_mode {
            PointMode::Normal => {
                self.normal.bind(selected_point, hover, commands);
            }
            PointMode::Track(track_mode) => {
                self.track.bind(selected_point, track_mode, hover, commands);
            }
        }
    }
}
