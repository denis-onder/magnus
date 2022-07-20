mod window;

mod CONSTANTS {
    pub const TITLE: &str = "Magnus";
    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 600;
}

fn main() {
    window::render_window(CONSTANTS::TITLE, CONSTANTS::WIDTH, CONSTANTS::HEIGHT);
}
