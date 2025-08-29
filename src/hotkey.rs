use crate::constants::*;
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use iced::futures::SinkExt;
use iced::{stream::channel, Subscription};

pub struct HotkeyManager {
    manager: Option<GlobalHotKeyManager>,
    current_hotkey: Option<HotKey>,
    enabled: bool,
    combination: String,
}

impl Clone for HotkeyManager {
    fn clone(&self) -> Self {
        Self {
            manager: GlobalHotKeyManager::new().ok(),
            current_hotkey: self.current_hotkey,
            enabled: self.enabled,
            combination: self.combination.clone(),
        }
    }
}

impl HotkeyManager {
    pub fn new() -> Self {
        Self {
            manager: GlobalHotKeyManager::new().ok(),
            current_hotkey: None,
            enabled: false,
            combination: DEFAULT_HOTKEY.to_string(),
        }
    }

    pub fn with_config(combination: String, enabled: bool) -> Self {
        let mut manager = Self::new();
        manager.combination = combination;
        manager.enabled = enabled;

        if let Ok(hotkey) = Self::parse_hotkey(&manager.combination) {
            manager.current_hotkey = Some(hotkey);
            if enabled {
                manager.register_current_hotkey();
            }
        }

        manager
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_combination(&self) -> &str {
        &self.combination
    }

    pub fn get_current_hotkey(&self) -> Option<&HotKey> {
        self.current_hotkey.as_ref()
    }

    pub fn parse_hotkey(combination: &str) -> Result<HotKey, String> {
        HotKey::try_from(combination).map_err(|_| ERROR_HOTKEY_PARSE.to_string())
    }

    pub fn is_valid_hotkey(combination: &str) -> bool {
        HotKey::try_from(combination).is_ok()
    }

    pub fn set_enabled(&mut self, enabled: bool) -> Result<String, String> {
        if enabled == self.enabled {
            return Ok(self.get_status_message());
        }

        if enabled {
            self.enabled = true;
            if self.register_current_hotkey() {
                Ok(STATUS_HOTKEY_REGISTERED.to_string())
            } else {
                Err(STATUS_HOTKEY_FAILED.to_string())
            }
        } else {
            self.unregister_current_hotkey();
            self.enabled = false;
            Ok("Hotkeys disabled".to_string())
        }
    }

    pub fn update_combination(&mut self, new_combination: String) -> Result<String, String> {
        let parsed_hotkey = Self::parse_hotkey(&new_combination)?;

        if self.enabled {
            self.unregister_current_hotkey();
        }

        self.combination = new_combination;
        self.current_hotkey = Some(parsed_hotkey);

        if self.enabled {
            if self.register_current_hotkey() {
                Ok(STATUS_HOTKEY_REGISTERED.to_string())
            } else {
                Err(ERROR_HOTKEY_REGISTER.to_string())
            }
        } else {
            Ok("Hotkey updated (disabled)".to_string())
        }
    }

    fn register_current_hotkey(&self) -> bool {
        if let (Some(ref manager), Some(hotkey)) = (&self.manager, &self.current_hotkey) {
            if self.enabled {
                return manager.register(*hotkey).is_ok();
            }
        }
        false
    }

    fn unregister_current_hotkey(&self) -> bool {
        if let (Some(ref manager), Some(hotkey)) = (&self.manager, &self.current_hotkey) {
            return manager.unregister(*hotkey).is_ok();
        }
        false
    }

    pub fn get_status_message(&self) -> String {
        if self.enabled {
            if self.current_hotkey.is_some() {
                STATUS_HOTKEY_REGISTERED.to_string()
            } else {
                STATUS_HOTKEY_FAILED.to_string()
            }
        } else {
            STATUS_READY.to_string()
        }
    }

    pub fn create_subscription<T>(&self) -> Subscription<T>
    where
        T: 'static + Send + Clone + From<HotkeyEvent>,
    {
        if self.enabled && self.current_hotkey.is_some() {
            Subscription::run(|| {
                channel(32, |mut sender| async move {
                    let receiver = GlobalHotKeyEvent::receiver();
                    loop {
                        if let Ok(event) = receiver.try_recv() {
                            if event.state() == HotKeyState::Pressed {
                                let _ = sender.send(T::from(HotkeyEvent::Pressed)).await;
                            } else {
                                let _ = sender.send(T::from(HotkeyEvent::Released)).await;
                            }
                        }
                        async_std::task::sleep(std::time::Duration::from_millis(
                            HOTKEY_POLL_INTERVAL_MS,
                        ))
                        .await;
                    }
                })
            })
        } else {
            Subscription::none()
        }
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum HotkeyEvent {
    Pressed,
    Released,
}
