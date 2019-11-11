use gilrs::{Button, EventType, GamepadId, Gilrs};

use quicksilver::{
    geom::{Line, Rectangle, Shape, Vector},
    graphics::{BlendMode, Image, View},
    input::{ButtonState, Key},
    lifecycle::Window,
    Result,
};

use crate::collision_lines::{collide_many_many, collide_many_one};
use crate::playing::health::Health;
use crate::{
    collision_lines::collides_with,
    constants::*,
    game_state::{
        Action,
        Action::{Continue, Quit},
        GameState,
    },
    line_renderer::LineRenderer,
    playing::{
        camera::Camera,
        collision_assets::CollisionAssets,
        health::reap,
        input::Input,
        landscape::{
            Landscape,
            LandscapeAction::{MakeRocket, MakeTurret},
        },
        player::Player,
        render_assets::RenderAssets,
        rocket::Rocket,
        shot::Shot,
        turret::{Turret, TurretAction::MakeShot},
        world_pos::WorldPos,
    },
};

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
}

impl Playing {
    pub(crate) fn new(line_images: Vec<Image>) -> Result<Self> {
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
            line_renderer: LineRenderer::new(line_images[0].clone()),
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
        })
    }

    /// Collide the player.
    fn collide_player(&mut self) {
        // Collide the player with the landscape.
        if collides_with(&self.player, &self.landscape) {
            self.player.as_mut().kill();
        }

        // Collide the player with the rockets.
        collide_many_one(&mut self.rockets, &mut self.player, |rocket, player| {
            rocket.as_mut().kill();
            player.as_mut().kill();
        });

        // Collide the player with the turrets.
        collide_many_one(&mut self.turrets, &mut self.player, |turret, player| {
            turret.as_mut().kill();
            player.as_mut().kill();
        });
    }

    /// Collide the player's shots.
    fn collide_shots(&mut self) {
        // Collide the player's shots with the landscape.
        collide_many_one(&mut self.shots, &mut self.landscape, |shot, _landscape| {
            shot.as_mut().kill();
        });

        // Collide the player's shots with the rockets.
        collide_many_many(&mut self.shots, &mut self.rockets, |shot, rocket| {
            shot.as_mut().kill();
            rocket.as_mut().kill();
        });

        // Collide the player's shots with the turrets.
        collide_many_many(&mut self.shots, &mut self.turrets, |shot, turret| {
            shot.as_mut().kill();
            turret.as_mut().kill();
        });
    }

    /// Collide the turrets' shots.
    fn collide_turret_shots(&mut self) {
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
        });
    }

    fn check_collisions(&mut self) {
        self.collide_player();
        self.collide_shots();
        self.collide_turret_shots();
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
            let forward_velocity = Vector::new(4, 0);

            self.player.control(forward_velocity, dx, dy, d_theta);
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
            self.camera.pos = self.camera.pos.translate(forward_velocity);

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
        rescale_viewport(window, self.camera.pos);
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
        self.line_renderer.render(window);

        Ok(())
    }
}
