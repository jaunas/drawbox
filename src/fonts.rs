use lazy_static::lazy_static;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::collections::HashMap;

const FONT_PATH: &str = "./resources/Roboto-Light.ttf";

lazy_static! {
    static ref TTF_CONTEXT: Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

pub struct Fonts(HashMap<u16, Font<'static, 'static>>);

impl Fonts {
    pub fn new() -> Fonts {
        let mut hash_map = HashMap::new();
        hash_map.insert(14, TTF_CONTEXT.load_font(FONT_PATH, 14).unwrap());

        Fonts(hash_map)
    }

    pub fn get(&self, size: u16) -> Option<&Font> {
        self.0.get(&size)
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Self::new()
    }
}
