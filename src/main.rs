mod actions;
mod global;
mod window;

use global::CONSTANTS;

fn main() {
    let options = window::WindowRenderOptions {
        title: String::from(CONSTANTS::TITLE),
        width: CONSTANTS::WIDTH,
        height: CONSTANTS::HEIGHT,
        event_handler_func: actions::handle_event,
    };

    window::render_window(options);
}
