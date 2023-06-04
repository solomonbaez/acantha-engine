extern "C" {
    pub fn log_number(number: usize);
}

fn main() {
    // JS communication requires unsafe declarations
    unsafe {
        log_number(4);
        engine::clear_screen_to_color(0.0, 0.0, 1.0, 1.0);
    };
}
