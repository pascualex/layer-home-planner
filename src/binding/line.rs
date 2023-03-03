use bevy::prelude::*;

use crate::{
    binding::{BindedCommands, Binding},
    command::{
        line::{DeleteLine, SplitLine},
        plan_mode::{SelectLine, SelectPoint, TrackPoint, Unselect},
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    input::Hover,
    plan::{Element, TrackMode},
};

pub struct LineBindingPlugin;

impl Plugin for LineBindingPlugin {
    fn build(&self, app: &mut App) {
        app.register_system_command(custom_split)
            .register_system_command(custom_delete);
    }
}

pub struct LineBindings {
    select: Binding,
    split: Binding,
    delete: Binding,
    unselect: Binding,
}

impl LineBindings {
    pub fn bind(&self, selected_line: Entity, hover: &Hover, commands: &mut BindedCommands) {
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
        commands.no_commit("Split", self.split, CustomSplit(selected_line));
        commands.commit("Delete", self.delete, CustomDelete(selected_line));
        commands.no_commit("Unselect", self.unselect, Unselect);
    }
}

impl Default for LineBindings {
    fn default() -> Self {
        Self {
            select: Binding::Mouse(MouseButton::Left),
            split: Binding::Keyboard(KeyCode::E),
            delete: Binding::Keyboard(KeyCode::Delete),
            unselect: Binding::Keyboard(KeyCode::Escape),
        }
    }
}

#[derive(Debug)]
struct CustomSplit(Entity);

fn custom_split(In(CustomSplit(old_line)): In<CustomSplit>, mut commands: Commands) {
    commands.add_system_command(Unselect);
    let new_point = commands.spawn_empty().id();
    commands.add_system_command(SplitLine(old_line, new_point));
    commands.add_system_command(TrackPoint(new_point, TrackMode::Split(old_line)));
}

#[derive(Debug)]
struct CustomDelete(Entity);

fn custom_delete(In(CustomDelete(line)): In<CustomDelete>, mut commands: Commands) {
    commands.add_system_command(Unselect);
    commands.add_system_command(DeleteLine(line));
}
