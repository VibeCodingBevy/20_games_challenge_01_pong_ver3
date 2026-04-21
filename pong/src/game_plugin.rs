use bevy::prelude::*;
use crate::components::{Ball, Config, GameState, LeftPaddle, RightPaddle, Score, Velocity, Wall};

const WINNING_SCORE: u32 = 10;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (spawn_game_objects, reset_score))
            .add_systems(OnExit(GameState::InGame), despawn_game_objects)
            .add_systems(FixedUpdate, game_logic.run_if(in_state(GameState::InGame)));
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
}

fn despawn_game_objects(
    mut commands: Commands,
    left_paddles: Query<Entity, With<LeftPaddle>>,
    right_paddles: Query<Entity, With<RightPaddle>>,
    balls: Query<Entity, With<Ball>>,
    walls: Query<Entity, With<Wall>>,
) {
    for entity in left_paddles.iter().chain(right_paddles.iter()).chain(balls.iter()).chain(walls.iter()) {
        commands.entity(entity).despawn();
    }
}

fn game_logic(
    config: Res<Config>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
    mut paddles: ParamSet<(
        Query<&mut Transform, (With<LeftPaddle>, Without<Ball>)>,
        Query<&mut Transform, (With<RightPaddle>, Without<Ball>)>,
    )>,
    ball: Single<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let mut direction = 0.0;
    if keys.pressed(KeyCode::ArrowUp) { direction += 1.0; }
    if keys.pressed(KeyCode::ArrowDown) { direction -= 1.0; }

    let half_w = config.screen.width as f32 / 2.0;
    let half_h = config.screen.height as f32 / 2.0;

    let (lp_pos, rp_pos) = {
        let wt = config.arena.wall_thickness;
        let min_y = -half_h + wt + config.paddle.height / 2.0;
        let max_y = half_h - wt - config.paddle.height / 2.0;

        let mut lp_pos = None;
        let mut rp_pos = None;

        {
            let mut left_q = paddles.p0();
            for t in left_q.iter() { lp_pos = Some(t.translation); }
            if direction != 0.0 {
                for mut t in left_q.iter_mut() {
                    t.translation.y += direction * config.paddle.speed * time.delta_secs();
                    t.translation.y = t.translation.y.clamp(min_y, max_y);
                }
            }
        }

        {
            let mut right_q = paddles.p1();
            for t in right_q.iter() { rp_pos = Some(t.translation); }
            if direction != 0.0 {
                for mut t in right_q.iter_mut() {
                    t.translation.y += direction * config.paddle.speed * time.delta_secs();
                    t.translation.y = t.translation.y.clamp(min_y, max_y);
                }
            }
        }

        (lp_pos, rp_pos)
    };

    let (mut bt, mut velocity) = ball.into_inner();

    let speed = config.ball.speed;
    let r = config.ball.diameter / 2.0;
    let wt = config.arena.wall_thickness;
    let pw = config.paddle.width;
    let ph = config.paddle.height;

    bt.translation += velocity.0.extend(0.0) * time.delta_secs();
    if bt.translation.y - r <= -half_h + wt { bt.translation.y = -half_h + wt + r; velocity.0.y = speed; }
    else if bt.translation.y + r >= half_h - wt { bt.translation.y = half_h - wt - r; velocity.0.y = -speed; }

    if let Some(lp) = lp_pos {
        let lx = lp.x + pw;
        if bt.translation.x - r <= lx && bt.translation.x >= lx - pw && bt.translation.y >= lp.y - ph/2.0 && bt.translation.y <= lp.y + ph/2.0 {
            let ho = (bt.translation.y - lp.y) / (ph / 2.0);
            let a = ho.clamp(-1.0, 1.0) * (std::f32::consts::PI / 4.0);
            velocity.0 = Vec2::new(speed * a.cos(), speed * a.sin());
            bt.translation.x = lx + r;
        }
    }

    if let Some(rp) = rp_pos {
        let rx = rp.x - pw;
        if bt.translation.x + r >= rx && bt.translation.x <= rx + pw && bt.translation.y >= rp.y - ph/2.0 && bt.translation.y <= rp.y + ph/2.0 {
            let ho = (bt.translation.y - rp.y) / (ph / 2.0);
            let a = ho.clamp(-1.0, 1.0) * (std::f32::consts::PI / 4.0);
            velocity.0 = Vec2::new(-speed * a.cos(), speed * a.sin());
            bt.translation.x = rx - r;
        }
    }

    let mut scored = false;
    if bt.translation.x < -half_w { score.right += 1; scored = true; }
    else if bt.translation.x > half_w { score.left += 1; scored = true; }

    if scored {
        bt.translation.x = 0.0;
        bt.translation.y = 0.0;
        velocity.0 = Vec2::new(speed, speed);

        if score.left >= WINNING_SCORE || score.right >= WINNING_SCORE {
            next_state.set(GameState::GameOver);
        }
    }
}