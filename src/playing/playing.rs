use crate::{
    collision_lines::{collide_many_many, collide_many_one, collides_with},
    constants::*,
    font::{text_to_model, VectorFont},
    game_state::{
        Action,
        Action::{Continue, Quit},
        GameState,
    },
    line_renderer::{LineRenderer, RenderModel},
    playing::{
        camera::Camera,
        collision_assets::CollisionAssets,
        health::{reap, Health},
        input::Input,
        landscape::{
            Landscape,
            LandscapeAction::{MakeRocket, MakeTurret},
        },
        particles::Particles,
        player::Player,
        render_assets::RenderAssets,
        rocket::Rocket,
        shot::Shot,
        turret::{Turret, TurretAction::MakeShot},
        world_pos::WorldPos,
    },
};

use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{BlendMode, Image, View},
    lifecycle::Window,
    Result,
};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_arch = "wasm32")]
use stdweb::web::Date;

use std::collections::HashMap;

const FORWARD_SPEED: f32 = 240.0;

const ROCKET_SCORE: i32 = 200;
const TURRET_SCORE: i32 = 150;

pub struct Playing {
    camera: Camera,
    line_renderer: LineRenderer,
    player: Player,
    landscape: Landscape,
    shots: Vec<Shot>,
    rockets: Vec<Rocket>,
    turrets: Vec<Turret>,
    turret_shots: Vec<Shot>,
    input: Input,
    render_assets: RenderAssets,
    collision_assets: CollisionAssets,
    particles: Particles,
    font: VectorFont,
    score: i32,
    score_model: RenderModel,
    redraw_score: bool,
    lives: i32,
    lives_model: RenderModel,
    redraw_lives: bool,
    high_score: i32,
    high_score_model: RenderModel,
    redraw_high_score: bool,
    last_draw_time: f64,
    delta: f32,
}

impl Playing {
    pub(crate) fn new(images: HashMap<String, Image>) -> Result<Self> {
        let mut landscape = Vec::new();
        let mut last_point = Vector::new(0.0, 15 * WINDOW_HEIGHT / 16);
        for x in (0..WINDOW_WIDTH + 32).step_by(32) {
            let next_point = Vector::new(x, last_point.y);
            landscape.push(Line::new(last_point, next_point));
            last_point = next_point;
        }
        let render_assets = RenderAssets::new();
        let collision_assets = CollisionAssets::new();
        Ok(Self {
            camera: Camera {
                pos: Vector::new(0, 0),
            },
            line_renderer: LineRenderer::new(images["line"].clone()),
            player: Player::new(
                render_assets.player(),
                collision_assets.player(),
                Vector::new(VIRTUAL_WIDTH / 4, VIRTUAL_HEIGHT / 4),
                90.0,
            ),
            landscape: Landscape::new(),
            shots: Vec::new(),
            rockets: Vec::new(),
            turrets: Vec::new(),
            turret_shots: Vec::new(),
            input: Input::new()?,
            render_assets: render_assets,
            collision_assets: collision_assets,
            particles: Particles::new(512, images["particle"].clone()),
            font: VectorFont::new(),
            score: 0,
            score_model: RenderModel::new(Vec::new()),
            redraw_score: true,
            lives: 3,
            lives_model: RenderModel::new(Vec::new()),
            redraw_lives: true,
            high_score: 5000,
            high_score_model: RenderModel::new(Vec::new()),
            redraw_high_score: true,
            last_draw_time: current_time(),
            delta: 0.0,
        })
    }

    /// Collide the player.
    fn collide_player(&mut self) {
        let particles = &mut self.particles;
        let player_pos = self.player.world_pos();

        // Collide the player with the landscape.
        if collides_with(&self.player, &self.landscape) {
            self.player.as_mut().kill();
            particles.add(128, player_pos, -90.0, 180.0);
        }

        // Collide the player with the rockets.
        collide_many_one(&mut self.rockets, &mut self.player, |rocket, player| {
            rocket.as_mut().kill();
            player.as_mut().kill();
            particles.add(48, rocket.world_pos(), 0.0, 180.0);
            particles.add(128, player_pos, -90.0, 180.0);
        });

        // Collide the player with the turrets.
        collide_many_one(&mut self.turrets, &mut self.player, |turret, player| {
            turret.as_mut().kill();
            player.as_mut().kill();
            particles.add(64, turret.world_pos(), 0.0, 45.0);
            particles.add(128, player_pos, -90.0, 180.0);
        });
    }

