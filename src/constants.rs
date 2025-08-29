pub const APP_TITLE: &str = "Clicker";
pub const APP_SUBTITLE: &str = "Rust-powered Universal Autoclicker";

pub const STATUS_READY: &str = "Ready to start - Set interval and click Start";
pub const STATUS_RUNNING: &str = "Autoclicker is running...";
pub const STATUS_STOPPED: &str = "Autoclicker stopped";
pub const STATUS_RESET: &str = "Reset complete";

pub const ERROR_INTERVAL_TOO_SMALL: &str = "Error: Interval must be at least 10ms";
pub const ERROR_INVALID_FORMAT: &str = "Error: Invalid interval format";
pub const ERROR_CLICK_FAILED: &str = "Failed to click";
pub const ERROR_ENIGO_INIT: &str = "Failed to initialize Enigo";

pub const UI_INTERVAL_VALID: &str = "✓";
pub const UI_INTERVAL_INVALID: &str = "⚠ Min: 10ms";

pub const UI_BUTTON_START: &str = "Start";
pub const UI_BUTTON_STOP: &str = "Stop";
pub const UI_BUTTON_RESET: &str = "Reset";

pub const UI_PERMISSION_NOTE: &str = "Note: Make sure to grant accessibility permissions on macOS";
pub const UI_WEBSITE_NOTE: &str = "Visit clicker.rs for updates";

pub const DEFAULT_INTERVAL: &str = "1000";
pub const MIN_INTERVAL: u64 = 10;

pub const UI_HOTKEY_LABEL: &str = "Start/Stop Hotkey:";
pub const UI_HOTKEY_ENABLED_LABEL: &str = "Enable Hotkeys";
pub const UI_HOTKEY_PLACEHOLDER: &str = "F6";
pub const UI_HOTKEY_VALID: &str = "✓";
pub const UI_HOTKEY_INVALID: &str = "⚠ Invalid";

pub const DEFAULT_HOTKEY: &str = "\\";
pub const HOTKEY_ENABLED_DEFAULT: bool = true;
pub const HOTKEY_POLL_INTERVAL_MS: u64 = 50;

pub const STATUS_HOTKEY_REGISTERED: &str = "Hotkey registered";
pub const STATUS_HOTKEY_FAILED: &str = "Failed to register hotkey";
pub const ERROR_HOTKEY_PARSE: &str = "Invalid hotkey format";
pub const ERROR_HOTKEY_REGISTER: &str = "Failed to register hotkey";

pub const UI_TITLE_SIZE: u16 = 32;
pub const UI_SUBTITLE_SIZE: u16 = 16;
pub const UI_STATUS_SIZE: u16 = 14;
pub const UI_NOTE_SIZE: u16 = 12;
pub const UI_VALIDATION_SIZE: u16 = 12;

pub const UI_SPACING_LARGE: u16 = 30;
pub const UI_SPACING_MEDIUM: u16 = 20;
pub const UI_SPACING_SMALL: u16 = 10;

pub const UI_INPUT_WIDTH: f32 = 100.0;
pub const UI_LABEL_WIDTH: f32 = 140.0;
pub const UI_VALIDATION_WIDTH: f32 = 80.0;

pub const UI_WINDOW_WIDTH: f32 = 400.0;
pub const UI_WINDOW_HEIGHT: f32 = 750.0;

pub const UI_CLICK_BUTTON_LABEL: &str = "Mouse Button:";
pub const UI_CLICK_TYPE_LABEL: &str = "Click Type:";

pub const UI_DELAY_MODE_LABEL: &str = "Delay Mode:";
pub const UI_CPS_LABEL: &str = "CPS (Clicks Per Second):";
pub const UI_CPS_PLACEHOLDER: &str = "1.0";
pub const UI_MIN_DELAY_LABEL: &str = "Min Delay (ms):";
pub const UI_MAX_DELAY_LABEL: &str = "Max Delay (ms):";
pub const UI_MIN_DELAY_PLACEHOLDER: &str = "100";
pub const UI_MAX_DELAY_PLACEHOLDER: &str = "500";

pub const DEFAULT_CPS: f64 = 11.0;
pub const DEFAULT_MIN_DELAY: u64 = 60;
pub const DEFAULT_MAX_DELAY: u64 = 110;
