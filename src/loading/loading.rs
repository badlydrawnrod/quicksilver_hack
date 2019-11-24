use crate::{
    menu::Menu,
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
        for name in vec!["line", "particle"] {
            loading.insert(
                name.to_string(),
                Asset::new(Image::load(name.to_string() + ".png")),
            );
        }
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
            // If the image isn't loaded then make an attempt to load it.
            if !loaded.contains_key(name) {
                image.execute(|image| {
                    // The image has been loaded successfully, so save it.
                    loaded.insert(name.to_string(), image.to_owned());
                    Ok(())
                })?;
            }
        }

        let result = if self.loaded.len() == self.loading.len() {
            // We successfully loaded everything.
            Transition(Box::new(Menu::new(self.loaded.clone())?))
        } else {
            // Still waiting.
            Continue
        };
        result.into()
    }
}
