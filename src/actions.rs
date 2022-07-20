extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

pub fn handle_event(event: &Event, canvas: &mut WindowCanvas) {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => std::process::exit(0),
        Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } => canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0)),
        _ => {}
    }
}
