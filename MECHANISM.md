# MECHANISM.md - Terminal Space Reservation Explained

This document provides a detailed explanation of how TimerTerm manipulates the terminal to create a persistent timer display, along with a working Python prototype for reference.

## Core Concept: Terminal Scrolling Regions

Most modern terminals support ANSI escape sequences that allow you to define which portion of the screen scrolls. This is the key to creating a persistent header that doesn't scroll away.

### How Scrolling Regions Work

1. **Normal Terminal Behavior**: By default, when output reaches the bottom of the terminal, everything scrolls up and the top line disappears.

2. **Modified Behavior with Scrolling Regions**: We can tell the terminal "only scroll lines 4 through 24" (for example), leaving lines 1-3 untouched.

## ANSI Escape Sequences Used

### Setting the Scrolling Region
```
\033[{top};{bottom}r
```
- `\033[` - ESC character followed by `[` (Control Sequence Introducer)
- `{top}` - First line of scrollable area (1-indexed)
- `{bottom}` - Last line of scrollable area
- `r` - Set scrolling region command

Example: `\033[4;24r` makes only lines 4-24 scroll, preserving lines 1-3.

### Cursor Movement
```
\033[{row};{col}H    # Move cursor to specific position
\033[s               # Save cursor position
\033[u               # Restore saved cursor position
```

### Line Clearing
```
\033[K               # Clear from cursor to end of line
\033[2K              # Clear entire line
```

### Visual Attributes
```
\033[1m              # Bold
\033[31m             # Red text
\033[32m             # Green text
\033[33m             # Yellow text
\033[0m              # Reset all attributes
```

### Cursor Visibility
```
\033[?25l            # Hide cursor
\033[?25h            # Show cursor
```

## Working Python Prototype

Here's a fully functional prototype that demonstrates all the mechanisms:

