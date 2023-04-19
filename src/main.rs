use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color};
use std::{thread::sleep, time::Duration};

use fonts::Fonts;
use sheet::Sheet;
use status_bar::StatusBar;
use view::View;

pub mod fonts;
pub mod sheet;
pub mod status_bar;
pub mod view;

pub fn main() {
    let mut sheet = Sheet::new(50, 50);
    sheet.set_color((10, 10), Color::YELLOW);
    let mut status_bar = StatusBar::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Drawbox", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let fonts = Fonts::new();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let mouse_state = event_pump.mouse_state();
        let position = sheet.translate_position(mouse_state.x(), mouse_state.y());
        if let Some(position) = position {
            status_bar.last_mouse_position = position;
            if mouse_state.is_mouse_button_pressed(MouseButton::Left) {
                sheet.set_color(position, Color::RED);
            }
        }

        sheet.draw(&mut canvas, &fonts);
        status_bar.draw(&mut canvas, &fonts);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
