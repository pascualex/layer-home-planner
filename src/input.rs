use bevy::{prelude::*, render::camera::RenderTarget};

use crate::ZOOM;

#[derive(SystemLabel)]
pub struct InputUpdate;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>()
            .add_system(update_cursor_positon.label(InputUpdate))
            .add_system(update_cursor_primary.label(InputUpdate))
            .add_system(update_cursor_alt.label(InputUpdate));
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Option<Vec2>,
    pub primary: bool,
    pub alt: bool,
}

fn update_cursor_positon(windows: Res<Windows>, query: Query<&Camera>, mut cursor: ResMut<Cursor>) {
    let camera = query.single();
    let window = match camera.target {
        RenderTarget::Window(id) => windows.get(id).unwrap(),
        RenderTarget::Image(_) => panic!(),
    };
    cursor.position = window.cursor_position().map(|screen_position| {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        (screen_position - size / 2.0) / ZOOM
    });
}

fn update_cursor_primary(input: Res<Input<MouseButton>>, mut cursor: ResMut<Cursor>) {
    cursor.primary = input.just_pressed(MouseButton::Left);
}

fn update_cursor_alt(input: Res<Input<KeyCode>>, mut cursor: ResMut<Cursor>) {
    cursor.alt = input.pressed(KeyCode::LAlt) || input.pressed(KeyCode::RAlt);
}
