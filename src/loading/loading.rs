use crate::playing::Playing;
use crate::{
    Action,
    Action::{Continue, Transition},
    GameState,
};

use quicksilver::{
    graphics::Image,
    lifecycle::{Asset, Window},
    Result,
};
use std::collections::HashMap;

pub struct Loading {
    line: Asset<Image>,
    particle: Asset<Image>,
    lines: HashMap<String, Image>,
}

impl Loading {
    pub fn new() -> Result<Self> {
        Ok(Self {
            line: Asset::new(Image::load("line.png")),
            particle: Asset::new(Image::load("particle.png")),
            lines: HashMap::new(),
        })
    }
}

impl GameState for Loading {
    fn update(&mut self, _window: &mut Window) -> Result<Action> {
        let lines = &mut self.lines;

        if !lines.contains_key("line") {
            self.line.execute(|image| {
                lines.insert("line".to_string(), image.to_owned());
                Ok(())
            })?;
        }

        if !lines.contains_key("particle") {
            self.particle.execute(|image| {
                lines.insert("particle".to_string(), image.to_owned());
                Ok(())
            })?;
        }

        let result = if self.lines.len() < 2 {
            Continue
        } else {
            Transition(Box::new(Playing::new(self.lines.clone())?))
        };
        result.into()
    }
}
