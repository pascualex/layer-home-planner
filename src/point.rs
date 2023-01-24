use bevy::{prelude::*, render::camera::RenderTarget};

use crate::{palette, tool::Selected};

pub const POINT_RADIUS: f32 = 8.0;

#[derive(SystemLabel)]
pub struct PointUpdate;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPointEvent>()
            .add_system(spawn_points.label(PointUpdate))
            .add_system(track_points_in_ui.label(PointUpdate));
    }
}

pub struct SpawnPointEvent {
    pub position: Vec2,
}

impl SpawnPointEvent {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

#[derive(Component)]
pub struct Point {
    pub ui_point: Entity,
}

impl Point {
    pub fn new(ui_point: Entity) -> Self {
        Self { ui_point }
    }
}

#[derive(Component)]
pub struct UiPoint {
    pub point: Entity,
}

impl UiPoint {
    pub fn new(point: Entity) -> Self {
        Self { point }
    }
}

fn spawn_points(
    mut events: EventReader<SpawnPointEvent>,
    mut selected: ResMut<Selected>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let ui_point_entity = commands.spawn_empty().id();
        let point_entity = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(
                    event.position.extend(0.0),
                )),
                Point::new(ui_point_entity),
            ))
            .id();
        selected.entity = Some(point_entity);
        commands.entity(ui_point_entity).insert((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::all(Val::Auto),
                    size: Size::new(Val::Px(POINT_RADIUS), Val::Px(POINT_RADIUS)),
                    ..default()
                },
                background_color: palette::LIGHT_WHITE.into(),
                ..default()
            },
            UiPoint::new(point_entity),
        ));
    }
}

fn track_points_in_ui(
    point_query: Query<(&Transform, &Point), Changed<Transform>>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    windows: Res<Windows>,
    mut ui_point_query: Query<&mut Style, With<UiPoint>>,
) {
    let (camera_transform, camera) = camera_query.single();
    let window = match camera.target {
        RenderTarget::Window(id) => windows.get(id).unwrap(),
        RenderTarget::Image(_) => panic!(),
    };
    for (point_transform, point) in &point_query {
        let ui_point_entity = point.ui_point;
        let Ok(mut style) = ui_point_query.get_mut(ui_point_entity) else {
            continue;
        };
        let Some(ndc) = camera.world_to_ndc(camera_transform, point_transform.translation) else {
            continue;
        };
        let screen_size = Vec2::new(window.width() as f32, window.height() as f32);
        let screen_position = ((ndc.truncate() + Vec2::ONE) / 2.0) * screen_size;
        style.position.left = Val::Px(screen_position.x - (POINT_RADIUS / 2.0));
        style.position.bottom = Val::Px(screen_position.y - (POINT_RADIUS / 2.0));
    }
}
