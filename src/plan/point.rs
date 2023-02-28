use bevy::prelude::*;

use crate::plan::{line::LINE_PRIORITY, HOVERED_COLOR, SELECTED_COLOR, STANDARD_COLOR};

pub const POINT_RADIUS: f32 = 0.06;
pub const POINT_VERTICES: usize = 16;
pub const STANDARD_POINT_PRIORITY: f32 = LINE_PRIORITY + 1.0;
pub const HOVERED_POINT_PRIORITY: f32 = STANDARD_POINT_PRIORITY + 0.1;
pub const SELECTED_POINT_PRIORITY: f32 = HOVERED_POINT_PRIORITY + 0.1;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PointAssets>();
    }
}

#[derive(Resource)]
pub struct PointAssets {
    mesh: Handle<Mesh>,
    pub standard_material: Handle<ColorMaterial>,
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
                standard_material: materials.add(STANDARD_COLOR.into()),
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
    pub fn new(blueprint: PointBlueprint, assets: &PointAssets) -> Self {
        Self {
            material_mesh: ColorMesh2dBundle {
                mesh: assets.mesh.clone().into(),
                material: assets.standard_material.clone(),
                transform: Transform::from_translation(
                    blueprint.position.extend(STANDARD_POINT_PRIORITY),
                ),
                ..default()
            },
            point: Point::default(),
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct PointBlueprint {
    position: Vec2,
}

impl PointBlueprint {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

#[derive(Component, Default)]
pub struct Point {
    pub lines: Vec<Entity>,
}

impl Point {
    pub fn remove_line(&mut self, line: Entity) {
        let index = self.lines.iter().position(|&l| l == line).unwrap();
        self.lines.remove(index);
    }
}
