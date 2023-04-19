use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    video::Window,
};

use crate::{fonts::Fonts, view::View};

const HEIGHT: u32 = 20;

pub struct StatusBar {
    pub last_mouse_position: (i32, i32),
}

impl StatusBar {
    pub fn new() -> Self {
        StatusBar {
            last_mouse_position: (0, 0),
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl View for StatusBar {
    fn draw(&mut self, canvas: &mut Canvas<Window>, fonts: &Fonts) {
        let (width, height) = canvas.window().size();
        canvas.set_draw_color(Color::GRAY);
        canvas
            .fill_rect(Rect::new(0, (height - HEIGHT) as i32, width, HEIGHT))
            .unwrap();

        let text = format!("Mouse position: {:?}", self.last_mouse_position);

        let surface = fonts
            .get(14)
            .unwrap()
            .render(&text)
            .shaded(Color::WHITE, Color::GRAY)
            .unwrap();

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();
        let TextureQuery {
            width: text_width,
            height: text_height,
            ..
        } = texture.query();

        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(
                    2,
                    (height - HEIGHT + 2) as i32,
                    text_width,
                    text_height,
                )),
            )
            .unwrap();
    }
}
