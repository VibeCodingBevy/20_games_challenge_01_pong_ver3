use bevy::prelude::*;
use crate::components::GameState;

#[derive(Component)]
pub struct MenuText;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), show_menu)
            .add_systems(OnExit(GameState::Menu), hide_menu)
            .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)));
    }
}

fn show_menu(mut commands: Commands) {
    commands.spawn((
        Text::new("PONG\n\nPress Space to Start"),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ))
    .insert(MenuText);
}

fn hide_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuText>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn menu_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InGame);
    }
}