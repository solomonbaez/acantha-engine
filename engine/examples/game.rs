extern "C" {
    pub fn log_number(number: usize);
}

fn main() {
    // JS communication requires unsafe declarations
    unsafe { log_number(4) }
}
