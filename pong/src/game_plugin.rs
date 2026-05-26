use bevy::prelude::*;
use rand::Rng;
use crate::components::{Ball, Config, GameState, LeftPaddle, RightPaddle, Score, Velocity, Wall, Divider, LeftScoreText, RightScoreText};

const WINNING_SCORE: u32 = 10;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct GameLogicSet;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (spawn_game_objects, reset_score))
            .add_systems(OnExit(GameState::InGame), despawn_game_objects)
            .configure_sets(FixedUpdate, GameLogicSet)
            .add_systems(
                FixedUpdate,
                (
                    move_paddles_system,
                    move_ball_system,
                    handle_wall_collisions_system,
                    handle_paddle_collisions_system,
                    handle_scoring_system,
                )
                .chain()
                .in_set(GameLogicSet)
                .run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, update_score_ui_system.run_if(in_state(GameState::InGame)));
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.left = 0;
    score.right = 0;
}

fn spawn_game_objects(mut commands: Commands, config: Res<Config>) {
    let wall_thickness = config.arena.wall_thickness;
    let screen_width = config.screen.width as f32;
    let screen_height = config.screen.height as f32;
    let half_width = screen_width / 2.0;
    let half_height = screen_height / 2.0;
    let paddle_y = 0.0;
    let left_x = -half_width + config.paddle.margin + config.paddle.width / 2.0;
    let right_x = half_width - config.paddle.margin - config.paddle.width / 2.0;

    let mut top_wall = commands.spawn_empty();
    top_wall.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(screen_width, wall_thickness)));
    top_wall.insert(Transform::from_xyz(0.0, half_height - wall_thickness / 2.0, 0.0));
    top_wall.insert(Wall);

    let mut bottom_wall = commands.spawn_empty();
    bottom_wall.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(screen_width, wall_thickness)));
    bottom_wall.insert(Transform::from_xyz(0.0, -half_height + wall_thickness / 2.0, 0.0));
    bottom_wall.insert(Wall);

    let mut left_paddle = commands.spawn_empty();
    left_paddle.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(config.paddle.width, config.paddle.height)));
    left_paddle.insert(Transform::from_xyz(left_x, paddle_y, 0.0));
    left_paddle.insert(LeftPaddle);

    let mut right_paddle = commands.spawn_empty();
    right_paddle.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(config.paddle.width, config.paddle.height)));
    right_paddle.insert(Transform::from_xyz(right_x, paddle_y, 0.0));
    right_paddle.insert(RightPaddle);

    let mut ball = commands.spawn_empty();
    ball.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(config.ball.diameter, config.ball.diameter)));
    ball.insert(Transform::from_xyz(0.0, 0.0, 0.0));
    ball.insert(Ball);
    ball.insert(Velocity(Vec2::new(config.ball.speed, config.ball.speed)));

    commands.spawn((
        Sprite::from_color(Color::srgb(0.7, 0.7, 0.7), Vec2::new(config.arena.divider_width, screen_height)),
        Transform::from_xyz(0.0, 0.0, -1.0),
        Divider,
    ));

    commands.spawn((
        Text::new("0"),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
        TextFont {
            font_size: 64.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(250.0),
            top: Val::Px(20.0),
            ..default()
        },
        LeftScoreText,
    ));

    commands.spawn((
        Text::new("0"),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
        TextFont {
            font_size: 64.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(250.0),
            top: Val::Px(20.0),
            ..default()
        },
        RightScoreText,
    ));
}

fn despawn_game_objects(
    mut commands: Commands,
    left_paddles: Query<Entity, With<LeftPaddle>>,
    right_paddles: Query<Entity, With<RightPaddle>>,
    balls: Query<Entity, With<Ball>>,
    walls: Query<Entity, With<Wall>>,
    dividers: Query<Entity, With<Divider>>,
    left_score_texts: Query<Entity, With<LeftScoreText>>,
    right_score_texts: Query<Entity, With<RightScoreText>>,
) {
    for entity in left_paddles.iter()
        .chain(right_paddles.iter())
        .chain(balls.iter())
        .chain(walls.iter())
        .chain(dividers.iter())
        .chain(left_score_texts.iter())
        .chain(right_score_texts.iter()) {
        commands.entity(entity).despawn();
    }
}

