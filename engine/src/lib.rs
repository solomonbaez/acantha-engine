extern "C" {
    fn js_screen_color(red: f32, green: f32, blue: f32, alpha: f32);
}

// encase unsafe calls
pub fn clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { js_screen_color(red, green, blue, alpha) }
}
