# SPEC.md - TimerTerm Technical Specification

## Overview
TimerTerm is a terminal application written in Rust that reserves the top portion of the terminal for a persistent countdown timer while maintaining full terminal functionality below.

## Development Methodology
**Test-Driven Development with E2E-First Approach:**

1. **Start with End-to-End Test**: Write a failing E2E test that describes the expected behavior
2. **Decompose into Unit Tests**: Identify the units needed to satisfy the E2E test
3. **Red-Green-Refactor Cycle for Units**:
   - **Red**: Write a failing unit test for expected behavior
   - **Verify Red**: Ensure test fails correctly (no false positives)
   - **Green**: Implement minimal code to pass the test
   - **Refactor**: Clean up test and implementation while keeping green
4. **Iterate Units**: Continue until enough units exist to pass the E2E test
5. **Next E2E Test**: Write the next failing E2E test and repeat

This approach ensures:
- Features are driven by actual user-facing behavior
- No unnecessary code is written
- Each component is testable in isolation
- System behavior is always validated end-to-end

## Development Approach
Two-phase implementation prioritizing Rust learning:
- **Phase 1**: Raw ANSI implementation with minimal dependencies (maximum learning)
- **Phase 2**: Refactor to use terminal libraries for polish and cross-platform support

---

## Phase 1: Core Implementation (Learning Focus)

### Dependencies
- `libc = "0.2"` - For all Unix system calls (termios, ioctl, signals)
- No other external dependencies - raw ANSI escape sequences for everything else

## Terminal Safety and RAII Pattern

### TerminalGuard Design
```rust
struct TerminalGuard {
    original_termios: libc::termios,
    // Exit behavior configurable via CLI flag
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // 1. Reset scrolling region to full screen
        // 2. Restore original termios settings
        // 3. Cursor visibility - see TODO.md for future consideration
        // 4. Screen content - see TODO.md for exit mode handling
    }
}
```

### Exit Behavior (MVP)
- **Default**: Preserve TimerTerm session (show what was done during timer)
- **Optional flag**: `--restore-previous` to return to pre-TimerTerm state
- **Future**: Additional exit modes to be explored (see TODO.md)

### Implementation Details
- **ANSI Sequences**: Implement as const strings or a dedicated module
- **Terminal Size Detection**: Use `libc::ioctl` with `TIOCGWINSZ`
- **Terminal Settings**: Use `libc::tcgetattr`/`tcsetattr` for termios manipulation
- **Standard Output**: Direct writes to `std::io::stdout()` with manual flushing
- **Error Handling**: Custom error type using Rust's `std::error::Error` trait
- **Platform Layer**: `Platform` trait with Unix implementation, designed for future macOS/Windows

---

## Phase 2: Enhanced Implementation (Polish Focus)

### Dependencies  
- `crossterm = "0.28"` - For cross-platform terminal manipulation
- `clap = "4.5"` - For command-line argument parsing  
- `signal-hook = "0.3"` - For better signal handling

### Threading Model Improvements
**Future options to explore:**

1. **Three-thread model with channels**:
   - Main thread: Coordinator with event loop
   - Shell thread: Dedicated shell process management
   - Display thread: Timer updates and animations
   - Use Rust channels (`mpsc`) for thread communication
   - Better separation of concerns

2. **Single-threaded async with polling**:
   - Use `tokio` or async-std
   - Poll shell process with timeout
   - Timer updates between polls
   - No thread synchronization needed
   - More complex but potentially more efficient

3. **Thread pool with work stealing**:
   - For handling multiple concurrent timers (future feature)
   - Better resource utilization
   - Could use `rayon` or similar

**Evaluation criteria for future threading model:**
- Signal handling reliability
- Terminal resize responsiveness  
- Resource usage (CPU, memory)
- Code complexity vs maintainability

