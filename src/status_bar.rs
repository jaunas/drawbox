use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    ttf::Font,
    video::Window,
};

const HEIGHT: u32 = 20;

pub struct StatusBar {
    last_mouse_position: (i32, i32),
}

impl StatusBar {
    pub fn new() -> Self {
        StatusBar {
            last_mouse_position: (0, 0),
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, font: &Font, position: Option<(i32, i32)>) {
        let (width, height) = canvas.window().size();
        canvas.set_draw_color(Color::GRAY);
        canvas
            .fill_rect(Rect::new(0, (height - HEIGHT) as i32, width, HEIGHT))
            .unwrap();

        let position = position.unwrap_or(self.last_mouse_position);
        self.last_mouse_position = position;
        let text = format!("Mouse position: ({}, {})", position.0, position.1);

        let surface = font
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
