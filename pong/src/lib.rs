use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize)]
pub struct Config {
    pub screen: Screen,
    pub ball: Ball,
    pub paddle: Paddle,
    pub arena: Arena,
}

#[derive(Deserialize)]
pub struct Screen { pub width: u32, pub height: u32 }
#[derive(Deserialize)]
pub struct Ball { pub diameter: f32, pub speed: f32 }
#[derive(Deserialize)]
pub struct Paddle { pub width: f32, pub height: f32, pub margin: f32, pub speed: f32 }
#[derive(Deserialize)]
pub struct Arena { pub wall_thickness: f32 }

#[derive(Component)] pub struct LeftPaddle;
#[derive(Component)] pub struct RightPaddle;
#[derive(Component)] pub struct Ballobj;
#[derive(Component)] pub struct Velocity(pub Vec2);

#[derive(Resource)] pub struct Score { pub left: u32, pub right: u32 }

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, game_logic);
    }
}

fn setup(mut cmds: Commands, config: Res<Config>) {
    cmds.spawn(Camera2d);
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

    let mut bw = cmds.spawn_empty();
    bw.insert(Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(sw, wt)));
    bw.insert(Transform::from_xyz(0.0, -half_h + wt / 2.0, 0.0));

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
    b.insert(Ballobj);
    b.insert(Velocity(Vec2::new(config.ball.speed, config.ball.speed)));
}

fn game_logic(
    config: Res<Config>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut paddles: ParamSet<(
        Query<&mut Transform, (With<LeftPaddle>, Without<Ballobj>)>,
        Query<&mut Transform, (With<RightPaddle>, Without<Ballobj>)>,
    )>,
    ball: Single<(&mut Transform, &mut Velocity), With<Ballobj>>,
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
    }
}
