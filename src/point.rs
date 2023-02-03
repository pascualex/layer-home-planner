use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{line::LINE_PRIORITY, palette};

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
            .add_event::<HighlightPointEvent>()
            .add_system(spawn_points.label(PointSpawn))
            .add_system(connect_points)
            .add_system(highlight_points);
    }
}

pub struct SpawnPointWithEntityEvent {
    pub entity: Entity,
    pub position: Vec2,
}

impl SpawnPointWithEntityEvent {
    pub fn new(entity: Entity, position: Vec2) -> Self {
        Self { entity, position }
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

pub struct HighlightPointEvent {
    pub point: Entity,
    pub level: HighlightLevel,
}

impl HighlightPointEvent {
    pub fn new(point: Entity, level: HighlightLevel) -> Self {
        Self { point, level }
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

pub enum HighlightLevel {
    Normal,
    Hovered,
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
                    translation: event.position.extend(POINT_PRIORITY),
                    rotation: Quat::from_rotation_z(PI / 4.0),
                    ..default()
                },
            ),
            Point::new(),
        ));
    }
}

fn connect_points(mut events: EventReader<ConnectPointEvent>, mut query: Query<&mut Point>) {
    for event in events.iter() {
        let Ok(mut point) = query.get_mut(event.point) else {
            return;
        };
        point.lines.push(event.line);
    }
}

fn highlight_points(
    mut events: EventReader<HighlightPointEvent>,
    mut query: Query<&mut DrawMode, With<Point>>,
) {
    for event in events.iter() {
        let Ok(mut draw_mode) = query.get_mut(event.point) else {
            continue;
        };
        let color = match event.level {
            HighlightLevel::Normal => NORMAL_COLOR,
            HighlightLevel::Hovered => HOVERED_COLOR,
            HighlightLevel::Selected => SELECTED_COLOR,
        };
        *draw_mode = DrawMode::Fill(FillMode::color(color));
    }
}
