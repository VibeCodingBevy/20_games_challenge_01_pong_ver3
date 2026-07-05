use bevy::prelude::*;
use crate::components::{Ball, GameState, LeftPaddle, LeftScoreText, PauseOverlay, RightPaddle, RightScoreText, Wall, Divider};
use crate::game_plugin::GameEntitiesSpawned;
use crate::menu_config::{MenuActionMessage, MenuDefinition, MenuItem};

pub struct PausePlugin;

#[derive(Resource)]
struct SavedMenuState {
    definition: MenuDefinition,
}

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), enter_paused)
            .add_systems(OnExit(GameState::Paused), exit_paused)
            .add_systems(Update, toggle_pause.run_if(in_state(GameState::InGame)))
            .add_systems(Update, pause_escape_resume.run_if(in_state(GameState::Paused)))
            .add_systems(Update, handle_pause_messages.run_if(in_state(GameState::Paused)));
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

fn enter_paused(
    mut commands: Commands,
    definition: Res<MenuDefinition>,
    saved_state: Option<Res<SavedMenuState>>,
) {
    if saved_state.is_none() {
        commands.insert_resource(SavedMenuState {
            definition: definition.clone(),
        });
    }

    commands.insert_resource(MenuDefinition {
        title: "PAUSED".into(),
        items: vec![
            MenuItem::Action {
                label: "Resume".into(),
                action: MenuActionMessage::Resume,
            },
            MenuItem::Action {
                label: "Back to Menu".into(),
                action: MenuActionMessage::BackToMenu,
            },
        ],
    });

    commands.spawn((
        PauseOverlay,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ZIndex(10),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
    ));
}

fn exit_paused(
    mut commands: Commands,
    saved_state: Res<SavedMenuState>,
    overlay: Query<Entity, With<PauseOverlay>>,
) {
    commands.insert_resource(saved_state.definition.clone());
    commands.remove_resource::<SavedMenuState>();

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

fn handle_pause_messages(
    mut events: MessageReader<MenuActionMessage>,
    mut commands: Commands,
    game_entities: Query<Entity, Or<(With<Ball>, With<LeftPaddle>, With<RightPaddle>, With<Wall>, With<Divider>, With<LeftScoreText>, With<RightScoreText>)>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        match event {
            MenuActionMessage::Resume => next_state.set(GameState::InGame),
            MenuActionMessage::BackToMenu => {
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
