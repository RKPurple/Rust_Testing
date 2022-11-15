use rust_gpiozero::*;
use std::thread;
use std::time::Duration;
// Test
fn main() {
    // Create a new LED attached to Pin 17
    let led = LED::new(17);

    // Blink the LED 5 times
    for _ in 0..5 {
        led.on();
        thread::sleep(Duration::from_secs(1));
        led.off();
        thread::sleep(Duration::from_secs(1));
    }
}
