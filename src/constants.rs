pub const APP_TITLE: &str = "Clicker";
pub const APP_SUBTITLE: &str = "Rust-powered Universal Autoclicker";

pub const STATUS_READY: &str = "Ready to start - Set interval and click Start";
pub const STATUS_RUNNING: &str = "Autoclicker is running...";
pub const STATUS_STOPPED: &str = "Autoclicker stopped";
pub const STATUS_RESET: &str = "Reset complete";
pub const STATUS_HOTKEY_REGISTERED: &str = "Hotkey registered";
pub const STATUS_HOTKEY_FAILED: &str = "Failed to register hotkey";

pub const ERROR_INTERVAL_TOO_SMALL: &str = "Error: Interval must be at least 10ms";
pub const ERROR_INVALID_FORMAT: &str = "Error: Invalid interval format";
pub const ERROR_CLICK_FAILED: &str = "Failed to click";
pub const ERROR_ENIGO_INIT: &str = "Failed to initialize Enigo";
pub const ERROR_HOTKEY_PARSE: &str = "Invalid hotkey format";
pub const ERROR_HOTKEY_REGISTER: &str = "Failed to register hotkey";

pub const VALIDATION_VALID: &str = "Valid";
pub const VALIDATION_INVALID: &str = "Invalid";

pub const UI_INTERVAL_VALID_TEXT: &str = VALIDATION_VALID;
pub const UI_INTERVAL_INVALID_TEXT: &str = VALIDATION_INVALID;
pub const UI_HOTKEY_VALID_TEXT: &str = VALIDATION_VALID;
pub const UI_HOTKEY_INVALID_TEXT: &str = VALIDATION_INVALID;

pub const FONT_SIZE_TITLE: u16 = 32;
pub const FONT_SIZE_SUBTITLE: u16 = 16;
pub const FONT_SIZE_SMALL: u16 = 12;

pub const UI_TITLE_SIZE: u16 = FONT_SIZE_TITLE;
pub const UI_SUBTITLE_SIZE: u16 = FONT_SIZE_SUBTITLE;
pub const UI_NOTE_SIZE: u16 = FONT_SIZE_SMALL;
pub const UI_VALIDATION_SIZE: u16 = FONT_SIZE_SMALL;

pub const UI_SPACING_LARGE: u16 = 30; // Major sections
pub const UI_SPACING_MEDIUM: u16 = 15; // Between form groups
pub const UI_SPACING_SMALL: u16 = 8; // Within form elements
pub const UI_SPACING_TINY: u16 = 5; // Icon-text spacing

pub const UI_WINDOW_WIDTH: f32 = 400.0;
pub const UI_WINDOW_HEIGHT: f32 = 750.0;
pub const UI_INPUT_WIDTH: f32 = 100.0;
pub const UI_LABEL_WIDTH: f32 = 140.0;
pub const UI_VALIDATION_WIDTH: f32 = 80.0;
pub const UI_CONTAINER_PADDING: u16 = 10;

pub const UI_BUTTON_START: &str = "Start";
pub const UI_BUTTON_STOP: &str = "Stop";
pub const UI_BUTTON_RESET: &str = "Reset";

pub const UI_DELAY_MODE_LABEL: &str = "Delay Mode:";
pub const UI_CPS_LABEL: &str = "CPS:";
pub const UI_MIN_DELAY_LABEL: &str = "Min Delay (ms):";
pub const UI_MAX_DELAY_LABEL: &str = "Max Delay (ms):";
pub const UI_CLICK_BUTTON_LABEL: &str = "Mouse Button:";
pub const UI_CLICK_TYPE_LABEL: &str = "Click Type:";
pub const UI_HOTKEY_LABEL: &str = "Start/Stop Hotkey:";
pub const UI_HOTKEY_ENABLED_LABEL: &str = "Enable Hotkeys";

pub const UI_CPS_PLACEHOLDER: &str = "1.0";
pub const UI_MIN_DELAY_PLACEHOLDER: &str = "100";
pub const UI_MAX_DELAY_PLACEHOLDER: &str = "500";
pub const UI_HOTKEY_PLACEHOLDER: &str = "F6";

pub const DEFAULT_INTERVAL: &str = "1000";
pub const DEFAULT_CPS: f64 = 11.0;
pub const DEFAULT_MIN_DELAY: u64 = 60;
pub const DEFAULT_MAX_DELAY: u64 = 110;
pub const DEFAULT_HOTKEY: &str = "\\";
pub const HOTKEY_ENABLED_DEFAULT: bool = true;

pub const MIN_INTERVAL: u64 = 1;
pub const HOTKEY_POLL_INTERVAL_MS: u64 = 50;

pub const UI_PERMISSION_NOTE: &str = "Note: Make sure to grant accessibility permissions on macOS";
pub const UI_WEBSITE_NOTE: &str = "Visit clicker.rs for updates";
pub const WEBSITE_URL: &str = "https://clicker.rs";

pub const UI_SECTION_DELAY_CONFIG: &str = "Delay Configuration";
pub const UI_SECTION_CLICK_CONFIG: &str = "Click Configuration";
pub const UI_SECTION_HOTKEY_CONFIG: &str = "Hotkey Configuration";

pub const COLOR_SUCCESS: [f32; 3] = [0.0, 0.8, 0.0]; // Green
pub const COLOR_ERROR: [f32; 3] = [0.9, 0.2, 0.2]; // Red
pub const COLOR_WARNING: [f32; 3] = [0.9, 0.7, 0.0]; // Yellow/Orange
pub const COLOR_INFO: [f32; 3] = [0.2, 0.6, 0.9]; // Blue
