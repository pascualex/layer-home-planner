use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    command::CommandApplication,
    palette,
    plan::{line::LINE_PRIORITY, PlanUpdate},
};

pub const POINT_RADIUS: f32 = 0.1;
pub const POINT_PRIORITY: f32 = LINE_PRIORITY + 1.0;
pub const NORMAL_COLOR: Color = palette::LIGHT_WHITE;
pub const HOVERED_COLOR: Color = palette::LIGHT_YELLOW;
pub const SELECTED_COLOR: Color = palette::LIGHT_ORANGE;

#[derive(SystemLabel)]
pub struct PointSpawn;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPointWithEntityEvent>()
            .add_event::<ConnectPointEvent>()
            .add_system(spawn_points.label(PlanUpdate).after(CommandApplication))
            .add_system(connect_points.label(PlanUpdate).after(CommandApplication))
            .add_system(highlight_points.label(PlanUpdate).after(CommandApplication));
    }
}

pub struct SpawnPointWithEntityEvent {
    pub entity: Entity,
}

impl SpawnPointWithEntityEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

pub struct ConnectPointEvent {
    pub point: Entity,
    pub line: Entity,
}

impl ConnectPointEvent {
    pub fn new(point: Entity, line: Entity) -> Self {
        Self { point, line }
    }
}

#[derive(Component)]
pub struct Point {
    pub lines: Vec<Entity>,
}

impl Point {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

#[derive(Component)]
pub enum Highlight {
    Normal,
    #[allow(dead_code)]
    Hovered,
    #[allow(dead_code)]
    Selected,
}

fn spawn_points(mut events: EventReader<SpawnPointWithEntityEvent>, mut commands: Commands) {
    for event in events.iter() {
        let shape = shapes::Rectangle {
            extents: Vec2::splat(POINT_RADIUS),
            ..default()
        };
        commands.entity(event.entity).insert((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(NORMAL_COLOR)),
                Transform {
                    translation: Vec2::ZERO.extend(POINT_PRIORITY),
                    rotation: Quat::from_rotation_z(PI / 4.0),
                    ..default()
                },
            ),
            Point::new(),
            Highlight::Normal,
        ));
    }
}

pub fn connect_points(mut events: EventReader<ConnectPointEvent>, mut query: Query<&mut Point>) {
    for event in events.iter() {
        let Ok(mut point) = query.get_mut(event.point) else {
            return;
        };
        point.lines.push(event.line);
    }
}

fn highlight_points(mut query: Query<(&mut DrawMode, &Highlight), Changed<Highlight>>) {
    for (mut draw_mode, highlight) in &mut query {
        let color = match highlight {
            Highlight::Normal => NORMAL_COLOR,
            Highlight::Hovered => HOVERED_COLOR,
            Highlight::Selected => SELECTED_COLOR,
        };
        *draw_mode = DrawMode::Fill(FillMode::color(color));
    }
}
