use gilrs::{Button, EventType, GamepadId, Gilrs};

use rand::{prelude::*, Rng};

use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{BlendMode, Color, Image, View},
    input::{ButtonState, Key},
    lifecycle::Window,
    Result,
};

use crate::collision_lines::CollisionLines;
use crate::constants::*;
use crate::game_state::{
    Action,
    Action::{Continue, Quit},
    GameState,
};
use crate::line_renderer::{LineRenderer, TintedLine};
use crate::transformed::Transformable;

trait Kill {
    fn kill(&mut self);
    fn is_dead(&self) -> bool;
}

macro_rules! killable {
    ($name : ident) => {
        impl Kill for $name {
            fn kill(&mut self) {
                self.alive = false;
            }

            fn is_dead(&self) -> bool {
                !self.alive
            }
        }
    };
}

trait Reap {
    fn reap(&mut self);
}

impl<T: Kill> Reap for Vec<T> {
    fn reap(&mut self) {
        let mut i = 0;
        while i < self.len() {
            if self[i].is_dead() {
                self.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

struct Camera {
    pos: Vector,
}

struct Turret {
    pos: Vector,
    angle: f32,
    model_lines: Vec<TintedLine>,
    render_lines: Vec<TintedLine>,
    collision_lines: CollisionLines,
    alive: bool,
}

killable!(Turret);

impl Turret {
    fn new(pos: Vector, angle: f32) -> Self {
        let lines = vec![
            TintedLine::new((-16, 0), (16, 0), Color::GREEN),
            TintedLine::new((-16, 0), (-12, 16), Color::GREEN),
            TintedLine::new((-12, 16), (-4, 16), Color::GREEN),
            TintedLine::new((-4, 16), (0, 24), Color::GREEN),
            TintedLine::new((0, 24), (4, 16), Color::GREEN),
            TintedLine::new((-4, 16), (12, 16), Color::GREEN),
            TintedLine::new((12, 16), (16, 0), Color::GREEN),
        ];
        let length = lines.len();
        Turret {
            pos,
            angle,
            model_lines: lines,
            render_lines: Vec::with_capacity(length),
            collision_lines: CollisionLines::new(Vec::with_capacity(length)),
            alive: true,
        }
    }

    fn control(&mut self, camera: &Camera) {
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.render_lines.clear();
        self.render_lines.extend(
            self.model_lines
                .iter()
                .map(|line| line.transformed(transform)),
        );

        self.collision_lines
            .update(transform, self.model_lines.iter().map(|line| line.line));

        if self.pos.x < camera.pos.x - 16.0 {
            self.kill();
        }
    }

    /// Draw the turret to the given line renderer.
    fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.render_lines.iter());
    }
}

struct Shot {
    pos: Vector,
    angle: f32,
    velocity: Vector,
    model_lines: Vec<TintedLine>,
    render_lines: Vec<TintedLine>,
    collision_lines: CollisionLines,
    alive: bool,
}

killable!(Shot);

impl Shot {
    fn new(pos: Vector, angle: f32) -> Self {
        let lines = vec![
            TintedLine::new((-4, 4), (4, 4), Color::GREEN),
            TintedLine::new((4, 4), (0, -4), Color::GREEN),
            TintedLine::new((0, -4), (-4, 4), Color::GREEN),
            TintedLine::new((0, 0), (0, -8), Color::GREEN),
        ];
        let length = lines.len();
        Shot {
            pos,
            angle,
            velocity: Transform::rotate(angle) * Vector::new(0.0, -8.0),
            model_lines: lines,
            render_lines: Vec::with_capacity(length),
            collision_lines: CollisionLines::new(Vec::with_capacity(length)),
            alive: true,
        }
    }

    fn control(&mut self, camera: &Camera) {
        self.pos += self.velocity;

        let transform = Transform::translate(camera.pos + self.pos) * Transform::rotate(self.angle);
        self.render_lines.clear();
        self.render_lines.extend(
            self.model_lines
                .iter()
                .map(|line| line.transformed(transform)),
        );

        self.collision_lines
            .update(transform, self.model_lines.iter().map(|line| line.line));
    }

    /// Draw the shot to the given line renderer.
    fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.render_lines.iter());
    }
}

struct Player {
    pos: Vector,
    angle: f32,
    model_lines: Vec<TintedLine>,
    render_lines: Vec<TintedLine>,
    collision_lines: CollisionLines,
    alive: bool,
}

killable!(Player);

impl Player {
    fn new(pos: Vector, angle: f32) -> Self {
        let lines = vec![
            TintedLine::new((-16, 16), (16, 16), Color::GREEN),
            TintedLine::new((16, 16), (0, -16), Color::GREEN),
            TintedLine::new((0, -16), (-16, 16), Color::GREEN),
        ];
        let length = lines.len();
        Player {
            pos,
            angle,
            model_lines: lines,
            render_lines: Vec::with_capacity(length),
            collision_lines: CollisionLines::new(Vec::with_capacity(length)),
            alive: true,
        }
    }

