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
        Text::new(format!("Game Over\n\n{} Wins!\n\nPress Space to Restart\nPress Escape for Menu", winner)),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ))
    .insert(GameOverText);
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
