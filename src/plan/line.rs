use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::plan::{BASE_PRIORITY, STANDARD_COLOR};

pub const LINE_WIDTH: f32 = 0.02;
pub const LINE_PRIORITY: f32 = BASE_PRIORITY + 1.0;

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LineAssets>();
    }
}

#[derive(Resource)]
pub struct LineAssets {
    material: Handle<ColorMaterial>,
}

impl FromWorld for LineAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        Self {
            material: materials.add(STANDARD_COLOR.into()),
        }
    }
}

pub struct LineShape {
    pub point_a: Vec2,
    pub point_b: Vec2,
    pub width: f32,
}

impl LineShape {
    pub fn new(point_a: Vec2, point_b: Vec2, width: f32) -> Self {
        Self {
            point_a,
            point_b,
            width,
        }
    }
}

impl From<LineShape> for Mesh {
    fn from(line: LineShape) -> Self {
        let extension = LINE_WIDTH / 2.0;
        let diff = line.point_a - line.point_b;
        let perp_norm = diff.perp().normalize_or_zero();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                (line.point_a - perp_norm * extension).extend(0.0),
                (line.point_a + perp_norm * extension).extend(0.0),
                (line.point_b - perp_norm * extension).extend(0.0),
                (line.point_b + perp_norm * extension).extend(0.0),
            ],
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::Z; 4]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![Vec2::ZERO; 4]);
        mesh.set_indices(Some(Indices::U16(vec![0, 1, 2, 1, 3, 2])));
        mesh
    }
}

#[derive(Bundle)]
pub struct LineBundle {
    material_mesh: ColorMesh2dBundle,
    line: Line,
}

impl LineBundle {
    pub fn new(point_a: Entity, point_b: Entity, assets: &LineAssets) -> Self {
        Self {
            material_mesh: ColorMesh2dBundle {
                material: assets.material.clone(),
                transform: Transform::from_translation(Vec2::ZERO.extend(LINE_PRIORITY)),
                ..default()
            },
            line: Line::new(point_a, point_b),
        }
    }
}

#[derive(Component)]
pub struct Line {
    pub point_a: Entity,
    pub point_b: Entity,
}

impl Line {
    pub fn new(point_a: Entity, point_b: Entity) -> Self {
        Self { point_a, point_b }
    }

    pub fn other(&self, point: Entity) -> Option<Entity> {
        if point == self.point_a {
            Some(self.point_b)
        } else if point == self.point_b {
            Some(self.point_a)
        } else {
            None
        }
    }
}
