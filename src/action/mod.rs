mod point;

use bevy::prelude::*;

use self::point::PointActionPlugin;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PointActionPlugin)
            .init_resource::<ActionState>()
            .init_resource::<Selection>();
    }
}

#[derive(Resource, Default)]
pub enum ActionState {
    #[default]
    None,
    Select,
    Deselect,
    Create,
    Merge,
    Extend,
}

#[derive(Resource, Default)]
pub struct Selection {
    pub point: Option<Entity>,
}
