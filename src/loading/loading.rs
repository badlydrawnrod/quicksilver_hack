use crate::{
    playing::Playing,
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
    loading: HashMap<String, Asset<Image>>,
    loaded: HashMap<String, Image>,
}

impl Loading {
    pub fn new() -> Result<Self> {
        let mut loading: HashMap<String, Asset<Image>> = HashMap::new();
        loading.insert("line".to_string(), Asset::new(Image::load("line.png")));
        loading.insert(
            "particle".to_string(),
            Asset::new(Image::load("particle.png")),
        );
        Ok(Self {
            loading: loading,
            loaded: HashMap::new(),
        })
    }
}

impl GameState for Loading {
    fn update(&mut self, _window: &mut Window) -> Result<Action> {
        let loaded = &mut self.loaded;
        for (name, image) in &mut self.loading {
            if !loaded.contains_key(name) {
                image.execute(|image| {
                    loaded.insert(name.to_string(), image.to_owned());
                    Ok(())
                })?;
            }
        }

        let result = if self.loaded.len() < self.loading.len() {
            Continue
        } else {
            Transition(Box::new(Playing::new(self.loaded.clone())?))
        };
        result.into()
    }
}
