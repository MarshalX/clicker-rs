# Contributing to Clicker

Thank you for considering contributing to the Clicker autoclicker project! This guide will help you get started with development and contribution.

## Prerequisites

Before you begin development, ensure you have:

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- On macOS: Xcode Command Line Tools
- Git for version control

## Getting Started

### Clone and Build
```bash
git clone https://github.com/MarshalX/clicker-rs.git
cd clicker-rs
cargo build --release
```

### Run the Application
```bash
cargo run
```

## Development

### Project Structure
```
clicker/
├── src/
│   ├── main.rs          # UI layer and application coordination
│   ├── clicker.rs       # Core clicking logic with CPS and jitter support
│   ├── config.rs        # Delay modes, validation, and configuration management
│   ├── hotkey.rs        # Global hotkey detection and management
│   ├── constants.rs     # UI text, sizes, and application constants
│   └── lib.rs           # Library interface for the core functionality
├── assets/
│   └── Info.plist       # macOS app metadata and permissions
├── Cargo.toml           # Project dependencies and metadata
├── CONTRIBUTING.md      # This file
└── README.md            # User documentation
```

### Key Components

- **main.rs**: Contains the GUI application built with Iced framework
- **clicker.rs**: Core autoclicker functionality with timer management
- **config.rs**: Configuration management and delay mode implementations
- **hotkey.rs**: Global hotkey detection using the global-hotkey crate
- **constants.rs**: UI constants, sizing, and text definitions

### Dependencies

- **iced**: Cross-platform GUI framework for the user interface
- **enigo**: Cross-platform mouse and keyboard simulation
- **global-hotkey**: Cross-platform global hotkey detection
- **rand**: Random number generation for jitter mode timing

## Building for Different Platforms

### macOS

#### Regular Binary
```bash
# Intel-based Macs
cargo build --release --target x86_64-apple-darwin

# Apple Silicon Macs
cargo build --release --target aarch64-apple-darwin
```

#### macOS App Bundle
For a proper macOS app bundle:

```bash
# Install cargo-bundle (one-time setup)
cargo install cargo-bundle

# Build the app bundle
cargo bundle --release
```

The app bundle will be created in `target/release/bundle/osx/Clicker.app`

### Windows (Cross-compilation from macOS)
```bash
# Install cross-compilation toolchain (one-time setup)
brew install mingw-w64
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### Linux
```bash
# Install cross-compilation toolchain (one-time setup)
brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu

# Build (may fail due to GUI dependencies - recommended to build on Linux directly)
cargo build --release --target x86_64-unknown-linux-gnu
```

## Development Guidelines

### Code Style
- Follow standard Rust formatting using `cargo fmt`
- Run `cargo clippy` to catch common mistakes and improve code quality
- Write clear, descriptive commit messages
- Add comments for complex logic, especially in timing-critical sections

### Testing
- Test your changes on your target platform
- Verify that the GUI responds correctly to user input
- Test hotkey functionality across different scenarios
- Ensure click timing accuracy for both CPS and jitter modes

### Safety Considerations
- Maintain the 10ms minimum delay safety limit
- Ensure proper input validation for all user inputs
- Test accessibility permissions on macOS
- Verify that error handling works correctly

## Contributing Process

1. **Fork the repository** on GitHub
2. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following the guidelines above
4. **Test thoroughly** on your target platform
5. **Commit your changes** with clear, descriptive messages:
   ```bash
   git commit -m "Add: Brief description of your changes"
   ```
6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Open a Pull Request** with:
   - Clear description of changes
   - Screenshots if UI changes are involved
   - Testing information (platform tested, scenarios covered)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
