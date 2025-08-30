# Clicker - Universal Autoclicker

A cross-platform autoclicker built with Rust, designed for any game or application. Built using the [Iced](https://github.com/iced-rs/iced) GUI framework and [Enigo](https://github.com/enigo-rs/enigo) for precise mouse emulation.

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

## Installation

### Download Pre-built Releases

The easiest way to get started is to download a pre-built binary from the [GitHub Releases page](https://github.com/MarshalX/clicker-rs/releases). 

Available for:
- **macOS**: Universal binary and app bundle
- **Windows**: Executable binary
- **Linux**: Binary for common distributions

Simply download the appropriate file for your platform and run it directly.

### Build from Source

If you prefer to build from source or need to customize the application:

#### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- On macOS: Xcode Command Line Tools

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

### Security & Gatekeeper Issues

When running unsigned applications on macOS, you may encounter security warnings. Here are solutions for common issues:

#### "Application is Damaged and Can't Be Opened"

If you see this error message, it's usually due to macOS quarantine attributes. Fix it by running:
```bash
xattr -c /path/to/Clicker.app
```
For example:
```bash
xattr -c ./target/release/bundle/osx/Clicker.app
```

#### "Cannot Open Because Developer Cannot Be Verified"
For unsigned applications, macOS will show this warning on first launch:

1. **First attempt**: Try to open the app normally - you'll get the warning dialog
2. **Go to System Settings**: Open **System Settings** → **Privacy & Security**
3. **Find the blocked app**: Scroll down to the "Security" section where you'll see a message about the blocked application
4. **Click "Open Anyway"**: Click the **"Open Anyway"** button next to the app warning
5. **Confirm**: In the confirmation dialog, click **"Open"** to permanently allow the application

Alternative method:
1. **Right-click** the application in Finder
2. **Hold Option key** and select **"Open"** from the context menu
3. **Click "Open"** in the dialog to confirm

These steps only need to be done once.

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

## Contributing

We welcome contributions to the Clicker project! For detailed information on how to contribute, including development setup, building for different platforms, and project structure, please see [CONTRIBUTING.md](CONTRIBUTING.md).

## Legal Notice

This software is intended for educational purposes and legitimate automation needs. Users are responsible for ensuring their use complies with the terms of service of any games or applications they use this tool with. The developers are not responsible for any account bans, penalties, or violations resulting from the use of this software.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
