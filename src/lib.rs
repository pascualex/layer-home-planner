#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod palette;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::camera::{RenderTarget, ScalingMode},
};
use bevy_prototype_lyon::prelude::*;

const VIEWPORT_SIZE: f32 = 20.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_startup_system(setup)
            .add_system(update_cursor_positon)
            .add_system(follow.after(update_cursor_positon));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct CursorPosition(Option<Vec2>);

#[derive(Component)]
struct Follow;

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(palette::DARK_BLACK),
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(VIEWPORT_SIZE),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 99.9),
        ..default()
    });
    // circle
    let shape = shapes::RegularPolygon {
        feature: RegularPolygonFeature::Radius(0.125),
        sides: 20,
        ..default()
    };
    commands.spawn((
        GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(palette::LIGHT_WHITE)),
            Transform::default(),
        ),
        Follow,
    ));
}

fn update_cursor_positon(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, transform) = query.single();
    let window = match camera.target {
        RenderTarget::Window(id) => windows.get(id).unwrap(),
        RenderTarget::Image(_) => panic!(),
    };
    **cursor_position = window.cursor_position().map(|screen_position| {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_position / size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
    });
}

fn follow(mut query: Query<&mut Transform, With<Follow>>, cursor_position: Res<CursorPosition>) {
    for mut transform in &mut query {
        if let Some(cursor_position) = **cursor_position {
            transform.translation.x = cursor_position.x;
            transform.translation.y = cursor_position.y;
        }
    }
}