### Improvements
- Cross-platform support (Windows Terminal, etc.)
- Better error handling with `anyhow` or `thiserror`
- More robust terminal state management
- Additional timer modes (stopwatch, pomodoro)
- Configuration file support
- Multiple concurrent timers
- Advanced notification options

---

## Core Requirements (Both Phases)

### Terminal Manipulation
*To be determined through specification questions*

### Timer Functionality
- **Duration Parsing**: 
  - Accept formats: seconds (`300`), MM:SS (`5:00`), HH:MM:SS (`1:30:00`)
  - Default: 10 minutes (600 seconds)
  - Maximum: 23:59:59 (under 24 hours)
  
- **Display Format**:
  - Remove leading zeros: `1:23:45` not `01:23:45`
  - Remove zero hours: `23:45` not `0:23:45`
  - Keep zero minutes: `0:45` not just `45`
  
- **Timer Mode**: Absolute time-based
  - Calculate end_time = current_time + duration at start
  - Always check against real time (handles sleep/suspend)
  - If computer wakes from sleep past end_time, timer shows as expired
  
- **Timer Expiration Behavior**:
  1. Terminal bell (`\x1b[7`)
  2. Desktop notification via `notify-send` command
  3. Display changes to: `TIMER: [spaces] COMPLETED!` (right-justified)
  4. Visual strobe animation:
     - 8-character wide inverted color block
     - Bounces left-to-right between column 0 and terminal width
     - Inverts FG/BG colors for text it overlaps (including "COMPLETED!")
     - Continues bouncing until user interaction
  5. Reset scrolling region to full terminal (timer display can now scroll away)
  6. Timer display remains visible but is no longer "reserved" space
  7. Continue running (no auto-exit in MVP)
  
- **Update Frequency**: 1 second intervals

### Desktop Notifications
- **MVP**: Shell out to `notify-send` command
  ```rust
  std::process::Command::new("notify-send")
      .arg("TimerTerm")
      .arg("Timer completed!")
      .spawn();
  ```
- **Future**: X11 support (also via notify-send), macOS native notifications

### Display Components
- **Reserved Lines**: 2 lines total
  - Line 1: Timer display with animated progress bar via color inversion
  - Line 2: Separator line ('=' characters) with matching color inversion
  
- **Timer Animation Behavior**:
  1. Progress bar advances left-to-right using inverted terminal colors
  2. Timer format starts as `TIMER: HH:MM:SS`
  3. When inversion reaches the time text, time "detaches" and moves right
  4. Time display stays 1 character ahead of progress bar edge
  5. Spaces dynamically inserted between "TIMER:" and time as needed
  6. When time reaches terminal edge, it stops and gets inverted by progress
  7. At 100%, both full lines are inverted
  
- **Visual Examples**:
  ```
  Start:    TIMER: 01:23:45
  Early:    \u2588\u2588\u2588\u2588MER: 01:23:45  
  Middle:   \u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588  00:41:23
  Late:     \u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588 00:00:45
  End:      \u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588\u2588
  ```
  
- **ANSI Implementation**:
  - Use `\x1b[7m` for inverse video start
  - Use `\x1b[27m` for inverse video end
  - Terminal's default colors (no custom colors in Phase 1)

