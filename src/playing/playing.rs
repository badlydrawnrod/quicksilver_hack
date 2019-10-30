use super::camera::Camera;
use super::killable::Reap;
use super::landscape::Landscape;
use super::player::Player;
use super::shot::Shot;
use super::turret::Turret;

use gilrs::{Button, EventType, GamepadId, Gilrs};

use quicksilver::{
    geom::{Line, Rectangle, Shape, Vector},
    graphics::{BlendMode, Image, View},
    input::{ButtonState, Key},
    lifecycle::Window,
    Result,
};

use crate::constants::*;
use crate::game_state::{
    Action,
    Action::{Continue, Quit},
    GameState,
};
use crate::line_renderer::LineRenderer;
use crate::playing::world_pos::WorldPos;

pub struct Playing {
    camera: Camera,
    line_renderer: LineRenderer,
    player: Player,
    landscape: Landscape,
    shots: Vec<Shot>,
    turrets: Vec<Turret>,
    turret_shots: Vec<Shot>,
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
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
        Ok(Self {
            camera: Camera {
                pos: Vector::new(0, 0),
            },
            line_renderer: LineRenderer::new(line_images[0].clone()),
            player: Player::new(Vector::new(VIRTUAL_WIDTH / 4, VIRTUAL_HEIGHT / 4), 90.0),
            landscape: Landscape::new(),
            shots: Vec::new(),
            turrets: Vec::new(),
            turret_shots: Vec::new(),
            gilrs: Gilrs::new()?,
            active_gamepad: None,
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

    /// Collide the player's shots and check for them going out of bounds.
    fn collide_shots(&mut self) {
        let playfield = Rectangle::new(
            self.camera.pos + Vector::new(-16.0, -16.0),
            (VIRTUAL_WIDTH as f32 + 32.0, VIRTUAL_HEIGHT as f32 + 32.0),
        );

        for shot in &mut self.shots {
            // Collide the shot with the landscape.
            if shot
                .collision_lines
                .intersects(&self.landscape.collision_lines)
            {
                shot.alive = false;
            }

            // Collide the shot with the turrets.
            for turret in &mut self.turrets {
                if shot.collision_lines.intersects(&turret.collision_lines) {
                    shot.alive = false;
                    turret.alive = false;
                }
            }

            // Check for the shot going out of bounds.
            shot.alive = shot.alive && playfield.contains(shot.world_pos());
        }
    }

    // Collide the turret shots and check for them going out of bounds.
    fn collide_turret_shots(&mut self) {
        let playfield = Rectangle::new(
            self.camera.pos + Vector::new(-16.0, -16.0),
            (VIRTUAL_WIDTH as f32 + 32.0, VIRTUAL_HEIGHT as f32 + 32.0),
        );

        for shot in &mut self.turret_shots {
            // Collide the shot with the landscape.
            if shot
                .collision_lines
                .intersects(&self.landscape.collision_lines)
            {
                shot.alive = false;
            }

            // Collide the shot with the player.
            if shot
                .collision_lines
                .intersects(&self.player.collision_lines)
            {
                shot.alive = false;
                self.player.alive = false;
            }

            // Check for the shot going out of bounds.
            shot.alive = shot.alive && playfield.contains(shot.world_pos());
        }
    }

    /// Collide the player.
    fn collide_player(&mut self) {
        // Collide the player with the landscape.
        if self
            .player
            .collision_lines
            .intersects(&self.landscape.collision_lines)
        {
            self.player.alive = false;
        }

        // Collide the player with the turrets.
        for turret in &mut self.turrets {
            if self
                .player
                .collision_lines
                .intersects(&turret.collision_lines)
            {
                self.player.alive = false;
                turret.alive = false;
            }
        }
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

        if self.player.alive {
            let forced_scroll = Vector::new(4, 0);

            self.player.control(forced_scroll, dx, dy, d_theta);
            if fire {
                let shot = Shot::new(self.player.world_pos(), forced_scroll, self.player.angle);
                self.shots.push(shot);
            }
            self.camera.pos = self.camera.pos.translate(forced_scroll);

            self.landscape.update(&self.camera);
            if self.landscape.want_turret {
                if let Some(last_line) = self.landscape.render_lines.last() {
                    let angle = (last_line.line.a - last_line.line.b).angle();
                    let midpoint = last_line.line.center();
                    let turret = Turret::new(midpoint, angle);
                    self.turrets.push(turret);
                }
            }

            for turret in &mut self.turrets {
                turret.control(&self.camera);
                if turret.is_firing {
                    let shot = Shot::new(
                        turret.world_pos() + Vector::new(0, -8),
                        Vector::ZERO,
                        turret.angle + 180.0,
                    );
                    self.turret_shots.push(shot);
                }
            }

            for shot in &mut self.shots {
                shot.control();
            }

            for shot in &mut self.turret_shots {
                shot.control();
            }

            self.collide_player();
            self.collide_shots();
            self.collide_turret_shots();
            self.shots.reap();
            self.turrets.reap();
            self.turret_shots.reap();
        }

        let result = if quit { Quit } else { Continue };
        result.into()
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        rescale_viewport(window, self.camera.pos);
        window.set_blend_mode(BlendMode::Additive)?;
        self.line_renderer.clear();
        self.landscape.draw(&mut self.line_renderer);
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
