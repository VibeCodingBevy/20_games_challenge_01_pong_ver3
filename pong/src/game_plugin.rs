use bevy::prelude::*;
use crate::components::{Ball, Config, GameState, LeftPaddle, RightPaddle, Score, Velocity, Wall, Divider};

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
            );
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.left = 0;
    score.right = 0;
}

fn spawn_game_objects(mut cmds: Commands, config: Res<Config>) {
    let wt = config.arena.wall_thickness;
    let sw = config.screen.width as f32;
    let sh = config.screen.height as f32;
    let half_w = sw / 2.0;
    let half_h = sh / 2.0;
    let py = 0.0;
    let lx = -half_w + config.paddle.margin + config.paddle.width / 2.0;
    let rx = half_w - config.paddle.margin - config.paddle.width / 2.0;

    let mut tw = cmds.spawn_empty();
    tw.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(sw, wt)));
    tw.insert(Transform::from_xyz(0.0, half_h - wt / 2.0, 0.0));
    tw.insert(Wall);

    let mut bw = cmds.spawn_empty();
    bw.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(sw, wt)));
    bw.insert(Transform::from_xyz(0.0, -half_h + wt / 2.0, 0.0));
    bw.insert(Wall);

    let mut lp = cmds.spawn_empty();
    lp.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(config.paddle.width, config.paddle.height)));
    lp.insert(Transform::from_xyz(lx, py, 0.0));
    lp.insert(LeftPaddle);

    let mut rp = cmds.spawn_empty();
    rp.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(config.paddle.width, config.paddle.height)));
    rp.insert(Transform::from_xyz(rx, py, 0.0));
    rp.insert(RightPaddle);

    let mut b = cmds.spawn_empty();
    b.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(config.ball.diameter, config.ball.diameter)));
    b.insert(Transform::from_xyz(0.0, 0.0, 0.0));
    b.insert(Ball);
    b.insert(Velocity(Vec2::new(config.ball.speed, config.ball.speed)));

    cmds.spawn((
        Sprite::from_color(Color::srgb(0.7, 0.7, 0.7), Vec2::new(config.arena.divider_width, sh)),
        Transform::from_xyz(0.0, 0.0, -1.0),
        Divider,
    ));
}

fn despawn_game_objects(
    mut commands: Commands,
    left_paddles: Query<Entity, With<LeftPaddle>>,
    right_paddles: Query<Entity, With<RightPaddle>>,
    balls: Query<Entity, With<Ball>>,
    walls: Query<Entity, With<Wall>>,
    dividers: Query<Entity, With<Divider>>,
) {
    for entity in left_paddles.iter()
        .chain(right_paddles.iter())
        .chain(balls.iter())
        .chain(walls.iter())
        .chain(dividers.iter()) {
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

    let half_h = config.screen.height as f32 / 2.0;
    let wt = config.arena.wall_thickness;
    let min_y = -half_h + wt + config.paddle.height / 2.0;
    let max_y = half_h - wt - config.paddle.height / 2.0;

    let mut left_q = paddles.p0();
    for mut t in left_q.iter_mut() {
        t.translation.y += direction * config.paddle.speed * time.delta_secs();
        t.translation.y = t.translation.y.clamp(min_y, max_y);
    }

    let mut right_q = paddles.p1();
    for mut t in right_q.iter_mut() {
        t.translation.y += direction * config.paddle.speed * time.delta_secs();
        t.translation.y = t.translation.y.clamp(min_y, max_y);
    }
}

fn move_ball_system(
    time: Res<Time>,
    ball: Single<(&mut Transform, &Velocity), With<Ball>>,
) {
    let (mut bt, velocity) = ball.into_inner();
    bt.translation += velocity.0.extend(0.0) * time.delta_secs();
}

fn handle_wall_collisions_system(
    config: Res<Config>,
    ball: Single<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let (mut bt, mut velocity) = ball.into_inner();
    let half_h = config.screen.height as f32 / 2.0;
    let r = config.ball.diameter / 2.0;
    let wt = config.arena.wall_thickness;
    let speed = config.ball.speed;

    if bt.translation.y - r <= -half_h + wt {
        bt.translation.y = -half_h + wt + r;
        velocity.0.y = speed;
    } else if bt.translation.y + r >= half_h - wt {
        bt.translation.y = half_h - wt - r;
        velocity.0.y = -speed;
    }
}

fn handle_paddle_collisions_system(
    config: Res<Config>,
    ball: Single<(&mut Transform, &mut Velocity), With<Ball>>,
    left_paddles: Query<&Transform, (With<LeftPaddle>, Without<Ball>)>,
    right_paddles: Query<&Transform, (With<RightPaddle>, Without<Ball>)>,
) {
    let (mut bt, mut velocity) = ball.into_inner();
    let r = config.ball.diameter / 2.0;
    let pw = config.paddle.width;
    let ph = config.paddle.height;
    let speed = config.ball.speed;

    for lp in left_paddles.iter() {
        let paddle_right = lp.translation.x + pw / 2.0;
        let paddle_left = lp.translation.x - pw / 2.0;

        if bt.translation.x - r <= paddle_right && bt.translation.x + r >= paddle_left
            && bt.translation.y >= lp.translation.y - ph / 2.0
            && bt.translation.y <= lp.translation.y + ph / 2.0 {
            let offset = (bt.translation.y - lp.translation.y) / (ph / 2.0);
            let angle = offset.clamp(-1.0, 1.0) * (std::f32::consts::PI / 4.0);
            velocity.0 = Vec2::new(speed * angle.cos(), speed * angle.sin());
            bt.translation.x = paddle_right + r;
            break;
        }
    }

    for rp in right_paddles.iter() {
        let paddle_right = rp.translation.x + pw / 2.0;
        let paddle_left = rp.translation.x - pw / 2.0;

        if bt.translation.x + r >= paddle_left && bt.translation.x - r <= paddle_right
            && bt.translation.y >= rp.translation.y - ph / 2.0
            && bt.translation.y <= rp.translation.y + ph / 2.0 {
            let offset = (bt.translation.y - rp.translation.y) / (ph / 2.0);
            let angle = offset.clamp(-1.0, 1.0) * (std::f32::consts::PI / 4.0);
            velocity.0 = Vec2::new(-speed * angle.cos(), speed * angle.sin());
            bt.translation.x = paddle_left - r;
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
    let (mut bt, mut velocity) = ball.into_inner();
    let half_w = config.screen.width as f32 / 2.0;
    let speed = config.ball.speed;

    let mut scored = false;
    if bt.translation.x < -half_w {
        score.right += 1;
        scored = true;
    } else if bt.translation.x > half_w {
        score.left += 1;
        scored = true;
    }

    if scored {
        bt.translation.x = 0.0;
        bt.translation.y = 0.0;
        velocity.0 = Vec2::new(speed, speed);

        if score.left >= WINNING_SCORE || score.right >= WINNING_SCORE {
            next_state.set(GameState::GameOver);
        }
    }
}
