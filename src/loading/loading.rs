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

pub struct Loading {
    line: Asset<Image>,
}

impl Loading {
    pub fn new() -> Result<Self> {
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