    /// Collide the player's shots.
    fn collide_shots(&mut self) {
        let particles = &mut self.particles;
        let old_score = self.score;
        let old_high_score = self.high_score;

        let score = &mut self.score;

        // Collide the player's shots with the landscape.
        collide_many_one(&mut self.shots, &mut self.landscape, |shot, _landscape| {
            shot.as_mut().kill();
            particles.add(4, shot.world_pos(), shot.angle() - 180.0, 15.0);
        });

        // Collide the player's shots with the rockets.
        collide_many_many(&mut self.shots, &mut self.rockets, |shot, rocket| {
            shot.as_mut().kill();
            rocket.as_mut().kill();
            particles.add(48, rocket.world_pos(), shot.angle(), 30.0);
            *score += ROCKET_SCORE;
        });

        // Collide the player's shots with the turrets.
        collide_many_many(&mut self.shots, &mut self.turrets, |shot, turret| {
            shot.as_mut().kill();
            turret.as_mut().kill();
            particles.add(64, turret.world_pos(), 0.0, 45.0);
            *score += TURRET_SCORE;
        });

        self.high_score = self.high_score.max(self.score);

        self.redraw_score = self.redraw_score || (self.score != old_score);
        self.redraw_high_score = self.redraw_high_score || (self.high_score != old_high_score);
    }

    /// Collide the turrets' shots.
    fn collide_turret_shots(&mut self) {
        let particles = &mut self.particles;

        // Collide the turrets' shots with the landscape.
        collide_many_one(
            &mut self.turret_shots,
            &mut self.landscape,
            |shot, _landscape| {
                shot.as_mut().kill();
            },
        );

        // Collide the turrets' shots with the player.
        collide_many_one(&mut self.turret_shots, &mut self.player, |shot, player| {
            shot.as_mut().kill();
            player.as_mut().kill();
            particles.add(128, player.world_pos(), -90.0, 180.0);
        });
    }

    fn check_collisions(&mut self) {
        self.collide_player();
        self.collide_shots();
        self.collide_turret_shots();
    }

    fn draw_status(&mut self) {
        if self.redraw_score {
            self.redraw_score = false;
            let score = format!("SCORE:{:06}", self.score);
            self.score_model = text_to_model(&self.font, score.as_str());
        }
        self.line_renderer.add_model(
            self.score_model.clone(),
            Transform::translate(self.camera.pos + Vector::new(4.0, 28.0)),
        );

        if self.redraw_high_score {
            self.redraw_high_score = false;
            let high_score = format!("HIGH:{:06}", self.high_score);
            self.high_score_model = text_to_model(&self.font, high_score.as_str());
        }
        self.line_renderer.add_model(
            self.high_score_model.clone(),
            Transform::translate(
                self.camera.pos + Vector::new(VIRTUAL_WIDTH as f32 / 2.0 - 140.0, 28.0),
            ),
        );

        if self.redraw_lives {
            self.redraw_lives = false;
            let lives = format!("LIVES:{}", self.lives);
            self.lives_model = text_to_model(&self.font, lives.as_str());
        }
        self.line_renderer.add_model(
            self.lives_model.clone(),
            Transform::translate(self.camera.pos + Vector::new(VIRTUAL_WIDTH as f32 - 140.0, 28.0)),
        );
    }

    fn draw_diags(&mut self, window: &mut Window) {
        // Current FPS.
        let fps = format!("FPS: {:2.2}", window.current_fps());
        let fps_model = text_to_model(&self.font, fps.as_str());
        self.line_renderer.add_model(
            fps_model,
            Transform::translate(
                self.camera.pos
                    + Vector::new(VIRTUAL_WIDTH as f32 - 200.0, VIRTUAL_HEIGHT as f32 - 64.0),
            ),
        );
        // Average FPS.
        let fps = format!("AVG: {:2.2}", window.average_fps());
        let fps_model = text_to_model(&self.font, fps.as_str());
        self.line_renderer.add_model(
            fps_model,
            Transform::translate(
                self.camera.pos
                    + Vector::new(VIRTUAL_WIDTH as f32 - 200.0, VIRTUAL_HEIGHT as f32 - 128.0),
            ),
        );
        // Delta time.
        let t = format!("DEL: {:02.2}", self.delta);
        let t_model = text_to_model(&self.font, t.as_str());
        self.line_renderer.add_model(
            t_model,
            Transform::translate(
                self.camera.pos
                    + Vector::new(VIRTUAL_WIDTH as f32 - 200.0, VIRTUAL_HEIGHT as f32 - 192.0),
            ),
        );
    }
}

