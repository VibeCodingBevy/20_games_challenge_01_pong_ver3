use bevy::prelude::*;

pub mod components;
pub mod credits_plugin;
pub mod menu_plugin;
pub mod game_over_plugin;
pub mod game_plugin;

pub use components::*;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<components::GameState>()
            .add_plugins(menu_plugin::MenuPlugin)
            .add_plugins(game_plugin::GamePlugin)
            .add_plugins(game_over_plugin::GameOverPlugin)
            .add_plugins(credits_plugin::CreditsPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}