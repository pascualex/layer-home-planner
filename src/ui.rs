use bevy::prelude::*;

use crate::{
    command::undo::{RedoActions, UndoActions},
    palette,
    plan::{line::Line, point::Point, PlanMode},
    AppSet,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>()
            .add_startup_systems((spawn_coordinates_panel, spawn_counters_panel))
            .add_systems(
                (
                    update_coordinates_text,
                    update_point_counter_text,
                    update_line_counter_text,
                    update_undo_counter_text,
                    update_redo_counter_text,
                )
                    .in_set(AppSet::Ui),
            );
    }
}

#[derive(Resource)]
struct UiAssets {
    font: Handle<Font>,
}

#[derive(Component)]
struct CoordinatesText;

#[derive(Component)]
struct PointCounterText;

#[derive(Component)]
struct LineCounterText;

#[derive(Component)]
struct UndoCounterText;

#[derive(Component)]
struct RedoCounterText;

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server: &AssetServer = world.resource();
        Self {
            font: asset_server.load("fonts/roboto_bold.ttf"),
        }
    }
}

fn spawn_coordinates_panel(assets: Res<UiAssets>, mut commands: Commands) {
    let root = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect::new(Val::Auto, Val::Px(40.0), Val::Px(40.0), Val::Auto),
            ..default()
        },
        ..default()
    };
    let coordinates_text = (
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
        builder.spawn(coordinates_text);
    });
}

fn spawn_counters_panel(assets: Res<UiAssets>, mut commands: Commands) {
    let root = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect::new(Val::Auto, Val::Px(40.0), Val::Auto, Val::Px(40.0)),
            ..default()
        },
        ..default()
    };
    let point_counter_text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized point counter",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        PointCounterText,
    );
    let line_counter_text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized line counter",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        LineCounterText,
    );
    let undo_counter_text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized undo counter",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        UndoCounterText,
    );
    let redo_counter_text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized redo counter",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        RedoCounterText,
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(point_counter_text);
        builder.spawn(line_counter_text);
        builder.spawn(undo_counter_text);
        builder.spawn(redo_counter_text);
    });
}

fn update_coordinates_text(
    mode: Res<PlanMode>,
    point_query: Query<&Transform, With<Point>>,
    mut text_query: Query<&mut Text, With<CoordinatesText>>,
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

fn update_point_counter_text(
    point_query: Query<(), With<Point>>,
    mut text_query: Query<&mut Text, With<PointCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = point_query.iter().len();
    text.sections[0].value = format!("Points: {count}");
}

fn update_line_counter_text(
    line_query: Query<(), With<Line>>,
    mut text_query: Query<&mut Text, With<LineCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = line_query.iter().len();
    text.sections[0].value = format!("Lines: {count}");
}

fn update_undo_counter_text(
    undo_actions: Res<UndoActions>,
    mut text_query: Query<&mut Text, With<UndoCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = undo_actions.len();
    text.sections[0].value = format!("Undo: {count}");
}

fn update_redo_counter_text(
    redo_actions: Res<RedoActions>,
    mut text_query: Query<&mut Text, With<RedoCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = redo_actions.len();
    text.sections[0].value = format!("Redo: {count}");
}