### Shell Integration
- **Launch Method (MVP)**: `std::process::Command` with user's `$SHELL`
- **Future**: Transition to fork/exec with `libc` for more control
- **Shell Exit Handling**:
  - If shell exits before timer completes:
    - Send notification: "Shell exited with HH:MM:SS remaining"
    - Exit TimerTerm (don't wait for timer)
  - If shell crashes: Same as above
  - Track shell exit status for logging

### Threading Model (MVP)
- **Simple two-thread model**:
  - Main thread: Launches shell via `std::process::Command::spawn()`, waits for exit
  - Timer thread: Updates display every second via `std::thread::spawn()`
  - Communication: Atomic flags for signals and state changes
  - Simplest implementation, good enough for MVP

### Signal Handling
- **Required Signals** (priority order):
  1. **SIGINT** (Ctrl+C) - Clean shutdown with terminal restoration
  2. **SIGTERM** - System shutdown, same as SIGINT
  3. **SIGWINCH** - Terminal resize, recalculate display width and animation positions
  4. **SIGHUP** - Terminal disconnect, attempt cleanup
  5. **SIGCHLD** - Child (shell) process died, log and exit
  6. **SIGTSTP** (Ctrl+Z) - TBD: Allow suspend or block?

- **Implementation**: Direct `libc::signal()` for Phase 1
- **Resize Behavior**: 
  - Recalculate terminal width on SIGWINCH
  - Adjust progress bar animation to new width
  - Reposition time display if needed
  - Handle edge case: terminal too narrow for display

### Logging
- **Log Location**: `$XDG_CACHE_HOME/timerterm/timerterm.log` (defaults to `~/.cache/timerterm/`)
- **Log Rotation**: Delete and recreate when log exceeds 64MB
- **Log Format**: Syslog-style plain text (works with grep, awk, tail, etc.)
  ```
  Jan 15 14:23:45 timerterm[12345]: START duration=600s end_time=14:33:45
  Jan 15 14:33:45 timerterm[12345]: COMPLETED duration=600s
  Jan 15 14:30:12 timerterm[12345]: SHELL_EXIT remaining=213s exit_code=0
  ```
- **Future Options**: 
  - `--no-log` flag to disable logging
  - `--log-file PATH` for custom location
  - Configurable rotation strategies

## Technical Architecture

### Build Configuration
```toml
[package]
name = "timerterm"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"

[profile.release]
opt-level = 2      # Good balance of speed and size
lto = true         # Link-time optimization for smaller binary
strip = true       # Strip symbols for smaller binary
```

### Testing Strategy

**Test-Driven Development Process:**
Following the E2E-first methodology described above, using these tools:

**Testing Dependencies:**
```toml
[dev-dependencies]
# E2E and CLI testing
assert_cmd = "2.0"      # Test binary execution
predicates = "3.0"      # Assertions on command output
rexpect = "0.5"         # Interactive terminal testing

# Terminal output verification
vt100 = "0.3"           # Parse ANSI sequences, verify display

# Snapshot testing (optional)
insta = "1.34"          # Snapshot testing for regression
```

**Unit Testable Components:**
```rust
// Pure functions to unit test:
- Duration parsing: parse_duration("5:30") -> 330
- Time formatting: format_time(330) -> "5:30"
- Progress calculation: get_progress(elapsed, total) -> 0.45
- Log formatting: format_log_entry(event) -> "Jan 15 14:23:45..."
- ANSI sequence generation: build_move_cursor(5, 10) -> "\x1b[5;10H"
```

**E2E Test Scenarios:**
```rust
// Using rexpect for full terminal behavior:
- Timer starts and displays correctly
- Progress bar advances over time
- Timer completes with notification
- Ctrl+C restores terminal properly
- Terminal resize updates display
```

**Terminal Output Testing:**
1. **Unit Level**: Test ANSI string generation
2. **Integration Level**: Use `vt100` to parse and verify ANSI output
3. **E2E Level**: Use `rexpect` to control PTY and verify behavior

**Testing Abstraction Pattern:**
```rust
trait TerminalWriter {
    fn write(&mut self, data: &str) -> Result<()>;
}

// Production: writes to stdout
struct StdoutWriter;

// Testing: captures to buffer
struct BufferWriter { buffer: Vec<u8> }
```

**Manual Testing Checklist:**
For behaviors that cannot be automated:
- [ ] Timer completes normally
- [ ] Ctrl+C during timer (terminal restored?)
- [ ] Shell exits early (notification sent?)
- [ ] Terminal resize during timer (display adapts?)
- [ ] System sleep/wake during timer (time still accurate?)
- [ ] Very narrow terminal (< 20 cols)
- [ ] Various duration formats (HH:MM:SS, MM:SS, seconds)
- [ ] Progress bar animation correctness
- [ ] Strobe animation on completion

### Command-Line Interface
**MVP Arguments:**
```bash
timerterm [duration]         # Default 10 minutes if omitted
timerterm 30                 # 30 seconds
timerterm 5:00               # 5 minutes  
timerterm 1:30:00            # 1 hour 30 minutes
```

**MVP Flags:**
```bash
timerterm --help / -h        # Show help
timerterm --version / -v     # Show version
timerterm --no-bell          # Disable terminal bell on completion
timerterm --no-notify        # Disable desktop notification
timerterm --restore-previous # Restore pre-timer terminal state on exit
```

**Implementation (Phase 1):**
- Use `std::env::args()` directly for argument parsing
- Manual parsing for learning Rust string manipulation
- Simple match statements for flag handling

**Future CLI enhancements:**
```bash
timerterm --log-file PATH    # Custom log location
timerterm --no-log           # Disable logging
timerterm --label "STRING"   # Label for the timer
timerterm --command "cmd"    # Run specific command instead of shell
timerterm --mode MODE         # stopwatch, pomodoro modes
```

### Error Handling
**Custom error type for domain-specific errors:**
```rust
#[derive(Debug)]
enum TimerTermError {
    Terminal(String),
    Shell(String), 
    Signal(String),
    Io(std::io::Error),
    Parse(String),  // For CLI parsing errors
}

impl std::error::Error for TimerTermError {}
impl From<std::io::Error> for TimerTermError { ... }
```

**Error Strategy:**
- Use standard library errors where they exist (`std::io::Error`)
- Create custom enum for domain-specific errors
- Implement `From` traits for ergonomic `?` operator usage
- Fatal errors: Terminal setup, termios save, shell not found
- Recoverable errors: Notification failures, log write failures
- Future: Consider `thiserror` crate if error handling becomes complex

### Module Structure
**Idiomatic Rust organization:**

```
src/
\u251c\u2500\u2500 main.rs          // Binary entry point only - arg parsing, calls lib
\u251c\u2500\u2500 lib.rs           // Library root - public API, re-exports
\u251c\u2500\u2500 error.rs         // Custom error types
\u251c\u2500\u2500 terminal/        // Terminal subsystem
\u2502   \u251c\u2500\u2500 mod.rs       // Terminal trait and common types
\u2502   \u251c\u2500\u2500 ansi.rs      // ANSI escape sequences constants
\u2502   \u2514\u2500\u2500 guard.rs     // TerminalGuard RAII implementation
\u251c\u2500\u2500 timer.rs         // Timer state and logic
\u251c\u2500\u2500 display.rs       // Display rendering and animations
\u251c\u2500\u2500 shell.rs         // Shell process management
\u251c\u2500\u2500 signal.rs        // Signal handling (singular, idiomatic)
\u2514\u2500\u2500 log.rs           // Logging (singular, idiomatic)
```

**Key Rust idioms applied:**
- Binary + library pattern (reusable lib.rs)
- Modules match ownership boundaries
- Submodules only when there's multiple related files
- Singular names for modules (log not logging)
- Types and traits at module root, implementations in submodules
- Platform-specific code via cfg attributes inline (not separate dir for MVP)

## Platform Support Strategy

### Phase 1: Unix Implementation (Linux/macOS)
- Pure `libc` implementation for all system calls
- Shared codebase for Linux and macOS (both POSIX-compliant)
- Focus on learning unsafe Rust and FFI

### Future Platform Expansion
- **macOS**: Verify compatibility, adjust for any BSD-specific differences
- **Windows**: Completely different implementation using Windows Console API for status line
- **Design Pattern**: Platform trait to abstract OS-specific operations

```rust
trait Platform {
    fn get_terminal_size() -> Result<(u16, u16)>;
    fn setup_terminal() -> Result<TerminalGuard>;
    fn reserve_lines(n: u16) -> Result<()>;
}
```