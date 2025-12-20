# Mastermind

A Mastermind game implementation in Rust organized as a workspace with a core library and multiple frontends.

## Project Structure

This project is organized as a Cargo workspace with the following crates:

- **core**: The main game logic library containing rules, feedback calculation, and game state management
- **tui**: Terminal user interface (planned - not implemented yet)
- **gui**: Graphical user interface (planned - not implemented yet)

## Core Library

The `core` crate handles all the game logic including:

- Game rules validation and configuration
- Secret code generation
- Guess validation and feedback calculation
- Game state management

### Dependencies

- **rand**: Used for generating random secret codes

## Installation

```bash
git clone https://github.com/ilyeshdz/mastermind-rust
cd mastermind-rust
cargo build
```

## Running

Currently, only the core library is implemented. To run tests:

```bash
cargo test
```

## TODO

- [ ] Implement TUI (Terminal User Interface) crate for console-based gameplay
- [ ] Implement GUI crate for graphical interface


Made with ❤️ by Ilyes
