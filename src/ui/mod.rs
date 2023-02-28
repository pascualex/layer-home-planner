mod binding;
mod coordinates;
mod counter;

use bevy::prelude::*;

use crate::ui::{
    binding::BindingUiPlugin, coordinates::CoordinatesUiPlugin, counter::CounterUiPlugin,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BindingUiPlugin)
            .add_plugin(CounterUiPlugin)
            .add_plugin(CoordinatesUiPlugin)
            .init_resource::<UiAssets>();
    }
}

#[derive(Resource)]
struct UiAssets {
    font: Handle<Font>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server: &AssetServer = world.resource();
        Self {
            font: asset_server.load("fonts/roboto_bold.ttf"),
        }
    }
}
