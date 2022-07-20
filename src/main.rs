mod actions;
mod global;
mod window;

use global::constants;

fn main() {
    let options = window::WindowRenderOptions {
        title: String::from(constants::TITLE),
        width: constants::WIDTH,
        height: constants::HEIGHT,
        event_handler_func: actions::handle_event,
    };

    window::render_window(options);
}
