# Autodate

A Windows system tray application that automatically monitors a directory and renames invoice files using a configurable date format based on the current date.

## Features

- **Automatic monitoring**: Watches a specific directory for new files
- **Configurable date format**: Customize the file naming pattern (e.g., `2025-10.pdf`, `2025-10-19.pdf`)
- **System tray icon**: Runs in the background with visual status indicators
- **Auto-start**: Option to run on Windows startup
- **Lightweight and efficient**: Minimal system resource usage

## Technologies

- **Rust** - Core programming language
- **winit** - Event loop and system event handling
- **tray-icon** - System tray icon management
- **notify** - File system monitoring
- **chrono** - Date manipulation and formatting

## Requirements

- Windows 10 or higher
- No external dependencies required

## Installation

### From source

```bash
# Clone the repository
git clone <repository-url>
cd invoices-name

# Build in release mode
cargo build --release

# The executable will be located at: target/release/autodate.exe
```

## Configuration

The application requires a `.env` file in the same directory as the executable with the following environment variables:

### Required Environment Variables

Create a `.env` file with the following configuration:

```env
# Required: Directory path to monitor for new files
WATCH_PATH=C:\\Users\\YourUser\\Documents\\Invoices

# Required: Date format for renaming files (uses chrono format specifiers)
# Examples:
#   %Y-%m       -> 2025-10
#   %Y-%m-%d    -> 2025-10-19
#   %Y%m%d      -> 20251019
FILE_FORMAT=%Y-%m

# Required: Delay in seconds before renaming a file (allows file to finish writing)
DELAY_SECONDS=5
```

### Format Specifiers

Common chrono format specifiers for `FILE_FORMAT`:
- `%Y` - Year with 4 digits (e.g., 2025)
- `%m` - Month as a zero-padded number (01-12)
- `%d` - Day as a zero-padded number (01-31)
- `%H` - Hour in 24h format (00-23)
- `%M` - Minute (00-59)

For more format options, see the [chrono documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

## Usage

1. Create a `.env` file with the required configuration
2. Run `autodate.exe`
3. The application will appear in the system tray
4. Right-click the icon to access the menu:
   - **Open folder**: Opens the monitored directory
   - **Start with Windows**: Enable/disable automatic startup
   - **Exit**: Close the application

## Status Indicators

- **Green**: Application is running correctly
- **Red**: Monitoring or configuration error

## Development

```bash
# Run in debug mode (with console)
cargo run

# Run in release mode (without console)
cargo run --release

# Clean build artifacts
cargo clean
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
