mod point;

use bevy::prelude::*;

use self::point::PointActionPlugin;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PointActionPlugin)
            .init_resource::<ActionState>();
    }
}

#[derive(Resource, Default)]
pub enum ActionState {
    #[default]
    None,
    Select,
    Deselect,
    Create,
    Extend,
}
