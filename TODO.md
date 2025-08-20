# TODO.md - TimerTerm Development Tasks

## Development Workflow

### TDD Process (E2E-First)
1. **Write E2E Test**: Create a failing end-to-end test for a complete user behavior
2. **Run E2E Test**: Verify it fails for the right reason (not a bad test)
3. **Identify Units**: Break down what components are needed
4. **For Each Unit**:
   - Write failing unit test (Red)
   - Verify test fails correctly
   - Write minimal code to pass (Green)
   - Refactor while keeping green
5. **Run E2E Test**: Check if it passes
6. **Iterate**: If E2E still fails, identify missing units and repeat
7. **Next Behavior**: Move to next E2E test

### Test File Organization
- `tests/` - E2E tests using rexpect/assert_cmd
- `src/*/tests.rs` - Unit tests in each module
- `tests/fixtures/` - Test data and expected outputs

---

## Active Tasks
*Currently working on:*
- [ ] Set up project structure and Cargo.toml

---

## E2E Test Scenarios

### 1. Signal Handling - Terminal Restoration

#### E2E Test: Program restores terminal on Ctrl+C
```rust
// tests/signal_handling.rs
#[test]
fn test_ctrl_c_restores_terminal() {
    // 1. Capture initial terminal state
    // 2. Start timerterm process
    // 3. Verify program is running
    // 4. Send SIGINT signal
    // 5. Verify clean exit (exit code)
    // 6. Verify terminal state restored
}
```

**Tasks:**
- [ ] Write E2E test using rexpect to spawn and kill process
- [ ] Implement minimal SIGINT handler

**Unit Tests Required:**

##### Terminal State Management
- [ ] Test: `terminal::save_termios()` returns valid termios struct
- [ ] Test: `terminal::restore_termios()` restores saved settings
- [ ] Test: `terminal::TerminalGuard::new()` saves state
- [ ] Test: `terminal::TerminalGuard::drop()` restores state

##### Signal Handler Setup
- [ ] Test: `signal::register_handler(SIGINT)` returns Ok
- [ ] Test: `signal::handler` sets global flag correctly
- [ ] Test: Main loop checks signal flag and exits

##### ANSI Cleanup
- [ ] Test: `ansi::reset_scrolling_region()` generates correct sequence
- [ ] Test: `ansi::show_cursor()` generates correct sequence
- [ ] Test: `display::cleanup()` writes all reset sequences

---

### 2. CLI Argument Parsing

#### E2E Test: Program parses command-line arguments correctly
```rust
// tests/cli_arguments.rs
#[test]
fn test_duration_parsing() {
    // Test various duration formats
    // 1. Run with "30" - verify 30 seconds
    // 2. Run with "90" - verify 90 seconds  
    // 3. Run with "5:00" - verify 300 seconds
    // 4. Run with "1:30:00" - verify 5400 seconds
    // 5. Run with no args - verify 600 seconds (10 min default)
}

#[test]
fn test_help_version_flags() {
    // 1. Run with --help, verify help text and exit 0
    // 2. Run with -h, verify same as --help
    // 3. Run with --version, verify version string and exit 0
    // 4. Run with -v, verify same as --version
}

#[test]
fn test_invalid_input_handling() {
    // 1. Run with "invalid" - verify error message and exit 1
    // 2. Run with "25:61" - verify error for invalid minutes
    // 3. Run with "25:00:00" - verify error for > 24 hours
    // 4. Run with negative number - verify error
}
```

**Tasks:**
- [ ] Write E2E tests using assert_cmd for CLI testing
- [ ] Implement argument parsing with std::env::args()
- [ ] Add input validation and error handling

**Unit Tests Required:**

##### Argument Parsing
- [ ] Test: `cli::parse_args()` extracts arguments from Vec<String>
- [ ] Test: Handles empty args (no arguments provided)
- [ ] Test: Identifies flags vs positional arguments

##### Duration Parsing
- [ ] Test: `cli::parse_duration("30")` returns Ok(30)
- [ ] Test: `cli::parse_duration("90")` returns Ok(90)
- [ ] Test: `cli::parse_duration("5:00")` returns Ok(300)
- [ ] Test: `cli::parse_duration("1:30:00")` returns Ok(5400)
- [ ] Test: `cli::parse_duration("")` returns Ok(600) for default
- [ ] Test: `cli::parse_duration("invalid")` returns Err
- [ ] Test: `cli::parse_duration("25:00")` returns Err (invalid minutes)
- [ ] Test: `cli::parse_duration("25:00:00")` returns Err (>24 hours)
- [ ] Test: `cli::parse_duration("-5")` returns Err (negative)

##### Help and Version
- [ ] Test: `cli::should_show_help()` detects --help and -h
- [ ] Test: `cli::should_show_version()` detects --version and -v  
- [ ] Test: `cli::format_help()` returns help text string
- [ ] Test: `cli::format_version()` returns version string

##### Error Messages
- [ ] Test: Error messages are clear and actionable
- [ ] Test: Suggest valid format on parse errors

---

### 3. Static Timer Display

