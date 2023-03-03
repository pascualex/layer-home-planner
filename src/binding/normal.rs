use bevy::prelude::*;

use crate::{
    binding::{BindedCommands, Binding},
    command::{
        plan_mode::{SelectLine, SelectPoint, TrackPoint},
        point::CreatePoint,
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    input::Hover,
    plan::{point::PointBlueprint, Element, TrackMode},
};

pub struct NormalBindingPlugin;

impl Plugin for NormalBindingPlugin {
    fn build(&self, app: &mut App) {
        app.register_system_command(custom_create);
    }
}

pub struct NormalBindings {
    select: Binding,
    create: Binding,
}

impl NormalBindings {
    pub fn bind(&self, hover: &Hover, commands: &mut BindedCommands) {
        match **hover {
            Element::Point(hovered_point) => {
                commands.no_commit("Select", self.select, SelectPoint(hovered_point));
            }
            Element::Line(hovered_line) => {
                commands.no_commit("Select", self.select, SelectLine(hovered_line));
            }
            Element::None => (),
        }
        commands.no_commit("Create", self.create, CustomCreate);
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
