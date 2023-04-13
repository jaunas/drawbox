use std::collections::HashMap;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct View {
    pixel_size: u32,
    image_width: u32,
    image_height: u32,
    colors: HashMap<(i32, i32), Color>,
}

impl View {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        Self::new_with_pixel_size(image_width, image_height, 10)
    }

    pub fn new_with_pixel_size(image_width: u32, image_height: u32, pixel_size: u32) -> Self {
        Self {
            pixel_size,
            image_width,
            image_height,
            colors: HashMap::new(),
        }
    }

    fn rect_iter(&self) -> RectIterator {
        RectIterator::new(self.pixel_size, self.image_width, self.image_height)
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for (position, rect) in self.rect_iter() {
            let color = self.colors.get(&position).unwrap_or(&Color::WHITE);
            canvas.set_draw_color(*color);
            canvas.fill_rect(rect).unwrap();
        }
    }

    pub fn translate_position(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        let position = (
            x / (self.pixel_size + 1) as i32,
            y / (self.pixel_size + 1) as i32,
        );

        if x as u32 % (self.pixel_size + 1) == 0
            || y as u32 % (self.pixel_size + 1) == 0
            || position.0 >= self.image_width as i32
            || position.1 >= self.image_height as i32
        {
            None
        } else {
            Some(position)
        }
    }

    pub fn set_color(&mut self, position: (i32, i32), color: Color) {
        self.colors.insert(position, color);
    }
}

struct RectIterator {
    x: i32,
    y: i32,
    pixel_size: u32,
    width: u32,
    height: u32,
}

impl RectIterator {
    pub fn new(pixel_size: u32, width: u32, height: u32) -> Self {
        RectIterator {
            x: 0,
            y: 0,
            pixel_size,
            width,
            height,
        }
    }
}

impl Iterator for RectIterator {
    type Item = ((i32, i32), Rect);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y as u32 == self.height {
            None
        } else {
            let position = (self.x, self.y);

            let rect = Rect::new(
                self.x * (self.pixel_size as i32 + 1) + 1,
                self.y * (self.pixel_size as i32 + 1) + 1,
                self.pixel_size,
                self.pixel_size,
            );

            self.x = (self.x + 1) % self.width as i32;
            self.y = if self.x == 0 { self.y + 1 } else { self.y };

            Some((position, rect))
        }
    }
}
