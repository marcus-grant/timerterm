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