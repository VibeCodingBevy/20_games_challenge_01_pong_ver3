use bevy::prelude::*;
use pong_lib::credits_config::CreditsConfig;
use pong_lib::{Config, PongPlugin, Score};

fn main() {
    let config_str = include_str!("../config.toml");
    let configuration: Config = toml::from_str(config_str).unwrap();

    let credits_config_str = include_str!("../credits_config.toml");
    let credits_config: CreditsConfig = toml::from_str(credits_config_str).unwrap();

    let screen_width = configuration.screen.width;
    let screen_height = configuration.screen.height;

    App::new()
        .insert_resource(configuration)
        .insert_resource(credits_config)
        .insert_resource(Score { left: 0, right: 0 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".into(),
                resolution: (screen_width, screen_height).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PongPlugin)
        .run();
}