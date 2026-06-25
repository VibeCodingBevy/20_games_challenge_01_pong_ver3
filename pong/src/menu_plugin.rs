use bevy::prelude::*;
use crate::components::{ButtonIndex, GameState, MenuAction, MenuRoot, MenuSelection};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuSelection>()
            .add_systems(OnEnter(GameState::Menu), (reset_selection, spawn_menu))
            .add_systems(Update, navigate_menu.run_if(in_state(GameState::Menu)))
            .add_systems(Update, activate_selected.run_if(in_state(GameState::Menu)))
            .add_systems(Update, update_menu_button_style.run_if(in_state(GameState::Menu)))
            .add_systems(Update, handle_menu_interaction.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_menu);
    }
}

fn reset_selection(mut selection: ResMut<MenuSelection>) {
    selection.0 = 0;
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        MenuRoot,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("PONG"),
            TextFont { font_size: 80.0, ..default() },
            TextColor(Color::WHITE),
            TextLayout::new(Justify::Center, LineBreak::NoWrap),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ));

        let items: [(MenuAction, &str); 3] = [
            (MenuAction::StartGame, "Start Game"),
            (MenuAction::Credits, "Credits"),
            (MenuAction::Quit, "Quit"),
        ];

        for (index, (action, label)) in items.iter().enumerate() {
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                action.clone(),
                ButtonIndex(index),
            )).with_children(|button_parent| {
                button_parent.spawn((
                    Text::new(*label),
                    TextFont { font_size: 36.0, ..default() },
                    TextColor(Color::WHITE),
                    TextLayout::new(Justify::Center, LineBreak::NoWrap),
                ));
            });
        }
    });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn navigate_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<MenuSelection>,
    buttons: Query<(), (With<Button>, With<MenuAction>)>,
) {
    let count = buttons.iter().len();
    if count == 0 {
        return;
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        selection.0 = (selection.0 + 1) % count;
    } else if keys.just_pressed(KeyCode::ArrowUp) {
        selection.0 = if selection.0 == 0 { count - 1 } else { selection.0 - 1 };
    }
}

fn activate_selected(
    keys: Res<ButtonInput<KeyCode>>,
    selection: Res<MenuSelection>,
    query: Query<(&MenuAction, &ButtonIndex)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if !keys.just_pressed(KeyCode::Enter) && !keys.just_pressed(KeyCode::Space) {
        return;
    }

    for (action, index) in query.iter() {
        if index.0 == selection.0 {
            match action {
                MenuAction::StartGame => next_state.set(GameState::InGame),
                MenuAction::Credits => next_state.set(GameState::Credits),
                MenuAction::Quit => { exit.write(AppExit::Success); },
            }
            return;
        }
    }
}

fn handle_menu_interaction(
    query: Query<(&Interaction, &MenuAction), (With<Button>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                MenuAction::StartGame => next_state.set(GameState::InGame),
                MenuAction::Credits => next_state.set(GameState::Credits),
                MenuAction::Quit => { exit.write(AppExit::Success); },
            }
        }
    }
}

fn update_menu_button_style(
    selection: Res<MenuSelection>,
    mut buttons: Query<(&Interaction, &ButtonIndex, &mut BackgroundColor, &Children), (With<Button>, With<MenuAction>)>,
    mut texts: Query<&mut TextColor>,
) {
    for (interaction, index, mut bg_color, children) in buttons.iter_mut() {
        let is_selected = index.0 == selection.0;

        let new_bg = if *interaction == Interaction::Pressed {
            Color::srgba(1.0, 1.0, 1.0, 0.3)
        } else if *interaction == Interaction::Hovered {
            Color::srgba(1.0, 1.0, 1.0, 0.15)
        } else if is_selected {
            Color::srgba(1.0, 1.0, 1.0, 0.1)
        } else {
            Color::NONE
        };

        let new_text_color = if is_selected && *interaction == Interaction::None {
            Color::srgb(1.0, 0.9, 0.5)
        } else {
            Color::WHITE
        };

        bg_color.0 = new_bg;

        for child in children.iter() {
            if let Ok(mut text_color) = texts.get_mut(child) {
                text_color.0 = new_text_color;
            }
        }
    }
}
