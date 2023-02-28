use bevy::prelude::*;

use crate::{
    binding::{Binding, BindingHits},
    command::{
        plan_mode::{SelectPoint, TrackPoint},
        point::CreatePoint,
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    input::Hover,
    plan::{point::PointBlueprint, TrackMode},
};

pub struct NormalBindingsPlugin;

impl Plugin for NormalBindingsPlugin {
    fn build(&self, app: &mut App) {
        app.register_system_command(custom_create);
    }
}

pub struct NormalBindings {
    select: Binding,
    create: Binding,
}

impl NormalBindings {
    pub fn get_hits(&self, hover: &Hover, hits: &mut BindingHits) {
        if let Some(hovered_point) = hover.point {
            hits.no_commit(self.select, SelectPoint(hovered_point));
        }
        hits.no_commit(self.create, CustomCreate);
    }
}

impl Default for NormalBindings {
    fn default() -> Self {
        Self {
            select: Binding::Mouse(MouseButton::Left),
            create: Binding::Keyboard(KeyCode::E),
        }
    }
}

#[derive(Debug)]
struct CustomCreate;

fn custom_create(In(CustomCreate): In<CustomCreate>, mut commands: Commands) {
    let new_point = commands.spawn_empty().id();
    commands.add_system_command(CreatePoint(new_point, PointBlueprint::default()));
    commands.add_system_command(TrackPoint(new_point, TrackMode::Create));
}
