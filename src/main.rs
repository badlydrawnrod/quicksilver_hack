// Play around with Quicksilver... trying to draw "glowy" lines.

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const VIRTUAL_WIDTH: u32 = WINDOW_WIDTH;
const VIRTUAL_HEIGHT: u32 = WINDOW_HEIGHT;
const LINE_THICKNESS: f32 = 8.0;

use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, BlendMode, Color, Drawable, Image, ImageScaleStrategy, Mesh, View,
    },
    input::{ButtonState, Key},
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

use gilrs::{Button, EventType, GamepadId, Gilrs};

use rand::{prelude::*, Rng};

use std::borrow::BorrowMut;
use Action::{Continue, Quit, Transition};

enum Action {
    Quit,
    // Stop the entire state machine (or game).
    Continue,
    // Continue in the current state.
    Transition(Box<dyn GameState>), // Switch to the new state.
}

impl From<Action> for Result<Action> {
    fn from(r: Action) -> Self {
        Ok(r)
    }
}

trait GameState {
    fn update(&mut self, _window: &mut Window) -> Result<Action> {
        Continue.into()
    }

    fn draw(&mut self, _window: &mut Window) -> Result<()> {
        Ok(())
    }
}

struct Loading {
    line: Asset<Image>,
}

impl Loading {
    fn new() -> Result<Self> {
        Ok(Self {
            line: Asset::new(Image::load("line.png")),
        })
    }
}

impl GameState for Loading {
    fn update(&mut self, _window: &mut Window) -> Result<Action> {
        let mut lines: Vec<Image> = Vec::new();
        self.line.execute(|image| {
            lines.push(image.to_owned());
            Ok(())
        })?;
        let result = if lines.is_empty() {
            Continue
        } else {
            Transition(Box::new(Playing::new(lines)?))
        };
        result.into()
    }
}

#[derive(Copy, Clone)]
struct TintedLine {
    line: Line,
    colour: Color,
}

impl TintedLine {
    fn new(start: impl Into<Vector>, end: impl Into<Vector>, colour: impl Into<Color>) -> Self {
        TintedLine {
            line: Line::new(start, end),
            colour: colour.into(),
        }
    }

    fn transformed(self, transform: Transform) -> Self {
        TintedLine::new(
            transform * self.line.a,
            transform * self.line.b,
            self.colour,
        )
    }
}

struct LineRenderer {
    image: Image,
    mesh: Mesh,
}

impl LineRenderer {
    fn new(image: Image) -> Self {
        LineRenderer {
            image,
            mesh: Mesh::new(),
        }
    }

    fn clear(&mut self) {
        self.mesh.clear();
    }

    fn add_lines<'a>(&mut self, lines: impl Iterator<Item = &'a TintedLine>) {
        let identity = Transform::IDENTITY;
        for tinted_line in lines {
            let thick_line = tinted_line.line.with_thickness(LINE_THICKNESS);
            thick_line.draw(
                self.mesh.borrow_mut(),
                Blended(&self.image, tinted_line.colour),
                identity,
                0.0,
            );
        }
    }

    fn render(&self, window: &mut Window) {
        window.mesh().extend(&self.mesh);
    }
}

struct Shot {
    pos: Vector,
    angle: f32,
    velocity: Vector,
    model_lines: Vec<TintedLine>,
    transformed_lines: Vec<TintedLine>,
    alive: bool,
}

impl Shot {
    fn new(pos: Vector, angle: f32) -> Self {
        let lines = vec![
            TintedLine::new((-4, 4), (4, 4), Color::RED),
            TintedLine::new((4, 4), (0, -4), Color::RED),
            TintedLine::new((0, -4), (-4, 4), Color::RED),
            TintedLine::new((0, 0), (0, -8), Color::YELLOW),
        ];
        let length = lines.len();
        Shot {
            pos,
            angle,
            velocity: Transform::rotate(angle) * Vector::new(0.0, -8.0),
            model_lines: lines,
            transformed_lines: Vec::with_capacity(length),
            alive: true,
        }
    }

    fn control(&mut self) {
        self.pos += self.velocity;

        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.transformed_lines.clear();
        self.transformed_lines.extend(
            self.model_lines
                .iter()
                .map(|line| line.transformed(transform)),
        );
    }

    fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.transformed_lines.iter());
    }
}

struct Player {
    pos: Vector,
    angle: f32,
    model_lines: Vec<TintedLine>,
    transformed_lines: Vec<TintedLine>,
    alive: bool,
}

