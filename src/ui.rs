use bevy::prelude::*;

use crate::{
    palette,
    plan::{point::Point, PlanMode},
    AppSet,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>()
            .add_startup_system(spawn_inspector_panel)
            .add_system(update_inspector_text.in_set(AppSet::Ui));
    }
}

#[derive(Resource)]
struct UiAssets {
    font: Handle<Font>,
}

#[derive(Component)]
struct InspectorText;

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server: &AssetServer = world.resource();
        Self {
            font: asset_server.load("fonts/roboto_bold.ttf"),
        }
    }
}

fn spawn_inspector_panel(assets: Res<UiAssets>, mut commands: Commands) {
    let root = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect::new(Val::Auto, Val::Px(40.0), Val::Px(40.0), Val::Auto),
            ..default()
        },
        ..default()
    };
    let text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        InspectorText,
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(text);
    });
}

fn update_inspector_text(
    mode: Res<PlanMode>,
    point_query: Query<&Transform, With<Point>>,
    mut text_query: Query<&mut Text, With<InspectorText>>,
) {
    let mut text = text_query.single_mut();
    if let Some(entity) = mode.selection() {
        let transform = point_query.get(entity).unwrap();
        text.sections[0].value = format!(
            "({:.2}, {:.2})",
            if transform.translation.x == -0.0 {
                0.0
            } else {
                transform.translation.x
            },
            if transform.translation.y == -0.0 {
                0.0
            } else {
                transform.translation.y
            },
        );
    } else {
        text.sections[0].value = "Nothing selected".to_string();
    }
}
