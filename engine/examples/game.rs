use engine::Key;

extern "C" {
    pub fn log_number(number: usize);
}

fn main() {
    let mut blue_amount = 0.0;
    let mut red_amount = 0.0;
    engine::set_event_handler(move |key| match key {
        Key::Left => {
            blue_amount += 0.1;
            engine::clear_screen_to_color(red_amount, 0.0, blue_amount, 1.0);
        }
        Key::Right => {
            blue_amount -= 0.1;
            engine::clear_screen_to_color(red_amount, 0.0, blue_amount, 1.0);
        }
        Key::Up => {
            red_amount += 0.1;
            engine::clear_screen_to_color(red_amount, 0.0, blue_amount, 1.0);
        }
        Key::Down => {
            red_amount -= 0.1;
            engine::clear_screen_to_color(red_amount, 0.0, blue_amount, 1.0);
        }
        Key::Space => {
            blue_amount = 0.0;
            red_amount = 0.0;
            engine::clear_screen_to_color(red_amount, 0.0, blue_amount, 1.0);
        }
    });
}
