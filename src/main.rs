// Play around with Quicksilver with a view to rewriting the Jessica Engine in Rust.

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32 = 640;
const VIRTUAL_WIDTH: u32 = 240;
const VIRTUAL_HEIGHT: u32 = 160;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Img, Color, Image, ImageScaleStrategy, View},
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

use gilrs::{Button, GamepadId, Gilrs};

fn load_tiles(filename: String, tile_size: i32) -> Asset<Vec<Image>> {
    Asset::new(Image::load(filename).map(move |image| {
        let area = image.area();
        let width_in_tiles = area.width() as i32 / tile_size;
        let height_in_tiles = area.height() as i32 / tile_size;
        let mut images: Vec<Image> = Vec::new();
        for y in 0..height_in_tiles {
            for x in 0..width_in_tiles {
                images.push(image.subimage(Rectangle::new(
                    (x * tile_size, y * tile_size),
                    (tile_size, tile_size),
                )));
            }
        }
        images
    }))
}

enum Action {
    Quit,                           // Stop the entire state machine (or game).
    Continue,                       // Continue in the current state.
    Transition(Box<dyn GameState>), // Switch to the new state.
}

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
    tiles: Asset<Vec<Image>>,
}

impl Loading {
    fn new() -> Result<Self> {
        Ok(Self {
            tiles: load_tiles("sprite_tiles.png".to_string(), 8),
        })
    }
}

impl GameState for Loading {
    fn update(&mut self, _window: &mut Window) -> Result<Action> {
        let mut tiles: Vec<Image> = Vec::new();
        self.tiles.execute(|images| {
            tiles.append(images);
            Ok(())
        })?;
        let result = if tiles.is_empty() {
            Continue
        } else {
            Transition(Box::new(Playing::new(tiles)?))
        };
        result.into()
    }
}

struct Playing {
    tiles: Vec<Image>,
    pos: Vector,
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
}

impl Playing {
    fn new(tiles: Vec<Image>) -> Result<Self> {
        Ok(Self {
            tiles: tiles,
            pos: Vector::ZERO,
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

        // Use GilRs directly, because Quicksilver doesn't see some of the buttons.

        // Examine new gamepad events.
        while let Some(event) = self.gilrs.next_event() {
            if self.active_gamepad.is_none() {
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

        let dx = match (left_pressed, right_pressed) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        let dy = match (up_pressed, down_pressed) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };

        if dx != 0 || dy != 0 {
            let movement = Vector::new(dx, dy);
            self.pos += movement.normalize();
        }

        let result = if quit { Quit } else { Continue };
        result.into()
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let (origin_x, origin_y) = (self.pos.x as i32, self.pos.y as i32);

        // Draw all of the tiles, indvidually.
        let mut images = self.tiles.iter();
        for y in 0..16 {
            for x in 0..16 {
                if let Some(image) = images.next() {
                    window.draw(
                        &image
                            .area()
                            .with_center((origin_x + x * 9, origin_y + y * 9)),
                        Img(&image),
                    );
                }
            }
        }
        Ok(())
    }
}

struct Game {
    game_state: Box<dyn GameState>,
}

impl Game {
    fn use_retro_view(window: &mut Window) {
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
        Game::use_retro_view(window);
        window.clear(Color::BLACK)?;
        self.game_state.draw(window)
    }
}

fn main() {
    let fps = 60.0;
    let update_rate_ms = 1000.0 / fps;
    let settings = Settings {
        scale: ImageScaleStrategy::Pixelate,
        update_rate: update_rate_ms,
        ..Settings::default()
    };
    run::<Game>(
        "Quicksilver hack",
        Vector::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        settings,
    );
}
