use bevy::prelude::*;

use crate::{
    palette,
    plan::{point::Point, PlanMode},
    ui::UiAssets,
    AppSet,
};

pub struct CoordinatesUiPlugin;

impl Plugin for CoordinatesUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_coordinates_panel)
            .add_system(update_coordinates_text.in_set(AppSet::Ui));
    }
}

#[derive(Component)]
struct CoordinatesText;

fn spawn_coordinates_panel(assets: Res<UiAssets>, mut commands: Commands) {
    let root = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect::new(Val::Px(40.0), Val::Auto, Val::Px(40.0), Val::Auto),
            ..default()
        },
        ..default()
    };
    let text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized coordinates",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        CoordinatesText,
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(text);
    });
}

fn update_coordinates_text(
    plan_mode: Res<PlanMode>,
    point_query: Query<&Transform, With<Point>>,
    mut text_query: Query<&mut Text, With<CoordinatesText>>,
) {
    let mut text = text_query.single_mut();
    if let Some(selected_point_entity) = plan_mode.point() {
        let transform = point_query.get(selected_point_entity).unwrap();
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
