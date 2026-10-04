use bevy::prelude::*;
use bevy_flock_credits_plugin::CreditsConfig;
use pong_lib::menu_config::{MenuActionMessage, MenuDefinition, MenuItem};
use pong_lib::{Config, PongPlugin, Score};

fn main() {
    let config_str = include_str!("../config.toml");
    let configuration: Config = toml::from_str(config_str).unwrap();

    let credits_config_str = include_str!("../credits_config.toml");
    let credits_config: CreditsConfig = toml::from_str(credits_config_str).unwrap();

    let screen_width = configuration.screen.width;
    let screen_height = configuration.screen.height;

    let menu_definition = MenuDefinition {
        title: "PONG".into(),
        items: vec![
            MenuItem::Action {
                label: "Start Game".into(),
                action: MenuActionMessage::StartGame,
            },
            MenuItem::Submenu {
                label: "Settings".into(),
                items: vec![
                    MenuItem::Select {
                        label: "Winning Score".into(),
                        options: vec!["3".into(), "5".into(), "7".into(), "11".into()],
                        selected: 1,
                    },
                    MenuItem::Action {
                        label: "Back".into(),
                        action: MenuActionMessage::Back,
                    },
                ],
            },
            MenuItem::Action {
                label: "Credits".into(),
                action: MenuActionMessage::Credits,
            },
            MenuItem::Action {
                label: "Quit".into(),
                action: MenuActionMessage::Quit,
            },
        ],
    };

    App::new()
        .insert_resource(configuration)
        .insert_resource(credits_config)
        .insert_resource(Score { left: 0, right: 0 })
        .insert_resource(menu_definition)
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
