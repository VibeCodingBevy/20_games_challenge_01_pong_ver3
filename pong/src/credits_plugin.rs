use bevy::prelude::*;
use crate::components::{Config, GameState};

#[derive(Component)]
pub struct CreditsText;

#[derive(Resource)]
pub struct CreditsTimer(pub Timer);

pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Credits), show_credits)
            .add_systems(OnExit(GameState::Credits), hide_credits)
            .add_systems(Update, credits_timeout.run_if(in_state(GameState::Credits)));
    }
}

fn show_credits(mut commands: Commands, config: Res<Config>) {
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
        CreditsText,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("LOL"),
            TextFont { font_size: config.game.font_size, ..default() },
            TextColor(Color::WHITE),
            TextLayout::new(Justify::Center, LineBreak::NoWrap),
        ));
    });

    commands.insert_resource(CreditsTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn hide_credits(mut commands: Commands, query: Query<Entity, With<CreditsText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<CreditsTimer>();
}

fn credits_timeout(
    time: Res<Time>,
    mut timer: ResMut<CreditsTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        next_state.set(GameState::Menu);
    }
}
