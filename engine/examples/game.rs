use engine::{Event, Key};

fn main() {
    let mut x_pos = 200.0;
    let mut y_pos = 30.0;

    let mut x_dir = 1.0;
    let mut y_dir = 1.0;

    let speed = 0.25;

    engine::set_event_handler(move |context, event| {
        if let Event::Draw = event {
            x_pos += x_dir * speed;
            y_pos += y_dir * speed;

            if x_pos <= 0.0 || x_pos >= 250.0 {
                x_dir *= -1.0;
            }

            if y_pos <= 0.0 || y_pos >= 200.0 {
                y_dir *= -1.0;
            }

            context.clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
            context.draw_rectangle(x_pos, y_pos, 50., 50., 1.0, 0.0, 0.0, 1.0);
        }
        if let Event::KeyDown(Key::Up) = event {
            y_dir += 5.0;
        }
        if let Event::KeyDown(Key::Down) = event {
            y_dir -= 5.0;
        }
        if let Event::KeyDown(Key::Right) = event {
            x_dir += 5.0;
        }
        if let Event::KeyDown(Key::Left) = event {
            x_dir -= 5.0;
        }
        if let Event::KeyDown(Key::Space) = event {
            x_dir = 0.0;
            y_dir = 0.0;
        }
    });
}
