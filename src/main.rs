// Play around with Quicksilver... trying to draw "glowy" lines.

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const VIRTUAL_WIDTH: u32 = WINDOW_WIDTH;
const VIRTUAL_HEIGHT: u32 = WINDOW_HEIGHT;

use quicksilver::{
    geom::{Line, Rectangle, Vector},
    graphics::{Background::Blended, Color, Image, ImageScaleStrategy, View},
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

use gilrs::{Button, GamepadId, Gilrs};

use rand::{prelude::*, Rng};

enum Action {
    Quit,                           // Stop the entire state machine (or game).
    Continue,                       // Continue in the current state.
    Transition(Box<dyn GameState>), // Switch to the new state.
}

use quicksilver::geom::Transform;
use quicksilver::graphics::BlendMode;
use quicksilver::prelude::Shape;
use Action::{Continue, Quit, Transition};

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
struct MyLine {
    start: Vector,
    end: Vector,
    colour: Color,
}

impl MyLine {
    fn new(start: impl Into<Vector>, end: impl Into<Vector>, colour: impl Into<Color>) -> Self {
        MyLine {
            start: start.into(),
            end: end.into(),
            colour: colour.into(),
        }
    }
}

// TODO: surely this should just be a mesh, or at least wrap one.
struct LineRenderer {
    image: Image,
    lines: Vec<MyLine>,
}

impl LineRenderer {
    fn new(image: Image) -> Self {
        LineRenderer {
            image,
            lines: Vec::with_capacity(256),
        }
    }

    fn clear(&mut self) {
        self.lines.clear();
    }

    fn extend<'a>(&mut self, transform: Transform, more_lines: impl Iterator<Item = &'a MyLine>) {
        let transformed_lines = more_lines
            .map(|line| MyLine::new(transform * line.start, transform * line.end, line.colour));
        self.lines.extend(transformed_lines);
    }

    fn render(&self, window: &mut Window) {
        for line in &self.lines {
            window.draw(
                &Line::new(line.start, line.end).with_thickness(16.0),
                Blended(&self.image, line.colour.with_alpha(0.75)),
            );
        }
    }
}

struct Player {
    pos: Vector,
    angle: f32,
    lines: Vec<MyLine>,
}

impl Player {
    fn new(pos: Vector, angle: f32) -> Self {
        let lines = vec![
            MyLine::new((-16, 16), (16, 16), Color::CYAN),
            MyLine::new((16, 16), (0, -16), Color::GREEN),
            MyLine::new((0, -16), (-16, 16), Color::GREEN),
        ];
        Player { pos, angle, lines }
    }

    fn move_by(&mut self, dv: Vector) {
        self.pos += dv;
    }

    fn draw(&self, line_renderer: &mut LineRenderer) {
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        line_renderer.extend(transform, self.lines.iter());
    }
}

const LANDSCAPE_MIN_Y: f32 = VIRTUAL_HEIGHT as f32 / 4.0;
const LANDSCAPE_MAX_Y: f32 = VIRTUAL_HEIGHT as f32 - 8.0;
const LANDSCAPE_MAX_DY: f32 = 80.0;
const LANDSCAPE_STEP: f32 = 16.0;

struct Landscape {
    landscape: Vec<MyLine>,
    rng: ThreadRng,
}

impl Landscape {
    fn new() -> Self {
        let mut landscape = Vec::new();
        let mut last_point = Vector::new(0.0, 15 * WINDOW_HEIGHT / 16);
        let mut x = 0.0;
        while x <= WINDOW_WIDTH as f32 + LANDSCAPE_STEP {
            let next_point = Vector::new(x, last_point.y);
            landscape.push(MyLine::new(last_point, next_point, Color::GREEN));
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
            line.start = line.start.translate((-4.0, 0.0));
            line.end = line.end.translate((-4.0, 0.0));
            //            *line = line.translate((-4.0, 0.0));
        }

        // We need to add a new line to our landscape if the rightmost point of the rightmost line
        // is about to become visible.
        let b = self.landscape[self.landscape.len() - 1].end;
        if b.x < LANDSCAPE_STEP + WINDOW_WIDTH as f32 {
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
                .push(MyLine::new(b, next_point, Color::GREEN));
        }

        // We need to remove the leftmost line from the landscape if it is no longer visible.
        let a = &self.landscape[0].end;
        if a.x < 0.0 {
            self.landscape.remove(0);
        }
    }

    fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.extend(Transform::IDENTITY, self.landscape.iter());
    }
}

struct Playing {
    line_renderer: LineRenderer,
    player: Player,
    landscape: Landscape,
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
            gilrs: Gilrs::new()?,
            active_gamepad: None,
        })
    }
}

impl GameState for Playing {
    fn update(&mut self, window: &mut Window) -> Result<Action> {
        let mut quit = false;
        let mut right_pressed = false;
        let mut left_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut rotate_anticlockwise = false;
        let mut rotate_clockwise = false;

        // Use GilRs directly, because Quicksilver doesn't see some of the buttons.

        // Examine new gamepad events.
        while let Some(event) = self.gilrs.next_event() {
            if self.active_gamepad.is_none() {
                // If we don't have an active gamepad yet, then we do now.
                self.active_gamepad = Some(event.id);
            }
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
        quit = quit || window.keyboard()[Key::Escape].is_down();
        right_pressed = right_pressed || window.keyboard()[Key::Right].is_down();
        left_pressed = left_pressed || window.keyboard()[Key::Left].is_down();
        up_pressed = up_pressed || window.keyboard()[Key::Up].is_down();
        down_pressed = down_pressed || window.keyboard()[Key::Down].is_down();
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

        if dx != 0.0 || dy != 0.0 {
            let movement = Vector::new(dx * 2.0, dy * 2.0);
            self.player.move_by(movement);
        }

        let d_theta = match (rotate_anticlockwise, rotate_clockwise) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
        self.player.angle += d_theta * 4.0;

        self.landscape.update();

        let result = if quit { Quit } else { Continue };
        result.into()
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.set_blend_mode(BlendMode::Additive)?;
        self.line_renderer.clear();
        self.landscape.draw(&mut self.line_renderer);
        self.player.draw(&mut self.line_renderer);
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
