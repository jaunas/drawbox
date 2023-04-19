use sdl2::{render::Canvas, video::Window};

use crate::fonts::Fonts;

pub trait View {
    fn draw(&mut self, canvas: &mut Canvas<Window>, fonts: &Fonts);
}
