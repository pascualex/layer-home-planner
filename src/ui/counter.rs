use bevy::prelude::*;

use crate::{
    command::action::{RedoActions, UncommittedCommands, UndoActions},
    palette,
    plan::{line::Line, point::Point},
    ui::UiAssets,
    AppSet,
};

pub struct CounterUiPlugin;

impl Plugin for CounterUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_counters_panel).add_systems(
            (
                update_point_counter_text,
                update_line_counter_text,
                update_undo_counter_text,
                update_redo_counter_text,
                update_uncommitted_counter_text,
            )
                .in_set(AppSet::Ui),
        );
    }
}

#[derive(Component)]
struct PointCounterText;

#[derive(Component)]
struct LineCounterText;

#[derive(Component)]
struct UndoCounterText;

#[derive(Component)]
struct RedoCounterText;

#[derive(Component)]
struct UncommittedCounterText;

fn spawn_counters_panel(assets: Res<UiAssets>, mut commands: Commands) {
    let root = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect::new(Val::Auto, Val::Px(40.0), Val::Px(40.0), Val::Auto),
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
    let uncommitted_counter_text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized uncommitted counter",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        UncommittedCounterText,
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(point_counter_text);
        builder.spawn(line_counter_text);
        builder.spawn(undo_counter_text);
        builder.spawn(redo_counter_text);
        builder.spawn(uncommitted_counter_text);
    });
}

fn update_point_counter_text(
    point_query: Query<(), With<Point>>,
    mut text_query: Query<&mut Text, With<PointCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = point_query.iter().len();
    text.sections[0].value = format!("Points {count}");
}

fn update_line_counter_text(
    line_query: Query<(), With<Line>>,
    mut text_query: Query<&mut Text, With<LineCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = line_query.iter().len();
    text.sections[0].value = format!("Lines {count}");
}

fn update_undo_counter_text(
    undo_actions: Res<UndoActions>,
    mut text_query: Query<&mut Text, With<UndoCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = undo_actions.len();
    text.sections[0].value = format!("Undo {count}");
}

fn update_redo_counter_text(
    redo_actions: Res<RedoActions>,
    mut text_query: Query<&mut Text, With<RedoCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = redo_actions.len();
    text.sections[0].value = format!("Redo {count}");
}

fn update_uncommitted_counter_text(
    uncommitted_commands: Res<UncommittedCommands>,
    mut text_query: Query<&mut Text, With<UncommittedCounterText>>,
) {
    let mut text = text_query.single_mut();
    let count = uncommitted_commands.len();
    text.sections[0].value = format!("Uncommitted {count}");
}