fn move_paddles_system(
    config: Res<Config>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut paddles: ParamSet<(
        Query<&mut Transform, (With<LeftPaddle>, Without<Ball>)>,
        Query<&mut Transform, (With<RightPaddle>, Without<Ball>)>,
    )>,
) {
    let mut direction = 0.0;
    if keys.pressed(KeyCode::ArrowUp) { direction += 1.0; }
    if keys.pressed(KeyCode::ArrowDown) { direction -= 1.0; }

    if direction == 0.0 {
        return;
    }

    let half_height = config.screen.height as f32 / 2.0;
    let wall_thickness = config.arena.wall_thickness;
    let min_y = -half_height + wall_thickness + config.paddle.height / 2.0;
    let max_y = half_height - wall_thickness - config.paddle.height / 2.0;

    let mut left_query = paddles.p0();
    for mut transform in left_query.iter_mut() {
        transform.translation.y += direction * config.paddle.speed * time.delta_secs();
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }

    let mut right_query = paddles.p1();
    for mut transform in right_query.iter_mut() {
        transform.translation.y += direction * config.paddle.speed * time.delta_secs();
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}

fn move_ball_system(
    time: Res<Time>,
    ball: Single<(&mut Transform, &Velocity), With<Ball>>,
) {
    let (mut ball_transform, velocity) = ball.into_inner();
    ball_transform.translation += velocity.0.extend(0.0) * time.delta_secs();
}

fn handle_wall_collisions_system(
    config: Res<Config>,
    ball: Single<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let (mut ball_transform, mut velocity) = ball.into_inner();
    let half_height = config.screen.height as f32 / 2.0;
    let radius = config.ball.diameter / 2.0;
    let wall_thickness = config.arena.wall_thickness;
    let speed = config.ball.speed;

    if ball_transform.translation.y - radius <= -half_height + wall_thickness {
        ball_transform.translation.y = -half_height + wall_thickness + radius;
        velocity.0.y = speed;
    } else if ball_transform.translation.y + radius >= half_height - wall_thickness {
        ball_transform.translation.y = half_height - wall_thickness - radius;
        velocity.0.y = -speed;
    }
}

fn handle_paddle_collisions_system(
    config: Res<Config>,
    ball: Single<(&mut Transform, &mut Velocity), With<Ball>>,
    left_paddles: Query<&Transform, (With<LeftPaddle>, Without<Ball>)>,
    right_paddles: Query<&Transform, (With<RightPaddle>, Without<Ball>)>,
) {
    let (mut ball_transform, mut velocity) = ball.into_inner();
    let radius = config.ball.diameter / 2.0;
    let paddle_width = config.paddle.width;
    let paddle_height = config.paddle.height;
    let speed = config.ball.speed;

    for left_paddle in left_paddles.iter() {
        let paddle_right = left_paddle.translation.x + paddle_width / 2.0;
        let paddle_left = left_paddle.translation.x - paddle_width / 2.0;

        if ball_transform.translation.x - radius <= paddle_right && ball_transform.translation.x + radius >= paddle_left
            && ball_transform.translation.y >= left_paddle.translation.y - paddle_height / 2.0
            && ball_transform.translation.y <= left_paddle.translation.y + paddle_height / 2.0 {
            let offset = (ball_transform.translation.y - left_paddle.translation.y) / (paddle_height / 2.0);
            let angle = offset.clamp(-1.0, 1.0) * (std::f32::consts::PI / 4.0);
            velocity.0 = Vec2::new(speed * angle.cos(), speed * angle.sin());
            ball_transform.translation.x = paddle_right + radius;
            break;
        }
    }

    for right_paddle in right_paddles.iter() {
        let paddle_right = right_paddle.translation.x + paddle_width / 2.0;
        let paddle_left = right_paddle.translation.x - paddle_width / 2.0;

        if ball_transform.translation.x + radius >= paddle_left && ball_transform.translation.x - radius <= paddle_right
            && ball_transform.translation.y >= right_paddle.translation.y - paddle_height / 2.0
            && ball_transform.translation.y <= right_paddle.translation.y + paddle_height / 2.0 {
            let offset = (ball_transform.translation.y - right_paddle.translation.y) / (paddle_height / 2.0);
            let angle = offset.clamp(-1.0, 1.0) * (std::f32::consts::PI / 4.0);
            velocity.0 = Vec2::new(-speed * angle.cos(), speed * angle.sin());
            ball_transform.translation.x = paddle_left - radius;
            break;
        }
    }
}

fn handle_scoring_system(
    config: Res<Config>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
    ball: Single<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let (mut ball_transform, mut velocity) = ball.into_inner();
    let half_width = config.screen.width as f32 / 2.0;
    let speed = config.ball.speed;

    let mut scored = false;
    let mut scored_left = false;
    if ball_transform.translation.x < -half_width {
        score.right += 1;
        scored = true;
    } else if ball_transform.translation.x > half_width {
        score.left += 1;
        scored_left = true;
        scored = true;
    }

    if scored {
        ball_transform.translation.x = 0.0;
        ball_transform.translation.y = 0.0;

        let mut rng = rand::rng();
        let y_direction = if rng.random::<f32>() > 0.5 { speed } else { -speed };
        velocity.0 = if scored_left {
            Vec2::new(speed, y_direction)
        } else {
            Vec2::new(-speed, y_direction)
        };

        if score.left >= WINNING_SCORE || score.right >= WINNING_SCORE {
            next_state.set(GameState::GameOver);
        }
    }
}

fn update_score_ui_system(
    score: Res<Score>,
    mut left_text: Single<&mut Text, (With<LeftScoreText>, Without<RightScoreText>)>,
    mut right_text: Single<&mut Text, (With<RightScoreText>, Without<LeftScoreText>)>,
) {
    if score.is_changed() {
        **left_text = Text::new(score.left.to_string());
        **right_text = Text::new(score.right.to_string());
    }
}
