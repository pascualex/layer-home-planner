#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod input;
mod palette;
mod point;
mod tool;
mod ui;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_prototype_lyon::prelude::*;
use point::Point;

use self::{
    input::InputPlugin,
    point::PointPlugin,
    tool::{Selected, ToolPlugin},
    ui::UiPlugin,
};

pub const ZOOM: f32 = 100.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputPlugin)
            .add_plugin(PointPlugin)
            .add_plugin(ToolPlugin)
            .add_plugin(UiPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut selected: ResMut<Selected>, mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(palette::DARK_BLACK),
        },
        transform: Transform::from_xyz(0.0, 0.0, 99.9),
        ..default()
    });
    // circle
    let shape = shapes::Circle {
        radius: 5.0,
        ..default()
    };
    let entity = commands
        .spawn((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(palette::LIGHT_WHITE)),
                Transform::default(),
            ),
            Point::new(Vec2::ZERO),
        ))
        .id();
    selected.entity = Some(entity);
}
