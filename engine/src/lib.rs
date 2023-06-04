#[no_mangle]
pub extern "C" fn key_pressed() {
    EVENT_HANDLER.with(|event_handler| (event_handler.borrow_mut())())
}

extern "C" {
    fn js_screen_color(red: f32, green: f32, blue: f32, alpha: f32);
}

// encase unsafe calls
pub fn clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { js_screen_color(red, green, blue, alpha) }
}

thread_local! {
    pub static EVENT_HANDLER:
        std::cell::RefCell<Box<dyn FnMut()>> = std::cell::RefCell::new(Box::new(||{}))
}

pub fn set_event_handler(function: impl FnMut() + 'static) {
    EVENT_HANDLER.with(|event_handler| {
        *event_handler.borrow_mut() = Box::new(function);
    });
}
