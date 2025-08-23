// src/main.rs
use std::time::Duration;
use std::thread;

mod signal;
mod cli;

fn main() {
    println!("TimerTerm: Hello, world!");

    // Parse CLI arguments
    let args: Vec<String> = std::env::args().collect();
    let duration = cli::parse_args(args).unwrap_or(600); // TODO: Make sure we handle errors

    // Register signal handlers
    signal::register_sigint_handler();

    let start = std::time::Instant::now();
    // DELETEME: Keep running until we implement proper signal handling
    loop {
        if signal::should_exit() || start.elapsed().as_secs() >= duration as u64 {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
}
