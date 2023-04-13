use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color};
use status_bar::StatusBar;
use std::{thread::sleep, time::Duration};
use view::View;

pub mod status_bar;
pub mod view;

const FONT_PATH: &str = "./resources/Roboto-Light.ttf";

pub fn main() {
    // if let Some(arg) = std::env::args().collect::<Vec<String>>().get(1) {
    //     if arg == "test" {
    //         let iter = RectIterator::new(10, 5, 5);
    //         for element in iter {
    //             println!("{:?}", element);
    //         }
    //         return;
    //     }
    // }

    let mut view = View::new(50, 50);
    view.set_color((10, 10), Color::YELLOW);
    let mut status_bar = StatusBar::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Drawbox", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(FONT_PATH, 14).unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let mouse_state = event_pump.mouse_state();
        let position = view.translate_position(mouse_state.x(), mouse_state.y());
        if mouse_state.is_mouse_button_pressed(MouseButton::Left) && position.is_some() {
            view.set_color(position.unwrap(), Color::RED);
        }

        view.draw(&mut canvas);
        status_bar.draw(&mut canvas, &font, position);

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
