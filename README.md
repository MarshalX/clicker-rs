# Clicker - Universal Autoclicker

A cross-platform autoclicker built with Rust, designed for any game or application. Built using the [Iced](https://github.com/iced-rs/iced) GUI framework and [Enigo](https://github.com/enigo-rs/enigo) for precise mouse emulation.

**Official Website:** [clicker.rs](https://clicker.rs)

> ⚠️ Use this autoclicker responsibly and ethically. Many online games and services have rules against automation tools. Always check and comply with the terms of service and rules of any game or application before using automation software. This tool should only be used where explicitly permitted or in offline/local environments.

## Features

- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Simple and intuitive GUI interface built with Rust
- ✅ **Fully functional mouse clicking** with Enigo integration
- ✅ **Two delay modes**: CPS (Clicks Per Second) and Jitter (Random Delay)
- ✅ **CPS Mode**: Set exact clicks per second (e.g., 2.5 CPS = 400ms constant delay)
- ✅ **Jitter Mode**: Random delay between min/max values for unpredictable timing
- ✅ Configurable click intervals (minimum 10ms)
- ✅ Simple running/stopped state tracking
- ✅ Start/Stop controls with validation
- ✅ Error handling and status messages
- ✅ macOS accessibility permissions handling
- ✅ Input validation with visual feedback
- ✅ **Global hotkey support** with configurable keys

## Demo

<img alt="Demo of clicker.rs" src="https://github.com/MarshalX/clicker-rs/raw/main/.github/images/demo.png">

## Delay Modes

The autoclicker supports two distinct delay modes to suit different use cases:

### 🎯 CPS Mode (Clicks Per Second)
- **Constant timing**: Provides consistent, predictable click intervals
- **Easy configuration**: Simply set the desired clicks per second (e.g., 11.5 CPS)
- **Automatic conversion**: CPS is automatically converted to milliseconds (1000ms ÷ CPS)
- **Use case**: Perfect for tasks requiring steady, uniform clicking

### 🎲 Jitter Mode (Random Delay)
- **Variable timing**: Random delay between each click within your specified range
- **Anti-detection**: Mimics human-like clicking patterns with unpredictable timing
- **Customizable range**: Set minimum and maximum delay values (e.g., 100ms - 500ms)
- **Use case**: Ideal for scenarios where you want to avoid detection patterns

## Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- On macOS: Xcode Command Line Tools

## Building and Running

### Clone and Build
```bash
git clone https://github.com/MarshalX/clicker-rs.git
cd clicker
cargo build --release
```

### Run the Application
```bash
cargo run
```

## macOS Setup

### Accessibility Permissions
On macOS, the application requires accessibility permissions to simulate mouse clicks:

1. Run the application once
2. Go to **System Preferences** → **Security & Privacy** → **Privacy** → **Accessibility**
3. Click the lock icon and enter your password
4. Add **Clicker** to the list of allowed applications
5. Restart the application

### Building macOS App Bundle
For a proper macOS app bundle with correct permissions:

```bash
# Install cargo-bundle
cargo install cargo-bundle

# Build the app bundle
cargo bundle --release
```

The app bundle will be created in `target/release/bundle/osx/Clicker.app`

## Usage

1. **Choose Delay Mode**: Select between CPS mode or Jitter mode
   - **CPS Mode**: For consistent, predictable timing
   - **Jitter Mode**: For random, human-like timing

2. **Configure Timing**:
   
   **For CPS Mode:**
   - Set your desired clicks per second (e.g., 2.5 CPS)
   - The app automatically converts this to delay (2.5 CPS = 400ms delay)
   - Minimum effective CPS depends on the 10ms safety limit
   
   **For Jitter Mode:**
   - Set minimum delay (e.g., 100ms)
   - Set maximum delay (e.g., 500ms)
   - Each click will have a random delay between these values
   - Both values must be at least 10ms for safety

3. **Start Clicking**: Click the "Start" button to begin the autoclicker
   - Button is disabled if the timing configuration is invalid
   - Real-time status updates show current state
   - Current delay info is displayed (e.g., "2.5 CPS" or "Jitter: 100ms - 500ms")

4. **Stop Clicking**: Click the "Stop" button to halt the autoclicker

5. **Configure Hotkey**: Set a global hotkey (default: F6) for start/stop
   - Supports function keys (F1-F12) and letter keys (A-Z)
   - Toggle "Enable Hotkeys" checkbox to activate
   - The app shows ✓ for valid hotkeys or ⚠ for invalid ones

6. **Use Hotkeys**: Press your configured hotkey anywhere to start/stop

7. **Reset**: Click "Reset" to stop the autoclicker and return to ready state

8. **Monitor Status**: View real-time status messages showing current state

### Safety Features
- **Input validation**: Prevents invalid CPS values, jitter ranges, and invalid hotkeys
- **Minimum delay enforcement**: All delay modes respect the 10ms minimum safety limit
- **CPS validation**: Prevents CPS values that would result in unsafe intervals
- **Jitter range validation**: Ensures min ≤ max and both values ≥ 10ms
- **Left mouse clicks only**: Safe for most applications
- **Current cursor position**: Clicks wherever your mouse is positioned
- **Easy start/stop**: Immediate response for quick disabling via GUI or hotkey
- **Global hotkeys**: Quick emergency stop with configurable keys
- **Error handling**: Clear error messages for troubleshooting
- **Visual feedback**: Status messages and validation indicators

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
└── README.md            # This file
```

### Dependencies
- **iced**: Cross-platform GUI framework
- **enigo**: Cross-platform mouse and keyboard simulation
- **global-hotkey**: Cross-platform global hotkey detection
- **rand**: Random number generation for jitter mode

### Building for Different Platforms

#### macOS
```bash
# Regular binary
cargo build --release --target x86_64-apple-darwin
# or for Apple Silicon:
cargo build --release --target aarch64-apple-darwin

# macOS App Bundle (requires cargo-bundle)
cargo install cargo-bundle
cargo bundle --release
```

#### Windows (Cross-compilation from macOS)
```bash
# Install cross-compilation toolchain (one-time setup)
brew install mingw-w64
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

#### Linux
```bash
# Install cross-compilation toolchain (one-time setup)
brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu

# Build (may fail due to GUI dependencies)
cargo build --release --target x86_64-unknown-linux-gnu
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Legal Notice

This software is intended for educational purposes and legitimate automation needs. Users are responsible for ensuring their use complies with the terms of service of any games or applications they use this tool with. The developers are not responsible for any account bans, penalties, or violations resulting from the use of this software.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
