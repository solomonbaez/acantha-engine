use engine::Key;

fn main() {
    let mut x_pos = 200.0;
    let mut y_pos = 30.0;

    engine::set_event_handler(move |key| {
        let move_amount = 20.0;
        match key {
            Key::Left => x_pos -= move_amount,
            Key::Right => x_pos += move_amount,
            Key::Up => y_pos += move_amount,
            Key::Down => y_pos -= move_amount,
            Key::Space => {}
        }

        engine::clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
        engine::draw_rectangle(x_pos, y_pos, 100., 100.);
    })
}
