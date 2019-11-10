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
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
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
            gilrs: Gilrs::new()?,
            active_gamepad: None,
            render_assets: render_assets,
            collision_assets: collision_assets,
        })
    }

    /// Poll all possible input sources.
    fn poll_inputs(&mut self, window: &mut Window) -> (bool, bool, f32, f32, f32) {
        let mut quit = false;
        let mut right_pressed = false;
        let mut left_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut rotate_anticlockwise = false;
        let mut rotate_clockwise = false;
        let mut fire: bool = false;

        // Examine new gamepad events using GilRs directly as Quicksilver doesn't see some of the
        // buttons.
        while let Some(event) = self.gilrs.next_event() {
            if self.active_gamepad.is_none() {
                // If we don't have an active gamepad yet, then we do now.
                self.active_gamepad = Some(event.id);
            }

            // Check the gamepad for edge-triggered scenarios such as a button being pressed or
            // released in this turn.
            match event.event {
                // Quitting and firing are edge-triggered.
                EventType::ButtonReleased(Button::Start, _) => quit = true,
                EventType::ButtonPressed(Button::South, _) => fire = true,
                _ => (),
            };
        }

        // Check the gamepad for level-triggered events, such as button being held down. All
        // movement is level-triggered.
        if let Some(id) = self.active_gamepad {
            let gamepad = self.gilrs.gamepad(id);
            left_pressed = left_pressed || gamepad.is_pressed(Button::DPadLeft);
            right_pressed = right_pressed || gamepad.is_pressed(Button::DPadRight);
            up_pressed = up_pressed || gamepad.is_pressed(Button::DPadUp);
            down_pressed = down_pressed || gamepad.is_pressed(Button::DPadDown);
            rotate_anticlockwise = rotate_anticlockwise || gamepad.is_pressed(Button::West);
            rotate_clockwise = rotate_clockwise || gamepad.is_pressed(Button::East);
        }

        // Check the keyboard for edge-triggered events. Quitting and firing are edge-triggered.
        quit = quit || window.keyboard()[Key::Escape] == ButtonState::Released;
        fire = fire || window.keyboard()[Key::Space] == ButtonState::Pressed;

        // Check the keyboard for level-triggered events. All movement is level-triggered.
        left_pressed = left_pressed
            || window.keyboard()[Key::Left].is_down()
            || window.keyboard()[Key::A].is_down();
        right_pressed = right_pressed
            || window.keyboard()[Key::Right].is_down()
            || window.keyboard()[Key::D].is_down();
        up_pressed = up_pressed
            || window.keyboard()[Key::Up].is_down()
            || window.keyboard()[Key::W].is_down();
        down_pressed = down_pressed
            || window.keyboard()[Key::Down].is_down()
            || window.keyboard()[Key::S].is_down();
        rotate_anticlockwise = rotate_anticlockwise || window.keyboard()[Key::Q].is_down();
        rotate_clockwise = rotate_clockwise || window.keyboard()[Key::E].is_down();

        let dx = match (left_pressed, right_pressed) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
        let dy = match (up_pressed, down_pressed) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
        let rotate_by = match (rotate_anticlockwise, rotate_clockwise) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        (quit, fire, dx, dy, rotate_by)
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
        let (quit, fire, dx, dy, d_theta) = self.poll_inputs(window);

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
