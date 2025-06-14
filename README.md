# rust-ff: Terminal-Based File Finder

A lightning-fast, keyboard-driven file search utility for the terminal. Built with Rust and ratatui for optimal performance and a clean user interface.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal-%23121011.svg?style=for-the-badge&logo=gnu-bash&logoColor=white)

## Features

- **Real-time searching** as you type with fast filtering
- **Intuitive keyboard navigation** for quick file selection  
- **VS Code integration** to open files directly
- **Relative path display** for better context
- **Clean terminal UI** with search input, file list and help panel
- **Command line arguments** support for initial search terms

## Installation

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))
- Visual Studio Code (for file opening functionality)

### Build from source

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-ff.git

# Navigate to the project directory
cd rust-ff

# Build with cargo
cargo build --release

# The binary will be available at ./target/release/rust-ff
```

### Usage

```bash
# Basic usage - opens file finder in current directory
rust-ff

# Start with an initial search term
rust-ff document
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Type any text | Filter files in real-time |
| `↑` / `↓` | Navigate through file list |
| `Enter` | Open selected file in VS Code |
| `?` | Toggle help screen |
| `Backspace` | Delete last character in search |
| `Esc` | Exit application |

## How It Works

rust-ff recursively scans the current directory for files matching your search criteria as you type. Files are displayed as relative paths for easier identification, and you can quickly navigate the list to open any file directly in VS Code.

The application uses debounced input handling to maintain responsiveness even when typing quickly.

## Performance

Built with Rust for speed and efficiency, rust-ff is designed to handle large directory structures with minimal resource usage.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Palth
