use bevy::{prelude::*, render::camera::RenderTarget};

use crate::{
    action::ActionState,
    plan::point::{Point, Selection, POINT_RADIUS},
    AppStage,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .init_resource::<Hover>()
            .add_system_set_to_stage(
                AppStage::Input,
                SystemSet::new()
                    .with_system(update_cursor)
                    .with_system(update_hover)
                    .with_system(process_input),
            );
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Option<Vec2>,
}

#[derive(Resource, Default)]
pub struct Hover {
    pub point: Option<Entity>,
}

fn update_cursor(
    windows: Res<Windows>,
    query: Query<(&GlobalTransform, &Camera)>,
    mut cursor: ResMut<Cursor>,
) {
    let (transform, camera) = query.single();
    let window = match camera.target {
        RenderTarget::Window(id) => windows.get(id).unwrap(),
        RenderTarget::Image(_) => panic!(),
    };
    cursor.position = if let Some(screen_position) = window.cursor_position() {
        let size = Vec2::new(window.width(), window.height());
        let ndc = (screen_position / size) * 2.0 - Vec2::ONE;
        camera
            .ndc_to_world(transform, ndc.extend(-1.0))
            .map(|p| p.truncate())
    } else {
        None
    };
}

fn update_hover(
    cursor: Res<Cursor>,
    query: Query<(Entity, &Transform), With<Point>>,
    selection: Res<Selection>,
    mut hover: ResMut<Hover>,
) {
    let Some(cursor_position) = cursor.position else {
        return;
    };
    let radius_squared = POINT_RADIUS * POINT_RADIUS;
    hover.point = query
        .iter()
        .filter(|(entity, _)| Some(*entity) != selection.point)
        .find(|(_, transform)| {
            let position = transform.translation.truncate();
            Vec2::distance_squared(position, cursor_position) <= radius_squared
        })
        .map(|(entity, _)| entity);
}

fn process_input(
    selection: Res<Selection>,
    hover: Res<Hover>,
    input: Res<Input<MouseButton>>,
    mut action_state: ResMut<ActionState>,
) {
    *action_state = ActionState::None;
    #[allow(clippy::collapsible_if)]
    #[allow(clippy::collapsible_else_if)]
    if selection.point.is_none() {
        if hover.point.is_none() {
            if input.just_pressed(MouseButton::Left) {
                *action_state = ActionState::Create;
            }
        } else {
            if input.just_pressed(MouseButton::Left) {
                *action_state = ActionState::Select;
            } else if input.just_pressed(MouseButton::Right) {
                *action_state = ActionState::Extend;
            }
        }
    } else {
        if hover.point.is_none() {
            if input.just_pressed(MouseButton::Left) {
                *action_state = ActionState::Deselect;
            }
        }
    }
}
