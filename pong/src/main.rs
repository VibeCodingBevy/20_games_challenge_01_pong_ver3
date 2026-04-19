use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

#[derive(Resource, Deserialize)]
struct Config {
    screen: Screen,
    ball: Ball,
    paddle: Paddle,
    arena: Arena,
}

#[derive(Deserialize)]
struct Screen { width: u32, height: u32 }
#[derive(Deserialize)]
struct Ball { diameter: f32, speed: f32 }
#[derive(Deserialize)]
struct Paddle { width: f32, height: f32, margin: f32, speed: f32 }
#[derive(Deserialize)]
struct Arena { wall_thickness: f32 }

#[derive(Component)] struct LeftPaddle;
#[derive(Component)] struct RightPaddle;
#[derive(Component)] struct Ballobj;
#[derive(Component)] struct Velocity(Vec2);

#[derive(Resource)] struct Score { left: u32, right: u32 }

fn main() {
    let config_str = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();

    let screen_width = config.screen.width;
    let screen_height = config.screen.height;

    App::new()
        .insert_resource(config)
        .insert_resource(Score { left: 0, right: 0 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".into(),
                resolution: (screen_width, screen_height).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, game_logic)
        .run();
}

fn setup(mut cmds: Commands, cfg: Res<Config>) {
    cmds.spawn(Camera2d);
    let wt = cfg.arena.wall_thickness;
    let sw = cfg.screen.width as f32;
    let sh = cfg.screen.height as f32;
    let py = sh / 2.0;
    let lx = cfg.paddle.margin + cfg.paddle.width / 2.0;
    let rx = sw - cfg.paddle.margin - cfg.paddle.width / 2.0;

    // Top/bottom walls
    let mut tw = cmds.spawn_empty();
    tw.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(sw, wt)));
    tw.insert(Transform::from_xyz(sw / 2.0, sh - wt / 2.0, 0.0));

    let mut bw = cmds.spawn_empty();
    bw.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(sw, wt)));
    bw.insert(Transform::from_xyz(sw / 2.0, wt / 2.0, 0.0));

    // Paddles
    let mut lp = cmds.spawn_empty();
    lp.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(cfg.paddle.width, cfg.paddle.height)));
    lp.insert(Transform::from_xyz(lx, py, 0.0));
    lp.insert(LeftPaddle);

    let mut rp = cmds.spawn_empty();
    rp.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(cfg.paddle.width, cfg.paddle.height)));
    rp.insert(Transform::from_xyz(rx, py, 0.0));
    rp.insert(RightPaddle);

    // Ball
    let mut b = cmds.spawn_empty();
    b.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(cfg.ball.diameter, cfg.ball.diameter)));
    b.insert(Transform::from_xyz(sw / 2.0, sh / 2.0, 0.0));
    b.insert(Ballobj);
    b.insert(Velocity(Vec2::new(cfg.ball.speed, cfg.ball.speed)));
}

fn game_logic(
    cfg: Res<Config>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut left_paddle_q: Query<&mut Transform, (With<LeftPaddle>, Without<Ballobj>, Without<RightPaddle>)>,
    mut right_paddle_q: Query<&mut Transform, (With<RightPaddle>, Without<Ballobj>, Without<LeftPaddle>)>,
    mut ball_q: Query<(&mut Transform, &mut Velocity), With<Ballobj>>,
) {
    let mut dir = 0.0;
    if keys.pressed(KeyCode::ArrowUp) { dir += 1.0; }
    if keys.pressed(KeyCode::ArrowDown) { dir -= 1.0; }

    let (lp_pos, rp_pos) = {
        let wt = cfg.arena.wall_thickness;
        let sh = cfg.screen.height as f32;
        let min_y = wt + cfg.paddle.height / 2.0;
        let max_y = sh - wt - cfg.paddle.height / 2.0;

        let mut lp_pos = None;
        let mut rp_pos = None;
        for t in left_paddle_q.iter() { lp_pos = Some(t.translation); }
        for t in right_paddle_q.iter() { rp_pos = Some(t.translation); }

        if dir != 0.0 {
            for mut t in left_paddle_q.iter_mut() {
                t.translation.y += dir * cfg.paddle.speed * time.delta_secs();
                t.translation.y = t.translation.y.clamp(min_y, max_y);
            }
            for mut t in right_paddle_q.iter_mut() {
                t.translation.y += dir * cfg.paddle.speed * time.delta_secs();
                t.translation.y = t.translation.y.clamp(min_y, max_y);
            }
        }
        (lp_pos, rp_pos)
    };

    let ball_result = ball_q.single_mut();
    let Ok((mut bt, mut vel)) = ball_result else { return; };

    let speed = cfg.ball.speed;
    let r = cfg.ball.diameter / 2.0;
    let wt = cfg.arena.wall_thickness;
    let sw = cfg.screen.width as f32;
    let sh = cfg.screen.height as f32;
    let pw = cfg.paddle.width;
    let ph = cfg.paddle.height;

    bt.translation += vel.0.extend(0.0) * time.delta_secs();
    if bt.translation.y - r <= wt { bt.translation.y = wt + r; vel.0.y = speed; }
    else if bt.translation.y + r >= sh - wt { bt.translation.y = sh - wt - r; vel.0.y = -speed; }

    if let Some(lp) = lp_pos {
        let lx = lp.x + pw;
        if bt.translation.x - r <= lx && bt.translation.x >= lx - pw && bt.translation.y >= lp.y - ph/2.0 && bt.translation.y <= lp.y + ph/2.0 {
            let ho = (bt.translation.y - lp.y) / (ph / 2.0);
            let a = ho.clamp(-1.0, 1.0) * (std::f32::consts::PI / 4.0);
            vel.0 = Vec2::new(speed * a.cos(), speed * a.sin());
            bt.translation.x = lx + r;
        }
    }

    if let Some(rp) = rp_pos {
        let rx = rp.x - pw;
        if bt.translation.x + r >= rx && bt.translation.x <= rx + pw && bt.translation.y >= rp.y - ph/2.0 && bt.translation.y <= rp.y + ph/2.0 {
            let ho = (bt.translation.y - rp.y) / (ph / 2.0);
            let a = ho.clamp(-1.0, 1.0) * (std::f32::consts::PI / 4.0);
            vel.0 = Vec2::new(-speed * a.cos(), speed * a.sin());
            bt.translation.x = rx - r;
        }
    }

    let mut scored = false;
    if bt.translation.x < 0.0 { score.right += 1; scored = true; }
    else if bt.translation.x > sw { score.left += 1; scored = true; }

    if scored {
        bt.translation.x = sw / 2.0;
        bt.translation.y = sh / 2.0;
        vel.0 = Vec2::new(speed, speed);
    }
}