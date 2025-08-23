// src/signal.rs
use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;

static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);

extern "C" fn sigint_handler(_: i32) {
    SHOULD_EXIT.store(true, Ordering::Relaxed);
}

pub fn register_sigint_handler() {
    unsafe {
        libc::signal(libc::SIGINT, sigint_handler as libc::sighandler_t);
    }
}

pub fn should_exit() -> bool {
    SHOULD_EXIT.load(Ordering::Relaxed)
}

// ============ Unit Tests =============
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_exit_initially_false() {
        // Reset the flag for clean test
        SHOULD_EXIT.store(false, Ordering::Relaxed);
        assert_eq!(should_exit(), false);
    }

    #[test]
    fn signal_handler_sets_flag() {
        // Reset the flag for clean test
        SHOULD_EXIT.store(false, Ordering::Relaxed);
        // Call signal handler directly
        sigint_handler(libc::SIGINT);
        // Verify flag is set
        assert_eq!(should_exit(), true);
    }

    #[test]
    fn register_handler_returns_ok_no_panic() {
        // Harder test since it's a system call,
        // but we can at least verify no panics
        register_sigint_handler();
        // If we get here, it didn't panic
    }
}

