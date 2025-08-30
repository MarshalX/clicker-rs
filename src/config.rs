use crate::constants::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickerConfig {
    pub interval_ms: u64,
    pub min_interval_ms: u64,
    pub delay_mode: DelayMode,
    pub cps: f64,
    pub min_delay_ms: u64,
    pub max_delay_ms: u64,
    pub auto_start: bool,
    pub click_type: ClickType,
    pub click_button: ClickButton,
    pub repeat_mode: RepeatMode,
    pub hotkeys_enabled: bool,
    pub hotkey_combination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DelayMode {
    Cps,
    Jitter,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClickType {
    Single,
    Double,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClickButton {
    Left,
    Right,
    Middle,
}

impl std::fmt::Display for DelayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DelayMode::Cps => write!(f, "CPS (Clicks Per Second)"),
            DelayMode::Jitter => write!(f, "Jitter (Random Delay)"),
        }
    }
}

impl std::fmt::Display for ClickType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickType::Single => write!(f, "Single Click"),
            ClickType::Double => write!(f, "Double Click"),
            ClickType::Hold => write!(f, "Hold"),
        }
    }
}

impl std::fmt::Display for ClickButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickButton::Left => write!(f, "Left"),
            ClickButton::Right => write!(f, "Right"),
            ClickButton::Middle => write!(f, "Middle"),
        }
    }
}

impl DelayMode {
    pub fn all() -> Vec<DelayMode> {
        vec![DelayMode::Cps, DelayMode::Jitter]
    }
}

impl ClickType {
    pub fn all() -> Vec<ClickType> {
        vec![ClickType::Single, ClickType::Double, ClickType::Hold]
    }
}

impl ClickButton {
    pub fn all() -> Vec<ClickButton> {
        vec![ClickButton::Left, ClickButton::Right, ClickButton::Middle]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatMode {
    Continuous,
    Count(u64),
    Duration(u64),
}

impl Default for ClickerConfig {
    fn default() -> Self {
        Self {
            interval_ms: DEFAULT_INTERVAL.parse().unwrap(),
            min_interval_ms: MIN_INTERVAL,
            delay_mode: DelayMode::Cps,
            cps: DEFAULT_CPS,
            min_delay_ms: DEFAULT_MIN_DELAY,
            max_delay_ms: DEFAULT_MAX_DELAY,
            auto_start: false,
            click_type: ClickType::Single,
            click_button: ClickButton::Left,
            repeat_mode: RepeatMode::Continuous,
            hotkeys_enabled: HOTKEY_ENABLED_DEFAULT,
            hotkey_combination: DEFAULT_HOTKEY.to_string(),
        }
    }
}

impl ClickerConfig {
    pub fn is_valid_interval(&self, interval: u64) -> bool {
        interval >= self.min_interval_ms
    }

    pub fn parse_interval_string(&mut self, interval_str: &str) -> Result<(), String> {
        match interval_str.parse::<u64>() {
            Ok(interval) if self.is_valid_interval(interval) => {
                self.interval_ms = interval;
                Ok(())
            }
            Ok(_) => Err(ERROR_INTERVAL_TOO_SMALL.to_string()),
            Err(_) => Err(ERROR_INVALID_FORMAT.to_string()),
        }
    }

    pub fn is_valid_cps(&self, cps: f64) -> bool {
        cps > 0.0 && (1000.0 / cps) >= self.min_interval_ms as f64
    }

    pub fn is_valid_jitter_range(&self, min_delay: u64, max_delay: u64) -> bool {
        min_delay >= self.min_interval_ms && min_delay <= max_delay
    }

    pub fn parse_cps_string(&mut self, cps_str: &str) -> Result<(), String> {
        match cps_str.parse::<f64>() {
            Ok(cps) if self.is_valid_cps(cps) => {
                self.cps = cps;
                Ok(())
            }
            Ok(_) => Err("Error: CPS value would result in interval too small".to_string()),
            Err(_) => Err(ERROR_INVALID_FORMAT.to_string()),
        }
    }

    pub fn set_jitter_range(&mut self, min_delay: u64, max_delay: u64) -> Result<(), String> {
        if self.is_valid_jitter_range(min_delay, max_delay) {
            self.min_delay_ms = min_delay;
            self.max_delay_ms = max_delay;
            Ok(())
        } else {
            Err("Error: Invalid jitter range".to_string())
        }
    }
}
