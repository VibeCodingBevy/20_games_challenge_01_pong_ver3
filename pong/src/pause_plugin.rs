use bevy::prelude::*;
use crate::components::{Ball, GameState, LeftPaddle, LeftScoreText, PauseOverlay, RightPaddle, RightScoreText, Wall, Divider};
use crate::game_plugin::GameEntitiesSpawned;

pub struct PausePlugin;

#[derive(Component)]
struct PauseMenuItem(usize);

const PAUSE_ITEMS: &[&str] = &["Resume", "Back to Menu"];

#[derive(Resource, Default)]
struct PauseMenuSelection(usize);

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PauseMenuSelection>()
            .add_systems(OnEnter(GameState::Paused), enter_paused)
            .add_systems(OnExit(GameState::Paused), exit_paused)
            .add_systems(Update, toggle_pause.run_if(in_state(GameState::InGame)))
            .add_systems(Update, pause_escape_resume.run_if(in_state(GameState::Paused)))
            .add_systems(Update, pause_navigate.run_if(in_state(GameState::Paused)))
            .add_systems(Update, pause_activate.run_if(in_state(GameState::Paused)))
            .add_systems(Update, pause_update_style.run_if(in_state(GameState::Paused)));
    }
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn enter_paused(mut commands: Commands) {
    commands.spawn((
        PauseOverlay,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ZIndex(-1),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ));

        parent.spawn((
            Text::new("PAUSED"),
            TextFont { font_size: 80.0, ..default() },
            TextColor(Color::WHITE),
            TextLayout::new(Justify::Center, LineBreak::NoWrap),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ));

        for (index, label) in PAUSE_ITEMS.iter().enumerate() {
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
                PauseMenuItem(index),
            )).with_children(|button_parent| {
                button_parent.spawn((
                    Text::new(label.to_string()),
                    TextFont { font_size: 36.0, ..default() },
                    TextColor(Color::WHITE),
                    TextLayout::new(Justify::Center, LineBreak::NoWrap),
                ));
            });
        }
    });
}

fn exit_paused(
    mut commands: Commands,
    overlay: Query<Entity, With<PauseOverlay>>,
) {
    for entity in overlay.iter() {
        commands.entity(entity).despawn();
    }
}

fn pause_escape_resume(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    }
}

fn pause_navigate(
    keys: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<PauseMenuSelection>,
) {
    let count = PAUSE_ITEMS.len();
    if keys.just_pressed(KeyCode::ArrowDown) {
        selection.0 = (selection.0 + 1) % count;
    } else if keys.just_pressed(KeyCode::ArrowUp) {
        selection.0 = if selection.0 == 0 { count - 1 } else { selection.0 - 1 };
    }
}

fn pause_activate(
    keys: Res<ButtonInput<KeyCode>>,
    selection: Res<PauseMenuSelection>,
    mut commands: Commands,
    game_entities: Query<Entity, Or<(With<Ball>, With<LeftPaddle>, With<RightPaddle>, With<Wall>, With<Divider>, With<LeftScoreText>, With<RightScoreText>)>>,
    mut next_state: ResMut<NextState<GameState>>,
    click_query: Query<(&Interaction, &PauseMenuItem), (With<Button>, Changed<Interaction>)>,
) {
    let mut activated: Option<usize> = None;

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        activated = Some(selection.0);
    }

    for (interaction, menu_item) in click_query.iter() {
        if *interaction == Interaction::Pressed {
            activated = Some(menu_item.0);
        }
    }

    if let Some(index) = activated {
        match index {
            0 => next_state.set(GameState::InGame),
            1 => {
                for entity in game_entities.iter() {
                    commands.entity(entity).despawn();
                }
                commands.remove_resource::<GameEntitiesSpawned>();
                next_state.set(GameState::Menu);
            }
            _ => {}
        }
    }
}

fn pause_update_style(
    selection: Res<PauseMenuSelection>,
    mut buttons: Query<(&Interaction, &PauseMenuItem, &Children)>,
    mut text_children: Query<(&mut Text, &mut TextColor)>,
) {
    for (interaction, menu_item, children) in buttons.iter_mut() {
        let is_selected = menu_item.0 == selection.0;

        let text_color = if is_selected && *interaction == Interaction::None {
            Color::srgb(1.0, 0.9, 0.5)
        } else {
            Color::WHITE
        };

        for child in children.iter() {
            if let Ok((_, mut text_color_comp)) = text_children.get_mut(child) {
                text_color_comp.0 = text_color;
            }
        }
    }
}