    fn control(&mut self, camera: &Camera, dx: f32, dy: f32, rotate_by: f32) {
        if !self.alive {
            return;
        }

        // Update the position.
        if dx != 0.0 || dy != 0.0 {
            let movement = Vector::new(dx * 2.0, dy * 2.0);
            self.pos += movement;
        }

        // Update the rotation.
        if rotate_by != 0.0 {
            self.angle += rotate_by * 4.0;
        }

        // Update the transformed model from the original model.
        let transform = Transform::translate(camera.pos + self.pos) * Transform::rotate(self.angle);
        self.render_lines.clear();
        self.render_lines.extend(
            self.model_lines
                .iter()
                .map(|line| line.transformed(transform)),
        );

        self.collision_lines
            .update(transform, self.model_lines.iter().map(|line| line.line));
    }

    /// Draw the player's ship to the given line renderer.
    fn draw(&self, line_renderer: &mut LineRenderer) {
        if self.alive {
            line_renderer.add_lines(self.render_lines.iter());
        }
    }
}

const LANDSCAPE_MIN_Y: f32 = VIRTUAL_HEIGHT as f32 / 4.0;
const LANDSCAPE_MAX_Y: f32 = VIRTUAL_HEIGHT as f32 - 8.0;
const LANDSCAPE_MAX_DY: f32 = 80.0;
const LANDSCAPE_STEP: f32 = 16.0;

struct Landscape {
    render_lines: Vec<TintedLine>,
    collision_lines: CollisionLines,
    rng: ThreadRng,
    want_turret: bool,
}

impl Landscape {
    fn new() -> Self {
        let mut render_lines = Vec::new();
        let mut collision_lines = Vec::new();
        let mut last_point = Vector::new(0.0, 15 * VIRTUAL_HEIGHT / 16);
        let mut x = 0.0;
        while x <= VIRTUAL_WIDTH as f32 + LANDSCAPE_STEP {
            let next_point = Vector::new(x, last_point.y);
            let line = Line::new(last_point, next_point);
            render_lines.push(TintedLine::new(line.a, line.b, Color::GREEN));
            collision_lines.push(line);
            last_point = next_point;
            x += LANDSCAPE_STEP;
        }
        Landscape {
            render_lines,
            collision_lines: CollisionLines::new(collision_lines),
            rng: rand::thread_rng(),
            want_turret: false,
        }
    }

    fn update(&mut self, camera: &Camera) {
        // We need to add a new line to our landscape if the rightmost point of the rightmost line
        // is about to become visible.
        self.want_turret = false;
        let b = self.render_lines[self.render_lines.len() - 1].line.b;
        if b.x < LANDSCAPE_STEP + VIRTUAL_WIDTH as f32 + camera.pos.x {
            let new_y = if self.rng.gen_range(0, 100) >= 25 {
                let mut new_y = b.y + self.rng.gen_range(-LANDSCAPE_MAX_DY, LANDSCAPE_MAX_DY);
                while new_y > LANDSCAPE_MAX_Y || new_y < LANDSCAPE_MIN_Y {
                    new_y = b.y + self.rng.gen_range(-64.0, 64.0);
                }
                new_y
            } else {
                b.y
            };
            let next_point = Vector::new(b.x + LANDSCAPE_STEP, new_y);
            self.render_lines
                .push(TintedLine::new(b, next_point, Color::GREEN));

            // Randomly add a turret if the conditions are right.
            self.want_turret = self.rng.gen_range(0, 100) >= 50 && b.distance(next_point) > 50.0;
        }

        // We need to remove the leftmost line from the landscape if it is no longer visible.
        let a = &self.render_lines[0].line.b;
        if a.x < camera.pos.x {
            self.render_lines.remove(0);
        }

        self.collision_lines.update(
            Transform::IDENTITY,
            self.render_lines.iter().map(|line| line.line),
        );
    }

    /// Draw the landscape to the given line renderer.
    fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.render_lines.iter());
    }
}

pub struct Playing {
    camera: Camera,
    line_renderer: LineRenderer,
    player: Player,
    landscape: Landscape,
    shots: Vec<Shot>,
    turrets: Vec<Turret>,
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

    /// Collide the player's shots with the landscape and check for them going out of bounds.
    fn collide_shots(&mut self) {
        let playfield = Rectangle::new(
            (-16.0, -16.0),
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
            shot.alive = shot.alive && playfield.contains(shot.pos);
        }
    }

    /// Collide the player with the landscape.
    fn collide_player(&mut self) {
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
            self.camera.pos = self.camera.pos.translate((4, 0));
            if fire {
                let shot = Shot::new(self.player.pos, self.player.angle);
                self.shots.push(shot);
            }

            self.player.control(&self.camera, dx, dy, d_theta);
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
            }

            for shot in &mut self.shots {
                shot.control(&self.camera);
            }

            self.collide_player();
            self.collide_shots();
            self.shots.reap();
            self.turrets.reap();
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
        self.line_renderer.render(window);

        Ok(())
    }
}
