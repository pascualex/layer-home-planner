use bevy::prelude::*;

use crate::plan::{line::LINE_PRIORITY, DEFAULT_COLOR, HOVERED_COLOR, SELECTED_COLOR};

pub const POINT_RADIUS: f32 = 0.06;
pub const POINT_VERTICES: usize = 16;
const POINT_PRIORITY: f32 = LINE_PRIORITY + 1.0;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PointAssets>();
    }
}

#[derive(Resource)]
pub struct PointAssets {
    mesh: Handle<Mesh>,
    pub default_material: Handle<ColorMaterial>,
    pub hovered_material: Handle<ColorMaterial>,
    pub selected_material: Handle<ColorMaterial>,
}

impl FromWorld for PointAssets {
    fn from_world(world: &mut World) -> Self {
        world.resource_scope(|world, mut meshes: Mut<Assets<Mesh>>| {
            let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
            Self {
                mesh: meshes.add(
                    shape::Circle {
                        radius: POINT_RADIUS,
                        vertices: POINT_VERTICES,
                    }
                    .into(),
                ),
                default_material: materials.add(DEFAULT_COLOR.into()),
                hovered_material: materials.add(HOVERED_COLOR.into()),
                selected_material: materials.add(SELECTED_COLOR.into()),
            }
        })
    }
}

#[derive(Bundle)]
pub struct PointBundle {
    material_mesh: ColorMesh2dBundle,
    point: Point,
}

impl PointBundle {
    pub fn new(lines: Vec<Entity>, assets: &PointAssets) -> Self {
        Self {
            material_mesh: ColorMesh2dBundle {
                mesh: assets.mesh.clone().into(),
                material: assets.default_material.clone(),
                transform: Transform::from_translation(Vec2::ZERO.extend(POINT_PRIORITY)),
                ..default()
            },
            point: Point::new(lines),
        }
    }

    pub fn empty(assets: &PointAssets) -> Self {
        Self::new(vec![], assets)
    }

    pub fn from_line(line: Entity, assets: &PointAssets) -> Self {
        Self::new(vec![line], assets)
    }
}

#[derive(Component)]
pub struct Point {
    pub lines: Vec<Entity>,
}

impl Point {
    pub fn new(lines: Vec<Entity>) -> Self {
        Self { lines }
    }
}
