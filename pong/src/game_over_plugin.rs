use bevy::prelude::*;
use crate::components::{Config, GameState, Score};

#[derive(Component)]
pub struct GameOverText;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), show_game_over)
            .add_systems(OnExit(GameState::GameOver), hide_game_over)
            .add_systems(Update, game_over_input.run_if(in_state(GameState::GameOver)));
    }
}

fn show_game_over(mut commands: Commands, score: Res<Score>, config: Res<Config>) {
    let winner = if score.left >= config.game.winning_score { "Player 1" } else { "Player 2" };
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GameOverText,
    )).with_children(|parent| {
        parent.spawn((
            Text::new(format!("Game Over\n\n{} Wins!\n\nPress Space to Restart\nPress Escape for Menu", winner)),
            TextFont { font_size: config.game.font_size, ..default() },
            TextColor(Color::WHITE),
            TextLayout::new(Justify::Center, LineBreak::NoWrap),
        ));
    });
}

fn hide_game_over(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn game_over_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
) {
    if keys.just_pressed(KeyCode::Space) {
        score.left = 0;
        score.right = 0;
        next_state.set(GameState::InGame);
    } else if keys.just_pressed(KeyCode::Escape) {
        score.left = 0;
        score.right = 0;
        next_state.set(GameState::Menu);
    }
}
