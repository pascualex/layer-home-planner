use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    input::{Cursor, Hover},
    palette,
    plan::line::LINE_PRIORITY,
    AppStage,
};

pub const POINT_RADIUS: f32 = 0.1;
pub const POINT_PRIORITY: f32 = LINE_PRIORITY + 1.0;
pub const NORMAL_COLOR: Color = palette::LIGHT_WHITE;
pub const HOVERED_COLOR: Color = palette::LIGHT_YELLOW;
pub const SELECTED_COLOR: Color = palette::LIGHT_ORANGE;

#[derive(SystemLabel)]
pub struct PointReconciliation;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selection>()
            .add_event::<SpawnPointInstruction>()
            .add_event::<ConnectPointInstruction>()
            .add_system_set_to_stage(
                AppStage::Instruction,
                SystemSet::new()
                    .with_system(spawn_points)
                    .with_system(connect_points),
            )
            .add_system_set_to_stage(
                AppStage::Reconciliation,
                SystemSet::new()
                    .label(PointReconciliation)
                    .with_system(move_selection_to_cursor)
                    .with_system(highlight_points),
            );
    }
}

#[derive(Resource, Default)]
pub struct Selection {
    pub point: Option<Entity>,
}

pub struct SpawnPointInstruction {
    pub entity: Entity,
    pub lines: Vec<Entity>,
}

impl SpawnPointInstruction {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            lines: Vec::new(),
        }
    }

    pub fn from_lines(entity: Entity, lines: &[Entity]) -> Self {
        Self {
            entity,
            lines: lines.to_vec(),
        }
    }
}

pub struct ConnectPointInstruction {
    pub point: Entity,
    pub line: Entity,
}

impl ConnectPointInstruction {
    pub fn new(point: Entity, line: Entity) -> Self {
        Self { point, line }
    }
}

#[derive(Component)]
pub struct Point {
    pub lines: Vec<Entity>,
}

impl Point {
    pub fn new(lines: &[Entity]) -> Self {
        Self {
            lines: lines.to_vec(),
        }
    }
}

fn spawn_points(mut instructions: EventReader<SpawnPointInstruction>, mut commands: Commands) {
    for instruction in instructions.iter() {
        let shape = shapes::Rectangle {
            extents: Vec2::splat(POINT_RADIUS),
            ..default()
        };
        commands.entity(instruction.entity).insert((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(NORMAL_COLOR)),
                Transform {
                    translation: Vec2::ZERO.extend(POINT_PRIORITY),
                    rotation: Quat::from_rotation_z(PI / 4.0),
                    ..default()
                },
            ),
            Point::new(&instruction.lines),
        ));
    }
}

fn connect_points(
    mut instructions: EventReader<ConnectPointInstruction>,
    mut query: Query<&mut Point>,
) {
    for instruction in instructions.iter() {
        let Ok(mut point) = query.get_mut(instruction.point) else {
            return;
        };
        point.lines.push(instruction.line);
    }
}

fn move_selection_to_cursor(
    cursor: Res<Cursor>,
    input: Res<Input<KeyCode>>,
    selection: Res<Selection>,
    mut query: Query<&mut Transform, With<Point>>,
) {
    let Some(entity) = selection.point else {
        return;
    };
    let Ok(mut transform) = query.get_mut(entity) else {
        return;
    };
    if let Some(position) = cursor.position {
        let decimals = if input.pressed(KeyCode::LAlt) { 2 } else { 1 };
        transform.translation.x = round(position.x, decimals);
        transform.translation.y = round(position.y, decimals);
    }
}

fn round(number: f32, decimals: u32) -> f32 {
    let offset = 10_i32.pow(decimals) as f32;
    (number * offset).round() / offset
}

fn highlight_points(
    selection: Res<Selection>,
    hover: Res<Hover>,
    mut query: Query<(Entity, &mut DrawMode), With<Point>>,
) {
    for (entity, mut draw_mode) in &mut query {
        let color = if Some(entity) == selection.point {
            SELECTED_COLOR
        } else if Some(entity) == hover.point {
            HOVERED_COLOR
        } else {
            NORMAL_COLOR
        };
        *draw_mode = DrawMode::Fill(FillMode::color(color));
    }
}
