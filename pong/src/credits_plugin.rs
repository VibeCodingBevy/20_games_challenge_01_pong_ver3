use bevy::prelude::*;
use crate::components::{Config, GameState};

#[derive(Component)]
pub struct CreditsText;

pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Credits), show_credits)
            .add_systems(OnExit(GameState::Credits), hide_credits)
            .add_systems(Update, scroll_credits.run_if(in_state(GameState::Credits)));
    }
}

const SCROLL_SPEED: f32 = 200.0;

fn show_credits(mut commands: Commands, config: Res<Config>) {
    let start_top = config.screen.height as f32 + 50.0;

    commands.spawn((
        Text::new("LOL"),
        TextFont { font_size: config.game.font_size, ..default() },
        TextColor(Color::WHITE),
        TextLayout::new(Justify::Center, LineBreak::NoWrap),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(start_top),
            ..default()
        },
        CreditsText,
    ));
}

fn hide_credits(mut commands: Commands, query: Query<Entity, With<CreditsText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn scroll_credits(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Node, With<CreditsText>>,
) {
    for mut node in query.iter_mut() {
        if let Val::Px(top) = &mut node.top {
            *top -= SCROLL_SPEED * time.delta_secs();

            if *top < -50.0 {
                next_state.set(GameState::Menu);
            }
        }
    }
}
