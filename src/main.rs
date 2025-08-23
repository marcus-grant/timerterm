// src/main.rs
use std::time::Duration;
use std::thread;

mod signal;

fn main() {
    println!("TimerTerm: Hello, world!");

    // Register signal handlers
    signal::register_sigint_handler();

    // DELETEME: Keep running until we implement proper signal handling
    loop {
        if signal::should_exit() { break; }
        thread::sleep(Duration::from_millis(100));
    }
}
