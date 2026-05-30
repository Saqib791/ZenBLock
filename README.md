# Zenblock

Zenblock is a powerful desktop productivity and focus application designed to help you eliminate distractions and get deep work done. 

## Features

- **Session Control / Website Blocker**: Set a timer and block restricted domains (like YouTube, Instagram, X) to prevent procrastination during your work sessions.
- **Lockdown Protocol**: A strict mode that locks you in and prevents you from prematurely terminating the focus session.
- **Ambient Sound Mixer**: Built-in sound mixer to play soothing background frequencies, helping you stay in the zone.
- **Task Manager**: Track your daily tasks with a built-in calendar and checklist.
- **Advanced Analytics**: Track your performance metrics over time. View your daily focus hours, current streaks, focus heatmaps, and activity trends directly from the dashboard.

## Installation

You can install Zenblock directly on Windows using the provided `.exe` or `.msi` installers.

### Building from Source

To build Zenblock locally, you will need to have [Rust](https://www.rust-lang.org/tools/install) and [Tauri CLI](https://tauri.app/) installed.

1. Clone the repository
2. Install dependencies (if any frontend dependencies are added)
3. Run the build command:
```bash
cargo tauri build
```

This will produce the standalone executable and installer files inside `src-tauri/target/release/bundle`.

## Technology Stack

- **Tauri**: Rust-based framework for building desktop applications.
- **Vanilla JS/HTML/CSS**: Fast and lightweight frontend with a modern "Hacker" aesthetic (Neon Protocol theme).
- **Chart.js**: For beautiful analytics and activity trend graphs.