impl Player {
    fn new(pos: Vector, angle: f32) -> Self {
        let lines = vec![
            TintedLine::new((-16, 16), (16, 16), Color::CYAN),
            TintedLine::new((16, 16), (0, -16), Color::GREEN),
            TintedLine::new((0, -16), (-16, 16), Color::GREEN),
        ];
        let length = lines.len();
        Player {
            pos,
            angle,
            model_lines: lines,
            transformed_lines: Vec::with_capacity(length),
            alive: true,
        }
    }

    fn control(&mut self, dx: f32, dy: f32, d_theta: f32) {
        if !self.alive {
            return;
        }

        // Update the position.
        if dx != 0.0 || dy != 0.0 {
            let movement = Vector::new(dx * 2.0, dy * 2.0);
            self.pos += movement;
        }

        // Update the rotation.
        if d_theta != 0.0 {
            self.angle += d_theta * 4.0;
        }

        // Update the transformed model from the original model.
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.transformed_lines.clear();
        self.transformed_lines.extend(
            self.model_lines
                .iter()
                .map(|line| line.transformed(transform)),
        );
    }

    fn draw(&self, line_renderer: &mut LineRenderer) {
        if self.alive {
            line_renderer.add_lines(self.transformed_lines.iter());
        }
    }
}

const LANDSCAPE_MIN_Y: f32 = VIRTUAL_HEIGHT as f32 / 4.0;
const LANDSCAPE_MAX_Y: f32 = VIRTUAL_HEIGHT as f32 - 8.0;
const LANDSCAPE_MAX_DY: f32 = 80.0;
const LANDSCAPE_STEP: f32 = 16.0;

struct Landscape {
    landscape: Vec<TintedLine>,
    rng: ThreadRng,
}

impl Landscape {
    fn new() -> Self {
        let mut landscape = Vec::new();
        let mut last_point = Vector::new(0.0, 15 * VIRTUAL_HEIGHT / 16);
        let mut x = 0.0;
        while x <= VIRTUAL_WIDTH as f32 + LANDSCAPE_STEP {
            let next_point = Vector::new(x, last_point.y);
            landscape.push(TintedLine::new(last_point, next_point, Color::GREEN));
            last_point = next_point;
            x += LANDSCAPE_STEP;
        }
        Landscape {
            landscape,
            rng: rand::thread_rng(),
        }
    }

    fn update(&mut self) {
        for line in self.landscape.iter_mut() {
            line.line = line.line.translate((-4.0, 0.0));
        }

        // We need to add a new line to our landscape if the rightmost point of the rightmost line
        // is about to become visible.
        let b = self.landscape[self.landscape.len() - 1].line.b;
        if b.x < LANDSCAPE_STEP + VIRTUAL_WIDTH as f32 {
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
            self.landscape
                .push(TintedLine::new(b, next_point, Color::GREEN));
        }

        // We need to remove the leftmost line from the landscape if it is no longer visible.
        let a = &self.landscape[0].line.b;
        if a.x < 0.0 {
            self.landscape.remove(0);
        }
    }

    fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.landscape.iter());
    }
}

struct Playing {
    line_renderer: LineRenderer,
    player: Player,
    landscape: Landscape,
    shots: Vec<Shot>,
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
}

impl Playing {
    fn new(line_images: Vec<Image>) -> Result<Self> {
        let mut landscape = Vec::new();
        let mut last_point = Vector::new(0.0, 15 * WINDOW_HEIGHT / 16);
        for x in (0..WINDOW_WIDTH + 32).step_by(32) {
            let next_point = Vector::new(x, last_point.y);
            landscape.push(Line::new(last_point, next_point));
            last_point = next_point;
        }
        Ok(Self {
            line_renderer: LineRenderer::new(line_images[0].clone()),
            player: Player::new(Vector::new(VIRTUAL_WIDTH / 4, VIRTUAL_HEIGHT / 4), 90.0),
            landscape: Landscape::new(),
            shots: Vec::new(),
            gilrs: Gilrs::new()?,
            active_gamepad: None,
        })
    }