#### E2E Test: Program displays timer in reserved space
```rust
// tests/timer_display.rs
#[test]
fn test_static_timer_display() {
    // 1. Start timerterm with 5 second duration
    // 2. Capture output using vt100 parser
    // 3. Verify line 1 contains "TIMER: 0:05"
    // 4. Verify line 2 contains separator "===="
    // 5. Verify cursor is below reserved area
    // 6. Send SIGINT to exit cleanly
}
```

**Tasks:**
- [ ] Write E2E test using rexpect + vt100 to verify display
- [ ] Implement static timer display (no countdown yet)

**Unit Tests Required:**

##### Terminal Size Detection
- [ ] Test: `terminal::get_size()` returns (cols, rows) using ioctl
- [ ] Test: `terminal::get_size()` handles ioctl failure gracefully

##### Scrolling Region Setup
- [ ] Test: `ansi::set_scrolling_region(3, 24)` generates `\x1b[3;24r`
- [ ] Test: `display::reserve_lines(2)` sets correct scrolling region

##### Timer Formatting
- [ ] Test: `timer::format_duration(5)` returns "0:05"
- [ ] Test: `timer::format_duration(65)` returns "1:05"
- [ ] Test: `timer::format_duration(3665)` returns "1:01:05"
- [ ] Test: Leading zeros removed correctly

##### Display Rendering
- [ ] Test: `display::render_timer_line()` formats "TIMER: X:XX" correctly
- [ ] Test: `display::render_separator()` creates full-width "===" line
- [ ] Test: `display::position_cursor()` generates correct ANSI sequence

##### Output Writing
- [ ] Test: `terminal::write()` flushes stdout after writing
- [ ] Test: ANSI sequences written in correct order

---

### 4. Countdown with Basic Progress

#### E2E Test: Timer counts down with progress indicator
```rust
// tests/countdown_progress.rs
#[test]
fn test_countdown_with_progress() {
    // 1. Start timerterm with 3 second duration
    // 2. Verify initial display "TIMER: 0:03" with no inversion
    // 3. After 1 second, verify "TIMER: 0:02" with ~33% inverted
    // 4. After 2 seconds, verify "TIMER: 0:01" with ~66% inverted
    // 5. After 3 seconds, verify "TIMER: 0:00" with 100% inverted
    // 6. Verify separator line matches inversion of timer line
}
```

**Tasks:**
- [ ] Write E2E test with timed checks using rexpect
- [ ] Implement countdown timer loop
- [ ] Add basic progress inversion (static, not animated)

**Unit Tests Required:**

##### Timer State Management
- [ ] Test: `timer::Timer::new(duration)` stores end time correctly
- [ ] Test: `timer::Timer::remaining()` calculates time left
- [ ] Test: `timer::Timer::elapsed()` returns time since start
- [ ] Test: `timer::Timer::progress()` returns 0.0 to 1.0

##### Progress Calculation
- [ ] Test: `display::calculate_invert_width(0.0, 80)` returns 0
- [ ] Test: `display::calculate_invert_width(0.5, 80)` returns 40
- [ ] Test: `display::calculate_invert_width(1.0, 80)` returns 80

##### Inverted Display Rendering
- [ ] Test: `ansi::start_inverse()` returns `\x1b[7m`
- [ ] Test: `ansi::end_inverse()` returns `\x1b[27m`
- [ ] Test: `display::render_with_inversion()` splits text at correct position
- [ ] Test: Inversion applied to both timer and separator lines

##### Update Loop
- [ ] Test: Timer updates every second (threading)
- [ ] Test: Display refreshes without flicker (cursor save/restore)
- [ ] Test: Signal flag stops update loop

---

### 5. Basic Shell Integration

#### E2E Test: Timer runs while shell is active
```rust
// tests/shell_integration.rs
#[test]
fn test_timer_with_shell() {
    // 1. Start timerterm with 10 second duration
    // 2. Verify timer display appears
    // 3. Verify shell prompt appears below timer
    // 4. Send command "echo test" to shell
    // 5. Verify "test" output appears
    // 6. Verify timer continues counting during shell use
    // 7. Send "exit" to close shell
    // 8. Verify program exits cleanly
}
```

**Tasks:**
- [ ] Write E2E test using rexpect for shell interaction
- [ ] Implement shell spawning in main thread
- [ ] Coordinate timer thread with shell thread

**Unit Tests Required:**

##### Shell Spawning
- [ ] Test: `shell::get_user_shell()` returns $SHELL or /bin/bash
- [ ] Test: `shell::spawn_shell()` returns Child process handle
- [ ] Test: Shell environment variables are preserved

##### Thread Coordination
- [ ] Test: Timer thread starts before shell
- [ ] Test: Timer thread continues while shell runs
- [ ] Test: Shared state accessed safely (atomics/mutex)

##### Shell Exit Handling
- [ ] Test: `shell::wait_for_exit()` returns exit status
- [ ] Test: Timer thread notified when shell exits
- [ ] Test: Clean shutdown sequence when shell exits

##### Display Coordination
- [ ] Test: Timer updates don't interfere with shell output
- [ ] Test: Cursor position maintained correctly
- [ ] Test: Shell output appears below reserved lines