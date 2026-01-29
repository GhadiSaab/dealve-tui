# Dealve

A terminal UI for browsing the best game deals, powered by [IsThereAnyDeal](https://isthereanydeal.com).

**Dealve** = Deal + Delve

## Features

- Browse top game deals
- Filter by store

## Installation

```bash
cargo install --path tui
```

## Usage

```bash
dealve
```

### Keyboard Shortcuts
- `↑/k` - Move up
- `↓/j` - Move down
- `q/Esc` - Quit
- `r` - Refresh deals

## Development

### Project Structure

```
dealve-tui/
├── core/    # Shared types and domain logic
├── api/     # IsThereAnyDeal API client
└── tui/     # Terminal UI application
```

### Building

```bash
cargo build
```

### Running

```bash
cargo run --bin dealve
```

## Contributing

Contributions are welcome! This project uses:
- Rust 2021 edition
- [Ratatui](https://github.com/ratatui-org/ratatui) for the TUI
- [tokio](https://tokio.rs) for async runtime
- Workspace structure for modularity

## License

Licensed under either of:
- MIT License
- Apache License, Version 2.0

at your option.
