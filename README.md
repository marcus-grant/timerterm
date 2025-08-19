# TimerTerm

A terminal application that displays a persistent countdown timer at the top of your terminal while maintaining full terminal functionality below.

## Overview

TimerTerm reserves the top few lines of your terminal window to display a countdown timer that remains visible and updates every second, regardless of what commands you run or how much output scrolls by in the main terminal area. Think of it as adding a permanent status bar to your terminal that shows time-critical information.

## Key Features

- **Persistent Timer Display**: Timer stays fixed at the top of your terminal window
- **Non-Intrusive**: Full terminal functionality preserved below the timer
- **Visual Feedback**: Color-coded timer states (green \u2192 yellow \u2192 red as time expires)
- **Multiple Timer Modes**: Countdown, stopwatch, and pomodoro timer support
- **Clean Integration**: Works with your existing terminal workflow
- **Graceful Exit**: Properly restores terminal state when closed

## How It Works

### Terminal Space Reservation

TimerTerm uses ANSI escape sequences to manipulate the terminal's scrolling region. Here's the core mechanism:

1. **Scrolling Region Definition**: The terminal is instructed to only scroll content within a specific region (e.g., lines 4 through bottom), leaving the top lines untouched
2. **Cursor Position Management**: The program saves and restores cursor positions to update the timer without disrupting your work
3. **Parallel Execution**: A background thread updates the timer display while the main thread runs your shell

### Technical Implementation

The program leverages several terminal control techniques:

- **ANSI Escape Sequences**: Platform-independent terminal control codes
- **Terminal Scrolling Regions**: `CSI r` command to define scrollable areas
- **Cursor Manipulation**: Save (`CSI s`), restore (`CSI u`), and position (`CSI H`) commands
- **Threading**: Separate timer updates from main shell interaction
- **Signal Handling**: Proper cleanup on interrupts (Ctrl+C, SIGTERM)

### Architecture

```
\u250c\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2510
\u2502     Timer Display (Line 1)      \u2502 \u2190 Fixed/Non-scrolling
\u2502     Status Bar    (Line 2)      \u2502 \u2190 Fixed/Non-scrolling  
\u2502     Separator     (Line 3)      \u2502 \u2190 Fixed/Non-scrolling
\u251c\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2524
\u2502                                 \u2502
\u2502     Normal Terminal Output      \u2502 \u2190 Scrollable region
\u2502     (Your shell runs here)      \u2502
\u2502                                 \u2502
\u2514\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2518
```

## Installation

```bash
# TODO: Installation instructions for Rust binary
```

## Usage

### Basic Usage

```bash
# TODO: Basic usage examples
```

### Timer Modes

```bash
# TODO: Different timer modes
```

### Configuration

```bash
# TODO: Configuration options
```

## Implementation Language

TimerTerm is being implemented in Rust for:
- Memory safety and performance
- Cross-platform compatibility
- Reliable signal handling
- Efficient threading model
- Single binary distribution

For a detailed explanation of the terminal manipulation mechanisms with working Python prototype code, see [MECHANISM.md](MECHANISM.md).

## Requirements

- Unix-like terminal with ANSI escape sequence support
- Terminal emulator that supports:
  - Scrolling regions
  - Cursor save/restore
  - Color output

## Compatibility

Tested and working on:
- Linux (GNOME Terminal, Konsole, xterm, Alacritty)
- macOS (Terminal.app, iTerm2)
- WSL/WSL2 on Windows
- Most modern terminal emulators

Not compatible with:
- Windows Command Prompt (use WSL instead)
- Very old terminal emulators without ANSI support
- Multiplexers may require special handling (tmux/screen)

## How It Maintains Terminal Functionality

TimerTerm launches your default shell (`$SHELL`) in the scrollable region, which means:

- All your normal commands work as expected
- Command history is preserved
- Shell configurations (.bashrc, .zshrc) are loaded
- Pipes, redirects, and job control work normally
- You can run any program, including full-screen applications (with some limitations)

## Limitations

- Full-screen TUI applications (vim, htop, etc.) may conflict with the reserved space
- Terminal multiplexers (tmux, screen) may need special configuration
- Some terminal emulators may handle scrolling regions differently
- Terminal resize events require special handling

## Contributing

Contributions are welcome! Please see:
- [MECHANISM.md](MECHANISM.md) - Detailed explanation of terminal manipulation with Python prototype
- [SPEC.md](SPEC.md) - Technical specifications for the Rust implementation
- [TODO.md](TODO.md) - Development roadmap and implementation steps

## License

MIT License - See LICENSE file for details