// Play around with Quicksilver.
mod collision_lines;
mod constants;
mod font;
mod game_state;
mod line_renderer;
mod loading;
mod playing;
mod transformed;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, ImageScaleStrategy, View},
    lifecycle::{run, Settings, State, Window},
    Result,
};

use constants::*;
use game_state::{Action, GameState};
use loading::Loading;

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
//        update_rate: fixed_update_interval_ms,
//        vsync: true,
        ..Settings::default()
    };
    run::<Game>(
        "Quicksilver hack/lines",
        Vector::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        settings,
    );
}