    fn poll_inputs(&mut self, window: &mut Window) -> (bool, bool, f32, f32, f32) {
        let mut quit = false;
        let mut right_pressed = false;
        let mut left_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut rotate_anticlockwise = false;
        let mut rotate_clockwise = false;
        let mut fire: bool = false;

        // Use GilRs directly, because Quicksilver doesn't see some of the buttons.
        // Examine new gamepad events.
        while let Some(event) = self.gilrs.next_event() {
            if self.active_gamepad.is_none() {
                // If we don't have an active gamepad yet, then we do now.
                self.active_gamepad = Some(event.id);
            }
            match event.event {
                EventType::ButtonPressed(Button::South, _) => fire = true,
                EventType::ButtonReleased(Button::Start, _) => quit = true,
                _ => (),
            };
        }

        // Check the player's gamepad.
        if let Some(id) = self.active_gamepad {
            let gamepad = self.gilrs.gamepad(id);
            quit = quit || gamepad.is_pressed(Button::Select);
            left_pressed = left_pressed || gamepad.is_pressed(Button::DPadLeft);
            right_pressed = right_pressed || gamepad.is_pressed(Button::DPadRight);
            up_pressed = up_pressed || gamepad.is_pressed(Button::DPadUp);
            down_pressed = down_pressed || gamepad.is_pressed(Button::DPadDown);
        }

        // Check the keyboard.
        quit = quit || window.keyboard()[Key::Escape] == ButtonState::Released;
        right_pressed = right_pressed || window.keyboard()[Key::Right].is_down();
        left_pressed = left_pressed || window.keyboard()[Key::Left].is_down();
        up_pressed = up_pressed || window.keyboard()[Key::Up].is_down();
        down_pressed = down_pressed || window.keyboard()[Key::Down].is_down();
        rotate_anticlockwise = rotate_anticlockwise || window.keyboard()[Key::Q].is_down();
        rotate_clockwise = rotate_clockwise || window.keyboard()[Key::E].is_down();
        fire = fire || window.keyboard()[Key::Space] == ButtonState::Pressed;

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
        let d_theta = match (rotate_anticlockwise, rotate_clockwise) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        (quit, fire, dx, dy, d_theta)
    }

    fn reap_dead_shots(&mut self) {
        let mut i = 0;
        while i != self.shots.len() {
            if !self.shots[i].alive {
                self.shots.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn collide_shots(&mut self) {
        let playfield = Rectangle::new(
            (-16.0, -16.0),
            (VIRTUAL_WIDTH as f32 + 32.0, VIRTUAL_HEIGHT as f32 + 32.0),
        );

        // Collide the player's shots with the landscape and check for them going out of bounds.
        for shot in &mut self.shots {
            // Collide the shot with the landscape.
            'dead: for line_a in &shot.transformed_lines {
                for line_b in &self.landscape.landscape {
                    if line_a.line.intersects(&line_b.line) {
                        shot.alive = false;
                        break 'dead;
                    }
                }
            }

            // Check for the shot going out of bounds.
            shot.alive = shot.alive && playfield.contains(shot.pos);
        }
    }

    fn collide_player(&mut self) {
        // Collide the player with the landscape.
        'kaboom: for line_a in &self.landscape.landscape {
            for line_b in &self.player.transformed_lines {
                if line_a.line.intersects(&line_b.line) {
                    self.player.alive = false;
                    break 'kaboom;
                }
            }
        }
    }
}

impl GameState for Playing {
    fn update(&mut self, window: &mut Window) -> Result<Action> {
        let (quit, fire, dx, dy, d_theta) = self.poll_inputs(window);

        if self.player.alive {
            if fire {
                let shot = Shot::new(self.player.pos, self.player.angle);
                self.shots.push(shot);
            }

            self.player.control(dx, dy, d_theta);
            self.landscape.update();
            for shot in &mut self.shots {
                shot.control();
            }

            self.collide_player();
            self.collide_shots();
            self.reap_dead_shots();
        }

        let result = if quit { Quit } else { Continue };
        result.into()
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.set_blend_mode(BlendMode::Additive)?;
        self.line_renderer.clear();
        self.landscape.draw(&mut self.line_renderer);
        self.player.draw(&mut self.line_renderer);
        for shot in &self.shots {
            shot.draw(&mut self.line_renderer);
        }
        self.line_renderer.render(window);

        Ok(())
    }
}

struct Game {
    game_state: Box<dyn GameState>,
}

impl Game {
    fn rescale_viewport(window: &mut Window) {
        let view_rect = Rectangle::new(
            Vector::new(0, 0),
            Vector::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
        );
        let view = View::new(view_rect);
        window.set_view(view);
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            game_state: Box::new(Loading::new()?),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        match self.game_state.update(window) {
            Ok(Action::Transition(new_state)) => {
                self.game_state = new_state;
            }
            Ok(Action::Quit) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Game::rescale_viewport(window);
        window.clear(Color::BLACK)?;
        self.game_state.draw(window)
    }
}

fn main() {
    let fixed_update_hz = 60.0;
    let fixed_update_interval_ms = 1000.0 / fixed_update_hz;
    let settings = Settings {
        scale: ImageScaleStrategy::Blur,
        update_rate: fixed_update_interval_ms,
        ..Settings::default()
    };
    run::<Game>(
        "Quicksilver hack/lines",
        Vector::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        settings,
    );
}