```python
#!/usr/bin/env python3

import sys
import time
import threading
import os
import signal
import termios
import tty

class TimerTerm:
    def __init__(self, duration_seconds=300):
        """
        Initialize TimerTerm with a duration in seconds.
        
        The timer will reserve the top 3 lines of the terminal:
        - Line 1: Timer display
        - Line 2: Status/progress bar
        - Line 3: Separator
        """
        self.duration = duration_seconds
        self.start_time = time.time()
        self.running = True
        self.reserved_lines = 3
        
        # Save terminal settings for restoration
        self.old_settings = termios.tcgetattr(sys.stdin)
        
        # Setup signal handlers for graceful cleanup
        signal.signal(signal.SIGINT, self.cleanup_handler)
        signal.signal(signal.SIGTERM, self.cleanup_handler)
        
    def cleanup_handler(self, signum, frame):
        """Handle signals for graceful shutdown"""
        self.cleanup()
        sys.exit(0)
        
    def get_terminal_size(self):
        """Get current terminal dimensions"""
        # Use stty size command - more reliable than some methods
        size = os.popen('stty size', 'r').read().split()
        if len(size) == 2:
            return int(size[0]), int(size[1])  # rows, cols
        return 24, 80  # fallback defaults
        
    def setup_terminal(self):
        """
        Configure the terminal for our display.
        
        This is where the magic happens:
        1. Clear the screen
        2. Set scrolling region to exclude top lines
        3. Hide cursor for cleaner display
        4. Position cursor in the working area
        """
        rows, cols = self.get_terminal_size()
        
        # Clear entire screen
        sys.stdout.write('\033[2J')
        
        # CRITICAL: Set scrolling region
        # This excludes our reserved lines from scrolling
        # Format: \033[{top};{bottom}r
        # We want scrolling from line (reserved_lines + 1) to bottom
        scroll_top = self.reserved_lines + 1
        sys.stdout.write(f'\033[{scroll_top};{rows}r')
        
        # Hide cursor for cleaner timer display
        sys.stdout.write('\033[?25l')
        
        # Move cursor to start of scrollable area
        sys.stdout.write(f'\033[{scroll_top};1H')
        
        # Force flush to ensure all escape sequences are sent
        sys.stdout.flush()
        
    def format_time(self, seconds):
        """Convert seconds to HH:MM:SS format"""
        hours = seconds // 3600
        minutes = (seconds % 3600) // 60
        secs = seconds % 60
        return f"{hours:02d}:{minutes:02d}:{secs:02d}"
        
    def get_progress_bar(self, elapsed, total, width=50):
        """Create a visual progress bar"""
        if total == 0:
            return "=" * width
        
        progress = min(1.0, elapsed / total)
        filled = int(width * progress)
        empty = width - filled
        
        return f"[{'=' * filled}{' ' * empty}]"
        
    def update_timer(self):
        """
        Background thread that updates the timer display.
        
        This runs independently of the main shell, updating every second.
        """
        while self.running:
            current_time = time.time()
            elapsed = int(current_time - self.start_time)
            remaining = max(0, self.duration - elapsed)
            
            rows, cols = self.get_terminal_size()
            
            # IMPORTANT: Save current cursor position
            # This preserves where the user is typing
            sys.stdout.write('\033[s')
            
            # Move to line 1 for timer display
            sys.stdout.write('\033[1;1H')
            sys.stdout.write('\033[K')  # Clear line
            
            # Determine color based on time remaining
            if remaining == 0:
                color = '\033[1;31m'  # Bold red
                timer_text = "\u23f0 TIMER EXPIRED!"
            elif remaining < 60:
                color = '\033[1;31m'  # Red - critical
                timer_text = f"\u23f0 TIMER: {self.format_time(remaining)} - LAST MINUTE!"
            elif remaining < 300:
                color = '\033[1;33m'  # Yellow - warning
                timer_text = f"\u23f0 TIMER: {self.format_time(remaining)}"
            else:
                color = '\033[1;32m'  # Green - normal
                timer_text = f"\u23f0 TIMER: {self.format_time(remaining)}"
            
            # Center the timer text
            padding = (cols - len(timer_text)) // 2
            sys.stdout.write(f"{color}{' ' * padding}{timer_text}\033[0m")
            
            # Move to line 2 for progress bar
            sys.stdout.write('\033[2;1H')
            sys.stdout.write('\033[K')  # Clear line
            
            # Draw progress bar
            bar_width = min(cols - 10, 50)
            progress_bar = self.get_progress_bar(elapsed, self.duration, bar_width)
            bar_padding = (cols - len(progress_bar)) // 2
            sys.stdout.write(f"{' ' * bar_padding}{progress_bar}")
            
            # Move to line 3 for separator
            sys.stdout.write('\033[3;1H')
            sys.stdout.write('\033[K')  # Clear line
            sys.stdout.write('\033[1;36m' + '\u2500' * cols + '\033[0m')
            
            # CRITICAL: Restore cursor position
            # This returns cursor to where user was typing
            sys.stdout.write('\033[u')
            
            # Flush output to ensure immediate display
            sys.stdout.flush()
            
            # Ring terminal bell on expiration (once)
            if remaining == 0 and elapsed == self.duration:
                sys.stdout.write('\a')
                sys.stdout.flush()
            
            # Update every second
            time.sleep(1)
            
    def run_shell(self):
        """
        Launch the user's shell in the scrollable region.
        
        This runs in the main thread while timer updates in background.
        """
        # Get user's preferred shell or default to bash
        shell = os.environ.get('SHELL', '/bin/bash')
        
        # Print welcome message in scrollable area
        print("TimerTerm started! Terminal ready for use.")
        print("Timer is running in the header. Press Ctrl+C to exit.")
        print()
        sys.stdout.flush()
        
        # Launch shell as subprocess
        # This gives the user a full interactive shell
        os.system(shell)
        
    def cleanup(self):
        """
        Restore terminal to original state.
        
        This is critical for leaving the terminal usable after exit.
        """
        self.running = False
        time.sleep(0.1)  # Allow update thread to finish
        
        # Reset scrolling region to full screen
        sys.stdout.write('\033[r')
        
        # Show cursor again
        sys.stdout.write('\033[?25h')
        
        # Clear screen and move to top
        sys.stdout.write('\033[2J\033[1;1H')
        
        # Restore original terminal settings
        termios.tcsetattr(sys.stdin, termios.TCSADRAIN, self.old_settings)
        
        sys.stdout.flush()
        print("TimerTerm closed. Terminal restored.")
        
    def start(self):
        """Main entry point - orchestrates the entire process"""
        try:
            # 1. Setup terminal with reserved space
            self.setup_terminal()
            
            # 2. Start timer update thread
            timer_thread = threading.Thread(target=self.update_timer)
            timer_thread.daemon = True  # Dies when main thread dies
            timer_thread.start()
            
            # 3. Run interactive shell in main thread
            self.run_shell()
            
        finally:
            # 4. Always cleanup, even on errors
            self.cleanup()

# Entry point
if __name__ == "__main__":
    import sys
    
    # Parse duration from command line
    duration = 300  # Default 5 minutes
    
    if len(sys.argv) > 1:
        arg = sys.argv[1]
        try:
            if ':' in arg:
                # Parse MM:SS or HH:MM:SS format
                parts = arg.split(':')
                if len(parts) == 2:
                    duration = int(parts[0]) * 60 + int(parts[1])
                elif len(parts) == 3:
                    duration = int(parts[0]) * 3600 + int(parts[1]) * 60 + int(parts[2])
            else:
                # Parse as seconds
                duration = int(arg)
        except ValueError:
            print(f"Invalid duration: {arg}")
            print("Usage: timerterm.py [seconds | MM:SS | HH:MM:SS]")
            sys.exit(1)
    
    # Create and start timer
    timer = TimerTerm(duration)
    timer.start()
```

