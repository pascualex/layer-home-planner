use bevy::{prelude::*, render::camera::RenderTarget};

#[derive(SystemLabel)]
pub struct CursorUpdate;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_system(update_cursor_positon.label(CursorUpdate));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CursorPosition(Option<Vec2>);

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
