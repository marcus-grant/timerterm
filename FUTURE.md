# TODO2.md - TimerTerm Development Tasks (Part 2)

## E2E Test Scenarios (Continued)

### 6. Shell Early Exit Handling

#### E2E Test: Handle shell exiting before timer completes
```rust
// tests/shell_early_exit.rs
#[test]
fn test_shell_exits_before_timer() {
    // 1. Start timerterm with 30 second duration
    // 2. Verify timer and shell start normally
    // 3. Send "exit" command to shell after 5 seconds
    // 4. Verify notification shows "Shell exited with 0:25 remaining"
    // 5. Verify log entry created for early exit
    // 6. Verify program exits cleanly (doesn't wait for timer)
}
```

**Tasks:**
- [ ] Write E2E test for early shell exit scenario
- [ ] Implement notification on early exit
- [ ] Add logging for shell exit events

**Unit Tests Required:**

##### Notification System
- [ ] Test: `notify::send_notification(title, body)` spawns notify-send
- [ ] Test: Notification handles notify-send not being available
- [ ] Test: Time remaining formatted correctly in notification

##### Logging
- [ ] Test: `log::init()` creates log directory if missing
- [ ] Test: `log::write_event()` appends to log file
- [ ] Test: Log rotation when file exceeds 64MB
- [ ] Test: Log format matches syslog style
- [ ] Test: Shell exit event logged with exit code and remaining time

##### Exit Coordination
- [ ] Test: Timer thread stops when shell exits early
- [ ] Test: Cleanup runs even on early exit
- [ ] Test: Exit code preserved from shell

---

### 7. Timer Completion Behavior

#### E2E Test: Timer reaches 0:00 with full completion sequence
```rust
// tests/timer_completion.rs
#[test]
fn test_timer_completion() {
    // 1. Start timerterm with 3 second duration
    // 2. Let timer run to completion
    // 3. Verify terminal bell sounds (check for \x07 in output)
    // 4. Verify desktop notification sent
    // 5. Verify display shows "TIMER: ... COMPLETED!" (right-justified)
    // 6. Verify strobe animation starts (8-char inverting window)
    // 7. Verify scrolling region reset (timer can scroll away)
    // 8. Run command to verify terminal still functional
}
```

**Tasks:**
- [ ] Write E2E test for complete timer expiration flow
- [ ] Implement all completion behaviors
- [ ] Add strobe animation

**Unit Tests Required:**

##### Completion Detection
- [ ] Test: `timer::is_expired()` returns true when remaining is 0
- [ ] Test: Completion triggers exactly once (not repeatedly)

##### Completion Display
- [ ] Test: `display::format_completed()` right-justifies "COMPLETED!"
- [ ] Test: Text changes from time to "COMPLETED!" at expiration

##### Strobe Animation
- [ ] Test: `display::calculate_strobe_position(elapsed)` bounces correctly
- [ ] Test: Strobe width is exactly 8 characters
- [ ] Test: Strobe inverts text it overlaps

##### Terminal Bell
- [ ] Test: `ansi::bell()` returns `\x07`
- [ ] Test: Bell sent exactly once on completion

##### Scrolling Region Reset
- [ ] Test: `ansi::reset_scrolling_region()` returns `\x1b[r`
- [ ] Test: Reset allows timer display to scroll

---

### 8. Full Progress Animation

#### E2E Test: Moving timer with text separation
```rust
// tests/progress_animation.rs
#[test]
fn test_moving_timer_animation() {
    // 1. Start timerterm with 10 second duration
    // 2. Verify timer starts as "TIMER: 0:10"
    // 3. As progress reaches "TIMER:" text, verify time separates
    // 4. Verify time stays 1 char ahead of progress edge
    // 5. Verify spaces inserted between ":" and time as needed
    // 6. When progress reaches right edge, verify time stops moving
    // 7. Verify time gets inverted when progress catches up
}
```

**Tasks:**
- [ ] Write E2E test for full animation behavior
- [ ] Implement timer text separation logic
- [ ] Add smooth animation with proper spacing

**Unit Tests Required:**

##### Animation Calculation
- [ ] Test: `display::calculate_timer_position()` for various progress values
- [ ] Test: Position keeps time 1 char ahead of progress
- [ ] Test: Time stops at terminal width minus time length

##### Text Separation
- [ ] Test: `display::should_separate_time(progress, width)` returns bool
- [ ] Test: Separation occurs when progress reaches end of "TIMER: "

##### Dynamic Spacing
- [ ] Test: `display::calculate_spacing()` returns correct spaces
- [ ] Test: Spaces increase as timer moves right
- [ ] Test: No spaces when timer at right edge

##### Rendering with Animation
- [ ] Test: Text renders correctly at each animation stage
- [ ] Test: Inversion and position work together correctly

---

### 9. Terminal Resize Handling (SIGWINCH)

