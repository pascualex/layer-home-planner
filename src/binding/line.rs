use bevy::prelude::*;

use crate::{
    binding::{Binding, BindingHits},
    command::{
        line::DeleteLine,
        plan_mode::{SelectLine, SelectPoint, Unselect},
        system_command::{AddSystemCommand, RegisterSystemCommand},
    },
    input::Hover,
    plan::Element,
};

pub struct LineBindingPlugin;

impl Plugin for LineBindingPlugin {
    fn build(&self, app: &mut App) {
        app.register_system_command(custom_delete);
    }
}

pub struct LineBindings {
    select: Binding,
    delete: Binding,
    unselect: Binding,
}

impl LineBindings {
    pub fn get_hits(&self, selected_point: Entity, hover: &Hover, hits: &mut BindingHits) {
        match **hover {
            Element::Point(hovered_point) => {
                hits.no_commit("Select", self.select, SelectPoint(hovered_point));
            }
            Element::Line(hovered_line) => {
                hits.no_commit("Select", self.select, SelectLine(hovered_line));
            }
            Element::None => {
                hits.no_commit("Unselect", self.select, Unselect);
            }
        }
        hits.commit("Delete", self.delete, CustomDelete(selected_point));
        hits.no_commit("Unselect", self.unselect, Unselect);
    }
}

impl Default for LineBindings {
    fn default() -> Self {
        Self {
            select: Binding::Mouse(MouseButton::Left),
            delete: Binding::Keyboard(KeyCode::Delete),
            unselect: Binding::Keyboard(KeyCode::Escape),
        }
    }
}

#[derive(Debug)]
struct CustomDelete(Entity);

fn custom_delete(In(CustomDelete(line)): In<CustomDelete>, mut commands: Commands) {
    commands.add_system_command(Unselect);
    commands.add_system_command(DeleteLine(line));
}
