# CPU Monitor

A real-time CPU monitoring application built with Rust.

## Features

- **Real-time CPU monitoring**: Tracks overall CPU usage and individual core performance
- **Visual graphs**: Line charts showing CPU usage over time with exponential moving averages
- **Process tracking**: Displays the current number of running processes
- **System information**: Shows OS details, architecture, and hostname
- **Color-coded indicators**: Green for low usage, yellow for medium, red for high

## Screenshots in reverse chronological order

![1.0 Finalized Version](Screenshots/1.0_Version.png)
![Added new workers and process count](Screenshots/Cores_with_process_count_and_os_info.png)
![Added progress bar and scrolling mechanism](Screenshots/Progress_bar_and_scrolling.png)
![Added_colour and dotted lines](Screenshots/Cores_with_color_and_dotted_line.png)
![Changed layout and added ema functionality](Screenshots/Cores_with_ema_and_new_layout.png)
![Cores with per core rolling graphs](Screenshots/Overall_and_per_core_rolling.png)
![Cores with progress bar](Screenshots/Cores_with_progress_bar_1.png)

## Requirements

- Rust 1.70 or later
- Windows/Linux/macOS (any platform supported by sysinfo)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/cpu-monitor.git
   cd cpu-monitor
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application:
```bash
cargo run --release
```

The GUI will open showing:
- Overall CPU usage with progress bar and graph
- Per-core CPU usage for each processor
- Current process count
- System information panel

Snapshots update every 100ms. If you wish to change this, change the const REFRESH_MILLISECONDS in src/config/app_variables.

## Architecture

The application is structured as follows:

- **Main entry point** (`src/main.rs`): Sets up communication channels and starts background workers
- **App logic** (`src/app/`): Handles GUI updates, state management, and data processing
- **Workers** (`src/workers/`): Background threads collecting CPU, process, and system data
- **Snapshots** (`src/snapshots/`): Data structures for system information
- **Graph rendering** (`src/graph/`): Drawing utilities for charts and progress bars
- **Configuration** (`src/config/`): Constants for layout, styling, and behavior

Data flows from background workers through multi-producer-single-consumer channels to the main GUI thread.

## Dependencies

- `eframe`: For the GUI framework
- `egui`: UI components and rendering
- `sysinfo`: System information collection
- `std::sync::mpsc`: Inter-thread communication

## Known Issues

- On Windows, the GUI is in light mode while on MacOS the GUI is in dark mode.
- On MacOS, the number of processes shown will be different than that in the Activity Monitor, this is because MacOS often bundles together related processes as one.

## License

MIT License

## Acknowledgments

Built with [Rust](https://doc.rust-lang.org/book/), [egui](https://github.com/emilk/egui), and [sysinfo](https://github.com/GuillaumeGomez/sysinfo).</content>
<parameter name="filePath">c:\Users\liujo\OneDrive\Documents\CPU-Monitor\README.md