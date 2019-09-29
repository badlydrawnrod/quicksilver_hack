// Play around with Quicksilver with a view to rewriting the Jessica Engine in Rust.

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32 = 640;
const VIRTUAL_WIDTH: u32 = 240;
const VIRTUAL_HEIGHT: u32 = 160;

use quicksilver::{
    geom::{Circle, Line, Rectangle, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Background::Img, Color, Image, ImageScaleStrategy, View},
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

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

enum UpdateStatus {
    Quit,
    Continue,
}

struct Playing {
    tiles: Asset<Vec<Image>>,
    x: i32,
    y: i32,
}

impl Playing {
    fn new() -> Result<Self> {
        Ok(Self {
            tiles: load_tiles("sprite_tiles.png".to_string(), 8),
            x: 0,
            y: 0,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<UpdateStatus> {
        let right_down = window.keyboard()[Key::Right].is_down();
        let left_down = window.keyboard()[Key::Left].is_down();
        let up_down = window.keyboard()[Key::Up].is_down();
        let down_down = window.keyboard()[Key::Down].is_down();

        let dx = match (left_down, right_down) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };
        let dy = match (up_down, down_down) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };

        // TODO: don't speed up for diagonals.
        self.x += dx;
        self.y += dy;

        if window.keyboard()[Key::Escape].is_down() {
            Ok(UpdateStatus::Quit)
        } else {
            Ok(UpdateStatus::Continue)
        }
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let origin_x = self.x;
        let origin_y = self.y;

        // Draw all of the tiles, indvidually.
        self.tiles.execute(|images| {
            let mut images = images.iter();
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
        })
    }
}

struct Game {
    playing: Playing,
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
            playing: Playing::new()?,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        match self.playing.update(window) {
            Ok(UpdateStatus::Quit) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Game::use_retro_view(window);
        window.clear(Color::BLACK)?;
        self.playing.draw(window)?;
        Ok(())
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

// TODO: compare with these other quicksilver games...
//      https://github.com/WushuWorks/I-am-the-Elder-God/blob/master/src/game_logic/main_state.rs
//      https://github.com/rickyhan/dyn-grammar/blob/master/src/main.rs
