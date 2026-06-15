use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize)]
pub struct Config {
    pub screen: Screen,
    pub ball: BallConfig,
    pub paddle: Paddle,
    pub arena: Arena,
    pub game: GameConfig,
}

#[derive(Deserialize)]
pub struct Screen { pub width: u32, pub height: u32 }
#[derive(Deserialize)]
pub struct BallConfig { pub diameter: f32, pub speed: f32 }
#[derive(Deserialize)]
pub struct Paddle { pub width: f32, pub height: f32, pub margin: f32, pub speed: f32 }
#[derive(Deserialize)] pub struct Arena { pub wall_thickness: f32, pub divider_width: f32 }
#[derive(Deserialize)] pub struct GameConfig { pub winning_score: u32, pub font_size: f32 }

#[derive(Component)] pub struct LeftPaddle;
#[derive(Component)] pub struct RightPaddle;
#[derive(Component)]
#[require(Transform, Velocity)]
pub struct Ball;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);
#[derive(Component)] pub struct Wall;
#[derive(Component)] pub struct Divider;
#[derive(Component)] pub struct LeftScoreText;
#[derive(Component)] pub struct RightScoreText;


#[derive(Resource)] pub struct Score { pub left: u32, pub right: u32 }

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    GameOver,
    Credits,
}
