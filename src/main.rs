use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use layer_home_planner::AppPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1280.0,
                        height: 720.0,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(AppPlugin)
        .run();
}
