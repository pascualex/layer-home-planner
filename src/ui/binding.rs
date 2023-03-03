use bevy::prelude::*;

use crate::{
    binding::{bind, BindedCommands, Binding},
    palette,
    ui::UiAssets,
    AppSet,
};

pub struct BindingUiPlugin;

impl Plugin for BindingUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_bindings_panel)
            .add_system((bind.pipe(update_bindings_panel)).in_set(AppSet::Ui));
    }
}

#[derive(Component)]
struct CommandsText;

#[derive(Component)]
struct BindingsText;

fn spawn_bindings_panel(assets: Res<UiAssets>, mut commands: Commands) {
    let root = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect::new(Val::Auto, Val::Px(40.0), Val::Auto, Val::Px(40.0)),
            ..default()
        },
        ..default()
    };
    let names_text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized binding names",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            ..default()
        },
        CommandsText,
    );
    let keys_text = (
        TextBundle {
            text: Text::from_section(
                "Uninitialized binding keys",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: palette::LIGHT_WHITE,
                },
            ),
            style: Style {
                margin: UiRect::left(Val::Px(16.0)),
                ..default()
            },
            ..default()
        },
        BindingsText,
    );
    commands.spawn(root).with_children(|builder| {
        builder.spawn(names_text);
        builder.spawn(keys_text);
    });
}

fn update_bindings_panel(
    In(binding_hits): In<BindedCommands>,
    mut commands_text_query: Query<&mut Text, With<CommandsText>>,
    mut bindings_text_query: Query<&mut Text, (With<BindingsText>, Without<CommandsText>)>,
) {
    let mut commands_strings = Vec::new();
    let mut bindings_strings = Vec::new();
    for binding_hit in binding_hits.0 {
        commands_strings.push(binding_hit.name);
        let binding_string = match binding_hit.binding {
            Binding::Mouse(mouse_button) => format!("{mouse_button:?}"),
            Binding::Keyboard(key_code) => format!("{key_code:?}"),
        };
        bindings_strings.push(binding_string);
    }
    let mut commands_text = commands_text_query.single_mut();
    commands_text.sections[0].value = commands_strings.join("\n");
    let mut bindings_text = bindings_text_query.single_mut();
    bindings_text.sections[0].value = bindings_strings.join("\n");
}
