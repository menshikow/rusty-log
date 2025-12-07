# rusty-log

A modern, feature-rich tail -f-style tool for log files with live filtering, intelligent coloring, and powerful query capabilities. Built with Rust for performance and reliability.

## Features

- **Real-time file following** - Watch log files as they're written (tail -f behavior)
- **Automatic syntax highlighting** - Intelligent color coding based on log levels (ERROR, WARN, INFO, DEBUG)
- **Powerful filtering** - Filter lines using regex patterns or simple text queries
- **Pattern highlighting** - Highlight matching patterns with visual emphasis
- **Line numbers** - Optional line number display for easy reference
- **Case-insensitive search** - Text queries work regardless of case
- **High performance** - Built with Rust for fast, efficient log processing
- **Robust error handling** - Graceful handling of file removal and errors

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rusty-log.git
cd rusty-log

# Build the release version
cargo build --release

# The binary will be at target/release/rusty-log
```

### Requirements

- Rust 1.70+ (2021 edition)
- Cargo

## Usage

### Basic Usage

```bash
# Follow a log file (shows last 10 lines by default)
rusty-log /var/log/app.log

# Follow with custom number of initial lines
rusty-log -n 50 /var/log/app.log
```

### Filtering

```bash
# Filter using regex pattern
rusty-log -f "error|fatal" /var/log/app.log

# Simple text query (case-insensitive)
rusty-log -q "database" /var/log/app.log

# Combine with line numbers
rusty-log -f "ERROR" -l /var/log/app.log
```

### Coloring and Highlighting

```bash
# Disable colors (useful for piping)
rusty-log --no-color /var/log/app.log

# Enable pattern highlighting
rusty-log -f "error" -H /var/log/app.log

# Highlight query matches
rusty-log -q "timeout" -H /var/log/app.log
```

### Advanced Examples

```bash
# Monitor errors with line numbers and highlighting
rusty-log -f "error|exception" -l -H /var/log/app.log

# Search for specific user activity
rusty-log -q "user123" -n 100 /var/log/access.log

# Watch multiple patterns (using regex OR)
rusty-log -f "(error|warn|critical)" /var/log/system.log

# Pipe output to another command (with no-color)
rusty-log --no-color /var/log/app.log | grep "specific"
```

## Command-Line Options

```
USAGE:
    rusty-log [OPTIONS] <FILE>

ARGS:
    <FILE>    File to tail

OPTIONS:
    -f, --filter <FILTER>        Filter pattern (regex) - only show lines matching this pattern
    -F, --follow <FOLLOW>        Follow file (tail -f behavior) [default: true]
    -H, --highlight              Highlight matching patterns
    -h, --help                   Print help information
    -l, --line-numbers           Show line numbers
    -n, --lines <LINES>          Number of lines to show initially [default: 10]
        --no-color               Disable coloring
    -q, --query <QUERY>          Query string - simple text search
    -V, --version                Print version information
```

## How It Works

1. **Initial Display**: Reads and displays the last N lines from the file (default: 10)
2. **File Watching**: Uses file system notifications to detect changes
3. **Real-time Processing**: As new lines are written, they are:
   - Filtered (if filter/query is specified)
   - Colorized based on log level detection
   - Highlighted (if pattern matching is enabled)
   - Displayed with optional line numbers

## Log Level Detection

The tool automatically detects and colors log levels:

- **ERROR** (Red, Bold): Lines containing "error", "fatal", "failed", "exception"
- **WARN** (Yellow): Lines containing "warn", "warning", "caution"
- **INFO** (Blue): Lines containing "info", "information"
- **DEBUG** (Cyan): Lines containing "debug", "trace"

## Examples

### Monitoring Application Errors

```bash
rusty-log -f "ERROR|FATAL|Exception" -l -H /var/log/myapp.log
```

### Tracking Specific Events

```bash
rusty-log -q "payment" -n 20 /var/log/transactions.log
```

### Debugging with Full Context

```bash
rusty-log -f "debug" -l /var/log/debug.log
```

### Production Monitoring

```bash
rusty-log -f "(error|warn)" -H /var/log/production.log
```

## Performance

- Efficient file watching using native OS notifications
- Minimal memory footprint
- Fast regex matching with compiled patterns
- Non-blocking I/O for responsive real-time updates

## Error Handling

The tool gracefully handles:
- File removal (stops watching and notifies)
- Permission errors
- Invalid regex patterns (with helpful error messages)
- File system errors

## Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Project Structure

```
rusty-log/
├── src/
│   ├── main.rs      # Entry point
│   ├── lib.rs       # Library exports
│   ├── cli.rs       # CLI argument parsing
│   ├── config.rs    # Configuration management
│   ├── tail.rs      # Tail functionality
│   ├── filter.rs    # Filtering logic
│   ├── color.rs     # Coloring/highlighting
│   └── query.rs     # Query parsing
├── Cargo.toml       # Project manifest
├── LICENSE          # MIT License
└── README.md        # This file
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [notify](https://github.com/notify-rs/notify) for file system watching
- Uses [colored](https://github.com/mackwic/colored) for terminal colors

## Changelog

### v0.1.0
- Initial release
- Real-time file following
- Regex and text query filtering
- Automatic log level detection and coloring
- Pattern highlighting
- Line number support