## Step-by-Step Execution Flow

### 1. Initialization Phase
```python
timer = TimerTerm(duration)
```
- Store duration and start time
- Save current terminal settings
- Register signal handlers for cleanup

### 2. Terminal Setup Phase
```python
self.setup_terminal()
```
- Clear screen: `\033[2J`
- Define scrolling region: `\033[4;{rows}r` (preserve top 3 lines)
- Hide cursor: `\033[?25l`
- Position cursor in work area: `\033[4;1H`

### 3. Timer Thread Launch
```python
timer_thread = threading.Thread(target=self.update_timer)
timer_thread.start()
```
The timer thread continuously:
- Saves cursor position: `\033[s`
- Updates lines 1-3 with timer info
- Restores cursor position: `\033[u`
- Sleeps for 1 second

### 4. Shell Execution
```python
os.system(shell)
```
- Launches user's shell in main thread
- Shell operates in scrollable region (line 4 onwards)
- User interacts normally, unaware of timer updates

### 5. Cleanup Phase
When user exits or Ctrl+C is pressed:
- Stop timer thread
- Reset scrolling region: `\033[r`
- Show cursor: `\033[?25h`
- Clear screen
- Restore terminal settings

## Key Insights

### Why This Works
1. **Scrolling regions are terminal-native**: The terminal emulator itself handles keeping the reserved lines in place
2. **Cursor save/restore is atomic**: The terminal buffers these operations, preventing visual glitches
3. **Threading isolation**: Timer updates and shell operations don't interfere with each other

### Critical Implementation Details
1. **Always flush stdout**: Escape sequences must be sent immediately
2. **Save/restore cursor religiously**: Any update to reserved space must preserve user's cursor position
3. **Handle signals properly**: Terminal must be restored even on unexpected exit
4. **Use daemon threads**: Ensures timer thread dies with main program

### Common Pitfalls to Avoid
1. **Forgetting to restore terminal state**: Leaves terminal in unusable state
2. **Not flushing output**: Causes delayed or missing updates
3. **Incorrect scrolling region math**: Off-by-one errors break the display
4. **Missing signal handlers**: Ctrl+C leaves terminal broken

## Testing the Mechanism

To verify the mechanism works correctly:

```bash
# Run the prototype
python3 timerterm.py 30

# Test scrolling - should not affect timer
seq 1 100

# Test long-running commands - timer continues
sleep 10

# Test command interaction - timer doesn't interfere
ls -la | grep python

# Test Ctrl+C - should restore terminal properly
# Press Ctrl+C
```

## Adapting to Rust

When implementing in Rust, the same ANSI sequences apply. Key Rust considerations:

1. Use `crossterm` or `termion` crate for terminal manipulation
2. Use `std::thread` for timer updates
3. Use `signal-hook` for proper signal handling
4. Use `nix::unistd::execvp()` or `std::process::Command` for shell execution
5. Implement `Drop` trait for automatic cleanup

The core mechanism remains identical - only the implementation language changes.