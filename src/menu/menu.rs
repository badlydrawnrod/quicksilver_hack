use crate::{
    constants::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH},
    font::{text_to_model, VectorFont},
    line_renderer::LineRenderer,
    menu::input::Input,
    playing::Playing,
    Action,
    Action::{Continue, Quit, Transition},
    GameState,
};

use quicksilver::{
    geom::{Transform, Vector},
    graphics::{BlendMode, Image},
    lifecycle::Window,
    Result,
};

use std::collections::HashMap;

pub struct Menu {
    assets: HashMap<String, Image>,
    input: Input,
    line_renderer: LineRenderer,
    font: VectorFont,
    ticks: u32,
    high_score: i32,
    last_score: Option<i32>,
}

impl Menu {
    pub fn new(
        assets: HashMap<String, Image>,
        high_score: i32,
        last_score: Option<i32>,
    ) -> Result<Self> {
        let line_image = assets["line"].clone();
        Ok(Self {
            assets: assets,
            input: Input::new()?,
            line_renderer: LineRenderer::new(line_image),
            font: VectorFont::new(),
            ticks: 0,
            high_score: high_score,
            last_score: last_score,
        })
    }
}

impl GameState for Menu {
    fn update(&mut self, window: &mut Window) -> Result<Action> {
        let (quit, start) = self.input.poll(window);

        if quit {
            return Quit.into();
        }

        if start {
            let result = Transition(Box::new(Playing::new(
                self.assets.clone(),
                self.high_score,
            )?));
            return result.into();
        }

        self.ticks += 1;

        Continue.into()
    }

    fn draw(&mut self, window: &mut Window, _alpha: f64) -> Result<()> {
        window.set_blend_mode(BlendMode::Additive)?;
        self.line_renderer.clear();

        let title = "A PURE RUST VECTOR GAME";
        let text_model = text_to_model(&self.font, title);
        self.line_renderer.add_model(
            &text_model,
            Transform::translate(Vector::new(VIRTUAL_WIDTH / 2 - 190, 1 * VIRTUAL_HEIGHT / 8)),
        );

        if let Some(last_score) = self.last_score {
            let score = format!("LAST SCORE:{:06}", last_score);
            let score_model = text_to_model(&self.font, score.as_str());
            self.line_renderer.add_model(
                &score_model,
                Transform::translate(Vector::new(
                    VIRTUAL_WIDTH as f32 / 2.0 - 140.0,
                    60.0 + VIRTUAL_HEIGHT as f32 / 2.0,
                )),
            );
        }

        let high_score = format!("HIGH SCORE:{:06}", self.high_score);
        let high_score_model = text_to_model(&self.font, high_score.as_str());
        self.line_renderer.add_model(
            &high_score_model,
            Transform::translate(Vector::new(
                VIRTUAL_WIDTH as f32 / 2.0 - 140.0,
                VIRTUAL_HEIGHT as f32 / 2.0,
            )),
        );

        if self.ticks % 50 < 30 {
            let press_start = "PRESS START";
            let text_model = text_to_model(&self.font, press_start);
            self.line_renderer.add_model(
                &text_model,
                Transform::translate(Vector::new(VIRTUAL_WIDTH / 2 - 70, 7 * VIRTUAL_HEIGHT / 8)),
            );
        }

        self.line_renderer.render(window);

        Ok(())
    }
}
