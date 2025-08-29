use crate::config::{ClickButton, ClickType, ClickerConfig, DelayMode};
use crate::constants::*;
use enigo::{Button, Enigo, Mouse, Settings};

#[derive(Debug, Clone)]
pub enum ClickerMessage {
    Tick,
    ClickError(String),
}

#[derive(Clone)]
pub struct Clicker {
    config: ClickerConfig,
    is_running: bool,
}

impl Clicker {
    pub fn new(config: ClickerConfig) -> Self {
        Self {
            config,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn config(&self) -> &ClickerConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut ClickerConfig {
        &mut self.config
    }

    pub fn update_config(&mut self, config: ClickerConfig) {
        self.config = config;
    }

    pub async fn perform_click(&self) -> ClickerMessage {
        if !self.is_running {
            return ClickerMessage::Tick;
        }

        match self.execute_click() {
            Ok(_) => {
                let delay = self.config.to_duration();
                std::thread::sleep(delay);
                ClickerMessage::Tick
            }
            Err(e) => ClickerMessage::ClickError(e),
        }
    }

    fn execute_click(&self) -> Result<(), String> {
        let mut enigo =
            Enigo::new(&Settings::default()).map_err(|e| format!("{}: {}", ERROR_ENIGO_INIT, e))?;

        let button = self.get_enigo_button();

        match self.config.click_type {
            ClickType::Single => {
                enigo
                    .button(button, enigo::Direction::Click)
                    .map_err(|e| format!("{}: {}", ERROR_CLICK_FAILED, e))?;
            }
            ClickType::Double => {
                enigo
                    .button(button, enigo::Direction::Click)
                    .map_err(|e| format!("{}: {}", ERROR_CLICK_FAILED, e))?;
                std::thread::sleep(std::time::Duration::from_millis(50));
                enigo
                    .button(button, enigo::Direction::Click)
                    .map_err(|e| format!("{}: {}", ERROR_CLICK_FAILED, e))?;
            }
            ClickType::Hold => {
                enigo
                    .button(button, enigo::Direction::Press)
                    .map_err(|e| format!("{}: {}", ERROR_CLICK_FAILED, e))?;
                std::thread::sleep(std::time::Duration::from_millis(100));
                enigo
                    .button(button, enigo::Direction::Release)
                    .map_err(|e| format!("{}: {}", ERROR_CLICK_FAILED, e))?;
            }
        }

        Ok(())
    }

    fn get_enigo_button(&self) -> Button {
        match self.config.click_button {
            ClickButton::Left => Button::Left,
            ClickButton::Right => Button::Right,
            ClickButton::Middle => Button::Middle,
        }
    }

    pub fn get_delay_info(&self) -> String {
        match self.config.delay_mode {
            DelayMode::CPS => {
                format!("{:.1} CPS", self.config.cps)
            }
            DelayMode::Jitter => {
                format!(
                    "Jitter: {}ms - {}ms",
                    self.config.min_delay_ms, self.config.max_delay_ms
                )
            }
        }
    }
}
