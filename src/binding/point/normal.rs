use bevy::prelude::*;

use crate::{
    binding::{BindedCommands, Binding},
    command::{
        line::CreateLine,
        plan_mode::{SelectLine, SelectPoint, TrackPoint, Unselect},
        point::{CreatePoint, DeletePoint},
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    input::Hover,
    plan::{line::LineBlueprint, point::PointBlueprint, Element, TrackMode},
};

pub struct NormalPointBindingPlugin;

impl Plugin for NormalPointBindingPlugin {
    fn build(&self, app: &mut App) {
        app.register_system_command(custom_extend)
            .register_system_command(custom_delete);
    }
}

pub struct NormalPointBindings {
    select: Binding,
    track: Binding,
    extend: Binding,
    delete: Binding,
    unselect: Binding,
}

impl NormalPointBindings {
    pub fn bind(&self, selected_point: Entity, hover: &Hover, commands: &mut BindedCommands) {
        match **hover {
            Element::Point(hovered_point) => {
                commands.no_commit("Select", self.select, SelectPoint(hovered_point));
            }
            Element::Line(hovered_line) => {
                commands.no_commit("Select", self.select, SelectLine(hovered_line));
            }
            Element::None => {
                commands.no_commit("Unselect", self.select, Unselect);
            }
        }
        commands.no_commit(
            "Move",
            self.track,
            TrackPoint(selected_point, TrackMode::Move),
        );
        commands.no_commit("Extend", self.extend, CustomExtend(selected_point));
        commands.commit("Delete", self.delete, CustomDelete(selected_point));
        commands.no_commit("Unselect", self.unselect, Unselect);
    }
}

impl Default for NormalPointBindings {
    fn default() -> Self {
        Self {
            select: Binding::Mouse(MouseButton::Left),
            track: Binding::Keyboard(KeyCode::G),
            extend: Binding::Keyboard(KeyCode::E),
            delete: Binding::Keyboard(KeyCode::Delete),
            unselect: Binding::Keyboard(KeyCode::Escape),
        }
    }
}

#[derive(Debug)]
struct CustomExtend(Entity);

fn custom_extend(In(CustomExtend(old_point)): In<CustomExtend>, mut commands: Commands) {
    let new_point = commands.spawn_empty().id();
    commands.add_system_command(CreatePoint(new_point, PointBlueprint::default()));
    let new_line = commands.spawn_empty().id();
    commands.add_system_command(CreateLine(
        new_line,
        LineBlueprint::new(old_point, new_point),
    ));
    commands.add_system_command(TrackPoint(new_point, TrackMode::Extend(old_point)));
}

#[derive(Debug)]
struct CustomDelete(Entity);

fn custom_delete(In(CustomDelete(point)): In<CustomDelete>, mut commands: Commands) {
    commands.add_system_command(Unselect);
    commands.add_system_command(DeletePoint(point));
}
