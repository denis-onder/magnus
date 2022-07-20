extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use std::time::Duration;

pub struct WindowRenderOptions {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub event_handler_func: fn(&Event, &mut WindowCanvas),
}

fn find_opengl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }

    return None;
}

pub fn render_window(options: WindowRenderOptions) {
    let opengl_driver_index = find_opengl_driver().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(options.title.as_str(), options.width, options.height)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(opengl_driver_index)
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    canvas.window().gl_set_context_to_current().unwrap();

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            (options.event_handler_func)(&event, &mut canvas);
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
}
