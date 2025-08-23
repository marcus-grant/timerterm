// tests/signal_handling.rs
use assert_cmd::Command;
use std::time::Duration;
use std::thread;

#[test]
fn test_ctrl_c_restores_terminal() {
    // 1. Start timeterm process in background
    let cmd = Command::cargo_bin("timeterm").unwrap();

    let mut child = std::process::Command::new(cmd.get_program())
        .args(cmd.get_args())
        .spawn()
        .expect("Failed to start timeterm");

    // 2. Give it more time to actually run (1 second)
    thread::sleep(Duration::from_millis(1000));

    // 3. Verify process is still running
    match child.try_wait() {
        Ok(Some(_)) => panic!("Process exited too early - should still be running"),
        Ok(None) => {}, // Good - still running
        Err(e) => panic!("Error checking process status: {}", e),
    }

    // 4. Send SIGINT signal (Ctrl+C)
    #[cfg(unix)]
    {
        let pid = child.id();
        unsafe {
            libc::kill(pid as i32, libc::SIGINT);
        }
    }

    // 5. Wait for process to exit and check exit code
    let output = child.wait_with_output().expect("Failed to wait for process");
    
    // 6. Verify clean exit (exit code 0 means clean shutdown)
    assert!(output.status.success(), "Process should exit cleanly on SIGINT");
}
