extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use rand::{thread_rng, Rng};

fn handle_color_debug(canvas: &mut WindowCanvas) {
    let mut rng = thread_rng();

    let red: u8 = rng.gen_range(0..255);
    let green: u8 = rng.gen_range(0..255);
    let blue: u8 = rng.gen_range(0..255);

    let rgb = Color::RGB(red, green, blue);
    println!("{:?}", rgb);

    canvas.set_draw_color(rgb);
}

pub fn handle_event(event: &Event, canvas: &mut WindowCanvas) {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => std::process::exit(0),
        Event::KeyDown {
            keycode: Some(Keycode::F10),
            ..
        } => handle_color_debug(canvas),
        _ => {}
    }
}
