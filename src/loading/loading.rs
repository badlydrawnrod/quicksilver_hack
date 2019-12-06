use crate::{
    menu::Menu,
    Action,
    Action::{Continue, Transition},
    GameState,
};

use quicksilver::{
    graphics::Image,
    lifecycle::{Asset, Window},
    sound::Sound,
    Result,
};

use std::collections::HashMap;
use std::rc::Rc;

pub struct Loading {
    loading_images: HashMap<String, Asset<Image>>,
    loaded_images: HashMap<String, Image>,
    loading_sounds: HashMap<String, Asset<Sound>>,
    loaded_sounds: HashMap<String, Sound>,
    high_score: i32,
}

pub struct GameAssets {
    pub images: HashMap<String, Image>,
    pub sounds: HashMap<String, Sound>,
}

impl Loading {
    pub fn new() -> Result<Self> {
        let mut images: HashMap<String, Asset<Image>> = HashMap::new();
        for name in vec!["line", "particle"] {
            images.insert(
                name.to_string(),
                Asset::new(Image::load(name.to_string() + ".png")),
            );
        }
        let mut sounds: HashMap<String, Asset<Sound>> = HashMap::new();
        for name in vec!["hit01", "hit02"] {
            sounds.insert(
                name.to_string(),
                Asset::new(Sound::load(name.to_string() + ".wav")),
            );
        }
        // It may seem strange to put the initial high score here, but ultimately it'll probably be
        // loaded much like other assets are.
        Ok(Self {
            loading_images: images,
            loaded_images: HashMap::new(),
            loading_sounds: sounds,
            loaded_sounds: HashMap::new(),
            high_score: 5000,
        })
    }
}

impl GameState for Loading {
    fn update(&mut self, _window: &mut Window) -> Result<Action> {
        let loaded = &mut self.loaded_images;
        for (name, image) in &mut self.loading_images {
            // If the image isn't loaded then make an attempt to load it.
            if !loaded.contains_key(name) {
                image.execute(|image| {
                    // The image has been loaded successfully, so save it.
                    loaded.insert(name.to_string(), image.to_owned());
                    Ok(())
                })?;
            }
        }

        let loaded = &mut self.loaded_sounds;
        for (name, sound) in &mut self.loading_sounds {
            // If the sound isn't loaded then make an attempt to load it.
            if !loaded.contains_key(name) {
                sound.execute(|sound| {
                    // The sound has been loaded successfully, so save it.
                    loaded.insert(name.to_string(), sound.to_owned());
                    Ok(())
                })?;
            }
        }

        let all_loaded = self.loaded_images.len() == self.loading_images.len()
            && self.loaded_sounds.len() == self.loading_sounds.len();

        let result = if all_loaded {
            let assets = GameAssets {
                images: self.loaded_images.clone(),
                sounds: self.loaded_sounds.clone(),
            };
            // We successfully loaded everything.
            Transition(Box::new(Menu::new(Rc::new(assets), self.high_score, None)?))
        } else {
            // Still waiting.
            Continue
        };
        result.into()
    }
}
