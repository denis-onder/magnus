mod global;
mod window;

use global::CONSTANTS;

fn main() {
    window::render_window(CONSTANTS::TITLE, CONSTANTS::WIDTH, CONSTANTS::HEIGHT);
}
