// Play around with Quicksilver... trying to draw "glowy" lines.

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const VIRTUAL_WIDTH: u32 = 1280;
const VIRTUAL_HEIGHT: u32 = 720;

use quicksilver::{
    geom::{Line, Rectangle, Vector},
    graphics::{
        Background::Blended, Color, Image, ImageScaleStrategy,
        View,
    },
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

use gilrs::{Button, GamepadId, Gilrs};

enum Action {
    Quit,                           // Stop the entire state machine (or game).
    Continue,                       // Continue in the current state.
    Transition(Box<dyn GameState>), // Switch to the new state.
}

use quicksilver::geom::Transform;
use quicksilver::graphics::BlendMode;
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

struct Player {
    pos: Vector,
}

impl Player {
    fn move_by(&mut self, dv: Vector) {
        self.pos += dv;
    }
}

struct Playing {
    line_images: Vec<Image>,
    lines: Vec<Line>,
    angle: f32,
    player: Player,
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
}

impl Playing {
    fn new(line_images: Vec<Image>) -> Result<Self> {
        let lines = vec![
            Line::new((-16, 16), (16, 16)),
            Line::new((16, 16), (0, -16)),
            Line::new((0, -16), (-16, 16)),
        ];
        Ok(Self {
            line_images: line_images,
            lines: lines,
            angle: 0.0,
            player: Player {
                pos: Vector::new(VIRTUAL_WIDTH / 2, VIRTUAL_HEIGHT / 8),
            },
            gilrs: Gilrs::new()?,
            active_gamepad: None,
        })
    }

    fn draw_player(&self, window: &mut Window) {
        let image = &self.line_images[0];
        let transform = Transform::translate(self.player.pos) * Transform::rotate(self.angle);
        for line in &self.lines {
            let line = Line::new(transform * line.a, transform * line.b).with_thickness(6.0);
            window.draw(&line, Blended(&image, Color::GREEN.with_alpha(0.75)));
        }
    }

    fn draw_grid(&self, window: &mut Window) {
        //  We can draw lines whose "background" is an image, or even an image blended with a
        // colour as shown here, which is promising for doing glowy lines.
        let image = &self.line_images[0];
        for x in (0..=VIRTUAL_WIDTH).step_by(VIRTUAL_HEIGHT as usize / 8) {
            window.draw(
                &Line::new((x, VIRTUAL_HEIGHT / 2), (x, VIRTUAL_HEIGHT)).with_thickness(6.0),
                Blended(&image, Color::YELLOW.with_alpha(0.75)),
            );
        }
        for y in (VIRTUAL_HEIGHT / 2..=VIRTUAL_HEIGHT).step_by(VIRTUAL_HEIGHT as usize / 8) {
            window.draw(
                &Line::new((0, y), (VIRTUAL_WIDTH, y)).with_thickness(6.0),
                Blended(&image, Color::YELLOW.with_alpha(0.75)),
            );
        }
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
        self.angle += d_theta * 4.0;

        let result = if quit { Quit } else { Continue };
        result.into()
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.set_blend_mode(BlendMode::Additive)?;
        self.draw_grid(window);
        self.draw_player(window);

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
    let fps = 60.0;
    let update_rate_ms = 1000.0 / fps;
    let settings = Settings {
        scale: ImageScaleStrategy::Blur,
        update_rate: update_rate_ms,
        ..Settings::default()
    };
    run::<Game>(
        "Quicksilver hack/lines",
        Vector::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        settings,
    );
}