#### E2E Test: Handle terminal window resizing
```rust
// tests/terminal_resize.rs
#[test]
fn test_terminal_resize() {
    // 1. Start timerterm in a resizable terminal
    // 2. Verify initial display at starting width
    // 3. Simulate terminal resize (smaller)
    // 4. Verify display adapts to new width
    // 5. Verify progress bar scales correctly
    // 6. Simulate resize (larger)
    // 7. Verify display expands properly
    // 8. Verify timer continues accurately during resizes
}
```

**Tasks:**
- [ ] Write E2E test for resize handling
- [ ] Implement SIGWINCH handler
- [ ] Add display recalculation logic

**Unit Tests Required:**

##### Signal Handling
- [ ] Test: `signal::register_handler(SIGWINCH)` succeeds
- [ ] Test: SIGWINCH handler sets resize flag

##### Resize Detection
- [ ] Test: `terminal::get_size()` returns new size after resize
- [ ] Test: Display width updates on resize

##### Display Adaptation
- [ ] Test: Progress bar recalculates for new width
- [ ] Test: Timer position adjusts if terminal too narrow
- [ ] Test: Separator line adjusts to new width
- [ ] Test: Handle minimum width gracefully (< 20 cols)

##### Resize During Animation
- [ ] Test: Animation position recalculates correctly
- [ ] Test: No display corruption during resize
- [ ] Test: Strobe animation adapts to new bounds

---

## Future Enhancements

### Additional Signals
- [ ] SIGTERM handling
- [ ] SIGHUP handling  
- [ ] SIGTSTP (Ctrl+Z) handling decision
- [ ] SIGCHLD proper handling

### Command-Line Flags (MVP)
- [ ] Parse --no-bell flag
- [ ] Parse --no-notify flag
- [ ] Parse --restore-previous flag
- [ ] Integration with relevant E2E tests

### Advanced Features
- [ ] Stopwatch mode (count up)
- [ ] Pomodoro timer mode
- [ ] Configuration file support
- [ ] Custom notification text
- [ ] Multiple concurrent timers
- [ ] Custom timer labels
- [ ] Command execution instead of shell

### Platform Support
- [ ] macOS compatibility testing
- [ ] macOS-specific adjustments
- [ ] Windows Console API research
- [ ] Windows status line implementation

### Phase 2: Library Migration
- [ ] Evaluate benefits of moving from `libc` to `crossterm`
- [ ] Benchmark performance difference
- [ ] Document learning outcomes from Phase 1
- [ ] Refactor if beneficial

---

## Future Considerations

### Terminal Exit Behavior Investigation
**Goal**: Determine the best default behavior and what options users need

#### Tasks:
1. **Research existing tools' exit behavior**
   - [ ] Study how `vim`, `less`, `htop` handle exit
   - [ ] Study how `watch`, `tmux` status bars handle exit
   - [ ] Document findings with pros/cons

2. **Prototype different exit modes**
   - [ ] Implement `--preserve-session` (default): Keep TimerTerm's work visible
   - [ ] Implement `--restore-previous`: Return to pre-TimerTerm terminal state
   - [ ] Implement `--clean-exit`: Clear everything as if TimerTerm never ran
   - [ ] Test each mode with different workflows

3. **Cursor visibility handling**
   - [ ] Research: Can we detect if cursor was hidden before TimerTerm started?
   - [ ] Test: What happens if we save/restore cursor visibility state?
   - [ ] Decision: Should we always force visible on exit or preserve original?
   - [ ] Implementation: Add to `TerminalGuard` based on decision

4. **Screen buffer strategy** 
   - [ ] Experiment: Run TimerTerm with alternate screen buffer
   - [ ] Question: Does this defeat the purpose of "persistent timer while working"?
   - [ ] Test case: What if user wants to copy text from their session?
   - [ ] Decision point: Should this be a flag like `--separate-screen`?

### Platform Abstraction Design
**Goal**: Design trait system for future macOS/Windows support

#### Tasks:
1. **Design Platform trait**
   - [ ] Define core operations needed across platforms
   - [ ] Separate Unix-specific from generic operations
   - [ ] Create `mod platform` with conditional compilation

2. **macOS Verification**
   - [ ] Test current libc implementation on macOS
   - [ ] Document any BSD vs Linux differences
   - [ ] Adjust Platform trait if needed

3. **Windows Research** 
   - [ ] Research Windows Console API for status lines
   - [ ] Prototype minimal Windows status bar
   - [ ] Design completely different approach for Windows
   - [ ] Consider if Windows Terminal's new features help

---

## Research Notes

### Terminal State Complexity
**Finding**: Terminal state involves more than just termios:
- Scrolling region
- Cursor visibility
- Cursor position
- Color settings
- Alternate screen buffer
- Character set (G0/G1)
- Terminal title

**Question**: How much state do we actually need to save/restore?

### Exit Mode Use Cases
**Use Case 1**: User runs compiler with timer
- Wants to see compiler output after timer ends
- Preference: Preserve session

**Use Case 2**: User doing focused work session
- Timer is just for time awareness
- Preference: Clean exit or restore previous

**Use Case 3**: User runs timer for cooking while coding
- Wants to continue working after timer
- Preference: Smart exit (clear timer, keep work)

**TODO**: Survey potential users for their preferences

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