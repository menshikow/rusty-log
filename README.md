# rusty-log

A modern tail -f-style tool for log files with live filtering, coloring, and simple queries.

## Features

- Real-time log file following (tail -f)
- Live filtering
- Syntax highlighting and coloring
- Simple query support

## Project Structure

```
rusty-log/
├── src/
│   ├── main.rs      # Entry point
│   ├── lib.rs       # Library exports
│   ├── tail.rs      # Tail functionality
│   ├── filter.rs    # Filtering logic
│   ├── color.rs     # Coloring/highlighting
│   ├── query.rs     # Query parsing
│   ├── config.rs    # Configuration
│   └── cli.rs       # CLI interface
├── Cargo.toml       # Project manifest
└── README.md        # This file
```

## Building

```bash
cargo build --release
```

## Usage

```bash
rusty-log [OPTIONS] <FILE>
```

