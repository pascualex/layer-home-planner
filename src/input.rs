use bevy::{prelude::*, render::camera::RenderTarget};

#[derive(SystemLabel)]
pub struct InputUpdate;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_system(update_cursor_positon.label(InputUpdate))
            .add_system(update_cursor_alt.label(InputUpdate))
            .add_system(update_deselect.label(InputUpdate));
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Option<Vec2>,
    pub alt: bool,
    pub deselect: bool,
}

fn update_cursor_positon(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform)>,
    mut cursor: ResMut<Cursor>,
) {
    let (camera, transform) = query.single();
    let window = match camera.target {
        RenderTarget::Window(id) => windows.get(id).unwrap(),
        RenderTarget::Image(_) => panic!(),
    };
    cursor.position = window.cursor_position().map(|screen_position| {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_position / size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
    });
}

fn update_cursor_alt(input: Res<Input<KeyCode>>, mut cursor: ResMut<Cursor>) {
    cursor.alt = input.pressed(KeyCode::LAlt) || input.pressed(KeyCode::RAlt);
}

fn update_deselect(input: Res<Input<MouseButton>>, mut cursor: ResMut<Cursor>) {
    cursor.deselect = input.just_pressed(MouseButton::Left);
}
