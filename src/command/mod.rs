mod point;

pub use self::point::{CreationCommand, ExtensionCommand, SelectionCommand};

use bevy::prelude::*;

use self::point::PointCommandPlugin;

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PointCommandPlugin);
    }
}