fn rescale_viewport(window: &mut Window, translate: Vector) {
    let view_rect = Rectangle::new(translate, Vector::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
    let view = View::new(view_rect);
    window.set_view(view);
}

impl GameState for Playing {
    fn update(&mut self, window: &mut Window) -> Result<Action> {
        let (quit, fire, dx, dy, d_theta) = self.input.poll(window);

        let health: &Health = self.player.as_ref();
        if health.is_alive() {
            let forward_velocity = Vector::new(FORWARD_SPEED * FIXED_UPDATE_INTERVAL_S as f32, 0.0);
            self.player.control(dx, dy, d_theta);
            if fire {
                let shot = Shot::new(
                    self.render_assets.shot(),
                    self.collision_assets.shot(),
                    self.player.world_pos(),
                    forward_velocity,
                    self.player.angle(),
                );
                self.shots.push(shot);
            }

            match self.landscape.update(&self.camera) {
                MakeTurret(line) => {
                    let angle = (line.a - line.b).angle();
                    let midpoint = line.center();
                    let turret = Turret::new(
                        self.render_assets.turret(),
                        self.collision_assets.turret(),
                        midpoint,
                        angle,
                    );
                    self.turrets.push(turret);
                }
                MakeRocket(line) => {
                    let angle = (line.a - line.b).angle();
                    let midpoint = line.center();
                    let rocket = Rocket::new(
                        self.render_assets.rocket(),
                        self.collision_assets.rocket(),
                        midpoint + Vector::new(0, -16),
                        angle + 180.0,
                    );
                    self.rockets.push(rocket);
                }
                _ => {}
            }

            let playfield = Rectangle::new(
                self.camera.pos + Vector::new(-16.0, -16.0),
                (VIRTUAL_WIDTH as f32 + 64.0, VIRTUAL_HEIGHT as f32 + 32.0),
            );

            for rocket in &mut self.rockets {
                rocket.control(&playfield);
            }

            for turret in &mut self.turrets {
                match turret.control(&playfield) {
                    MakeShot(pos, angle) => {
                        let shot = Shot::new(
                            self.render_assets.shot(),
                            self.collision_assets.shot(),
                            pos + Vector::new(0, -8),
                            Vector::ZERO,
                            angle + 180.0,
                        );
                        self.turret_shots.push(shot);
                    }
                    _ => {}
                }
            }

            for shot in &mut self.shots {
                shot.control(&playfield);
            }

            for shot in &mut self.turret_shots {
                shot.control(&playfield);
            }

            self.check_collisions();
            reap(&mut self.rockets);
            reap(&mut self.shots);
            reap(&mut self.turrets);
            reap(&mut self.turret_shots);
        }

        let result = if quit { Quit } else { Continue };
        result.into()
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let now = current_time();
        let delta = (now - self.last_draw_time) as f32;
        self.delta = delta;
        self.last_draw_time = now;

        let health: &Health = self.player.as_ref();
        if health.is_alive() {
            // Drive the player and camera forward. The player is moved with the camera rather than
            // at a fixed rate to prevent it from stuttering when the update rate and draw rate are
            // not in sync.
            let forward_velocity = Vector::new(FORWARD_SPEED * 0.001 * self.delta as f32, 0.0);
            self.player.advance(forward_velocity);
            self.camera.pos = self.camera.pos.translate(forward_velocity);
        }

        let translate = self.camera.pos;
        rescale_viewport(window, translate);

        window.set_blend_mode(BlendMode::Additive)?;
        self.line_renderer.clear();
        self.landscape.draw(&mut self.line_renderer);
        for rocket in &self.rockets {
            rocket.draw(&mut self.line_renderer);
        }
        for turret in &self.turrets {
            turret.draw(&mut self.line_renderer);
        }
        self.player.draw(&mut self.line_renderer);
        for shot in &self.shots {
            shot.draw(&mut self.line_renderer);
        }
        for shot in &self.turret_shots {
            shot.draw(&mut self.line_renderer);
        }
        self.draw_status();
        if DRAW_DIAGS {
            self.draw_diags(window);
        }

        self.line_renderer.render(window);

        window.reset_blend_mode()?;
        self.particles.draw(window, delta);

        Ok(())
    }
}

// "Borrowed" from Quicksilver as I want to know the time.

#[cfg(not(target_arch = "wasm32"))]
fn current_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as f64 * 1000.0 + since_the_epoch.subsec_nanos() as f64 / 1e6
}

#[cfg(target_arch = "wasm32")]
fn current_time() -> f64 {
    Date::now()
}
