use bevy::{prelude::*, render::camera::RenderTarget, utils::HashMap};

use crate::{palette, tool::Selected};

pub const POINT_RADIUS: f32 = 10.0;

#[derive(SystemLabel)]
pub struct PointUpdate;

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Points>()
            .add_event::<SpawnPointEvent>()
            .add_system(spawn_points.label(PointUpdate))
            .add_system(track_points_in_ui.label(PointUpdate));
    }
}

#[derive(Resource, Default)]
struct Points {
    ui_points: HashMap<Entity, Entity>,
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
pub struct Point;

#[derive(Component)]
struct UiPoint;

fn spawn_points(
    mut events: EventReader<SpawnPointEvent>,
    mut points: ResMut<Points>,
    mut selected: ResMut<Selected>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let point_entity = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(
                    event.position.extend(0.0),
                )),
                Point,
            ))
            .id();
        selected.entity = Some(point_entity);
        let ui_point_entity = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect::all(Val::Auto),
                        size: Size::new(Val::Px(POINT_RADIUS), Val::Px(POINT_RADIUS)),
                        ..default()
                    },
                    background_color: palette::LIGHT_WHITE.into(),
                    ..default()
                },
                UiPoint,
            ))
            .id();
        points.ui_points.insert(point_entity, ui_point_entity);
    }
}

fn track_points_in_ui(
    point_query: Query<(Entity, &Transform), (With<Point>, Changed<Transform>)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    windows: Res<Windows>,
    points: Res<Points>,
    mut ui_point_query: Query<&mut Style, With<UiPoint>>,
) {
    let (camera_transform, camera) = camera_query.single();
    let window = match camera.target {
        RenderTarget::Window(id) => windows.get(id).unwrap(),
        RenderTarget::Image(_) => panic!(),
    };
    for (point_entity, point_transform) in &point_query {
        let Some(&ui_point_entity) = points.ui_points.get(&point_entity) else {
            continue;
        };
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
