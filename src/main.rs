mod clicker;
mod config;
mod constants;
mod hotkey;

use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input, Space};
use iced::{Element, Length, Size, Subscription, Task};

use clicker::{Clicker, ClickerMessage};
use config::{ClickButton, ClickType, ClickerConfig, DelayMode};
use constants::*;
use hotkey::{HotkeyEvent, HotkeyManager};

fn main() -> iced::Result {
    iced::application(APP_TITLE, update, view)
        .subscription(subscription)
        .window_size(Size::new(UI_WINDOW_WIDTH, UI_WINDOW_HEIGHT))
        .resizable(false)
        .run_with(new)
}

struct State {
    click_interval: String,
    cps_input: String,
    min_delay_input: String,
    max_delay_input: String,
    status_message: String,
    clicker: Clicker,
    hotkey_manager: HotkeyManager,
}

fn new() -> (State, Task<Message>) {
    let clicker_config = ClickerConfig::default();

    let hotkey_manager = HotkeyManager::with_config(
        clicker_config.hotkey_combination.clone(),
        clicker_config.hotkeys_enabled,
    );

    let status_message = hotkey_manager.get_status_message();

    (
        State {
            click_interval: DEFAULT_INTERVAL.to_string(),
            cps_input: DEFAULT_CPS.to_string(),
            min_delay_input: DEFAULT_MIN_DELAY.to_string(),
            max_delay_input: DEFAULT_MAX_DELAY.to_string(),
            status_message,
            clicker: Clicker::new(clicker_config),
            hotkey_manager,
        },
        Task::none(),
    )
}

#[derive(Debug, Clone)]
pub enum Message {
    IntervalChanged(String),
    DelayModeChanged(DelayMode),
    CpsChanged(String),
    MinDelayChanged(String),
    MaxDelayChanged(String),
    StartStop,
    Reset,
    ClickerMessage(ClickerMessage),
    HotkeyChanged(String),
    HotkeyEnabledChanged(bool),
    HotkeyEvent(HotkeyEvent),
    ClickButtonChanged(ClickButton),
    ClickTypeChanged(ClickType),
}

impl From<HotkeyEvent> for Message {
    fn from(event: HotkeyEvent) -> Self {
        Message::HotkeyEvent(event)
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::IntervalChanged(value) => {
            state.click_interval = value.clone();
            if let Err(e) = state.clicker.config_mut().from_string(&value) {
                state.status_message = e;
            } else if !state.clicker.is_running() {
                state.status_message = STATUS_READY.to_string();
            }
        }
        Message::DelayModeChanged(mode) => {
            state.clicker.config_mut().delay_mode = mode;
            if !state.clicker.is_running() {
                state.status_message = STATUS_READY.to_string();
            }
        }
        Message::CpsChanged(value) => {
            state.cps_input = value.clone();
            if let Err(e) = state.clicker.config_mut().from_cps_string(&value) {
                state.status_message = e;
            } else if !state.clicker.is_running() {
                state.status_message = STATUS_READY.to_string();
            }
        }
        Message::MinDelayChanged(value) => {
            state.min_delay_input = value.clone();
            if let Ok(min_delay) = value.parse::<u64>() {
                let max_delay = state
                    .max_delay_input
                    .parse::<u64>()
                    .unwrap_or(DEFAULT_MAX_DELAY);
                if let Err(e) = state
                    .clicker
                    .config_mut()
                    .set_jitter_range(min_delay, max_delay)
                {
                    state.status_message = e;
                } else if !state.clicker.is_running() {
                    state.status_message = STATUS_READY.to_string();
                }
            } else {
                state.status_message = ERROR_INVALID_FORMAT.to_string();
            }
        }
        Message::MaxDelayChanged(value) => {
            state.max_delay_input = value.clone();
            if let Ok(max_delay) = value.parse::<u64>() {
                let min_delay = state
                    .min_delay_input
                    .parse::<u64>()
                    .unwrap_or(DEFAULT_MIN_DELAY);
                if let Err(e) = state
                    .clicker
                    .config_mut()
                    .set_jitter_range(min_delay, max_delay)
                {
                    state.status_message = e;
                } else if !state.clicker.is_running() {
                    state.status_message = STATUS_READY.to_string();
                }
            } else {
                state.status_message = ERROR_INVALID_FORMAT.to_string();
            }
        }
        Message::StartStop => {
            if state.clicker.is_running() {
                state.clicker.stop();
                state.status_message = STATUS_STOPPED.to_string();
            } else {
                let config_valid = match state.clicker.config().delay_mode {
                    DelayMode::CPS => {
                        match state.clicker.config_mut().from_cps_string(&state.cps_input) {
                            Ok(_) => true,
                            Err(e) => {
                                state.status_message = e;
                                false
                            }
                        }
                    }
                    DelayMode::Jitter => {
                        let min_delay = state.min_delay_input.parse::<u64>().unwrap_or(0);
                        let max_delay = state.max_delay_input.parse::<u64>().unwrap_or(0);
                        match state
                            .clicker
                            .config_mut()
                            .set_jitter_range(min_delay, max_delay)
                        {
                            Ok(_) => true,
                            Err(e) => {
                                state.status_message = e;
                                false
                            }
                        }
                    }
                };

                if config_valid {
                    state.clicker.start();
                    state.status_message =
                        format!("{} - {}", STATUS_RUNNING, state.clicker.get_delay_info());
                    return perform_click(state.clicker.clone());
                }
            }
        }
        Message::Reset => {
            state.clicker.stop();
            state.status_message = STATUS_RESET.to_string();
        }
        Message::ClickerMessage(clicker_msg) => match clicker_msg {
            ClickerMessage::Tick => {
                if state.clicker.is_running() {
                    return perform_click(state.clicker.clone());
                }
            }
            ClickerMessage::ClickError(error) => {
                state.status_message = format!("Error: {}", error);
                state.clicker.stop();
            }
        },
        Message::HotkeyChanged(value) => {
            match state.hotkey_manager.update_combination(value.clone()) {
                Ok(msg) => {
                    state.status_message = msg;
                    state.clicker.config_mut().hotkey_combination = value;
                }
                Err(err) => {
                    state.status_message = err;
                }
            }
        }
        Message::HotkeyEnabledChanged(enabled) => match state.hotkey_manager.set_enabled(enabled) {
            Ok(msg) => {
                state.status_message = msg;
                state.clicker.config_mut().hotkeys_enabled = enabled;
            }
            Err(err) => {
                state.status_message = err;
            }
        },
        Message::HotkeyEvent(HotkeyEvent::Pressed) => {
            return Task::done(Message::StartStop);
        }
        Message::HotkeyEvent(HotkeyEvent::Released) => {
            // For now, we only care about key press events
            // Released events can be handled here if needed in the future
        }
        Message::ClickButtonChanged(button) => {
            state.clicker.config_mut().click_button = button;
            if !state.clicker.is_running() {
                state.status_message = STATUS_READY.to_string();
            }
        }
        Message::ClickTypeChanged(click_type) => {
            state.clicker.config_mut().click_type = click_type;
            if !state.clicker.is_running() {
                state.status_message = STATUS_READY.to_string();
            }
        }
    }
    Task::none()
}

fn subscription(state: &State) -> Subscription<Message> {
    state.hotkey_manager.create_subscription()
}

fn perform_click(clicker: Clicker) -> Task<Message> {
    Task::perform(async move { clicker.perform_click().await }, |result| {
        Message::ClickerMessage(result)
    })
}

fn view(state: &State) -> Element<Message> {
    let title = text(APP_TITLE).size(UI_TITLE_SIZE);
    let subtitle = text(APP_SUBTITLE).size(UI_SUBTITLE_SIZE);
    let status = text(&state.status_message).size(UI_STATUS_SIZE);

    let delay_mode_dropdown = row![
        text(UI_DELAY_MODE_LABEL).width(Length::Fixed(UI_LABEL_WIDTH)),
        pick_list(
            DelayMode::all(),
            Some(state.clicker.config().delay_mode.clone()),
            Message::DelayModeChanged
        )
        .width(Length::Fixed(UI_INPUT_WIDTH + UI_VALIDATION_WIDTH))
    ]
    .spacing(UI_SPACING_SMALL);

    let delay_config_inputs = match state.clicker.config().delay_mode {
        DelayMode::CPS => {
            let is_valid_cps = state
                .clicker
                .config()
                .is_valid_cps(state.cps_input.parse::<f64>().unwrap_or(0.0));

            column![row![
                text(UI_CPS_LABEL).width(Length::Fixed(UI_LABEL_WIDTH)),
                text_input(UI_CPS_PLACEHOLDER, &state.cps_input)
                    .on_input(Message::CpsChanged)
                    .width(Length::Fixed(UI_INPUT_WIDTH)),
                text(if is_valid_cps {
                    UI_INTERVAL_VALID
                } else {
                    UI_INTERVAL_INVALID
                })
                .size(UI_VALIDATION_SIZE)
                .width(Length::Fixed(UI_VALIDATION_WIDTH))
            ]
            .spacing(UI_SPACING_SMALL)]
        }
        DelayMode::Jitter => {
            let min_delay = state.min_delay_input.parse::<u64>().unwrap_or(0);
            let max_delay = state.max_delay_input.parse::<u64>().unwrap_or(0);
            let is_valid_jitter = state
                .clicker
                .config()
                .is_valid_jitter_range(min_delay, max_delay);

            column![
                row![
                    text(UI_MIN_DELAY_LABEL).width(Length::Fixed(UI_LABEL_WIDTH)),
                    text_input(UI_MIN_DELAY_PLACEHOLDER, &state.min_delay_input)
                        .on_input(Message::MinDelayChanged)
                        .width(Length::Fixed(UI_INPUT_WIDTH)),
                    text(if is_valid_jitter && min_delay > 0 {
                        UI_INTERVAL_VALID
                    } else {
                        UI_INTERVAL_INVALID
                    })
                    .size(UI_VALIDATION_SIZE)
                    .width(Length::Fixed(UI_VALIDATION_WIDTH))
                ]
                .spacing(UI_SPACING_SMALL),
                Space::with_height(UI_SPACING_SMALL),
                row![
                    text(UI_MAX_DELAY_LABEL).width(Length::Fixed(UI_LABEL_WIDTH)),
                    text_input(UI_MAX_DELAY_PLACEHOLDER, &state.max_delay_input)
                        .on_input(Message::MaxDelayChanged)
                        .width(Length::Fixed(UI_INPUT_WIDTH)),
                    text(if is_valid_jitter && max_delay > 0 {
                        UI_INTERVAL_VALID
                    } else {
                        UI_INTERVAL_INVALID
                    })
                    .size(UI_VALIDATION_SIZE)
                    .width(Length::Fixed(UI_VALIDATION_WIDTH))
                ]
                .spacing(UI_SPACING_SMALL)
            ]
        }
    };

    let is_valid_hotkey = HotkeyManager::is_valid_hotkey(state.hotkey_manager.get_combination());

    let hotkey_input = row![
        text(UI_HOTKEY_LABEL).width(Length::Fixed(UI_LABEL_WIDTH)),
        text_input(
            UI_HOTKEY_PLACEHOLDER,
            state.hotkey_manager.get_combination()
        )
        .on_input(Message::HotkeyChanged)
        .width(Length::Fixed(UI_INPUT_WIDTH)),
        text(if is_valid_hotkey {
            UI_HOTKEY_VALID
        } else {
            UI_HOTKEY_INVALID
        })
        .size(UI_VALIDATION_SIZE)
        .width(Length::Fixed(UI_VALIDATION_WIDTH))
    ]
    .spacing(UI_SPACING_SMALL);

    let hotkey_enabled = checkbox(UI_HOTKEY_ENABLED_LABEL, state.hotkey_manager.is_enabled())
        .on_toggle(Message::HotkeyEnabledChanged);

    let click_button_dropdown = row![
        text(UI_CLICK_BUTTON_LABEL).width(Length::Fixed(UI_LABEL_WIDTH)),
        pick_list(
            ClickButton::all(),
            Some(state.clicker.config().click_button.clone()),
            Message::ClickButtonChanged
        )
        .width(Length::Fixed(UI_INPUT_WIDTH + UI_VALIDATION_WIDTH))
    ]
    .spacing(UI_SPACING_SMALL);

    let click_type_dropdown = row![
        text(UI_CLICK_TYPE_LABEL).width(Length::Fixed(UI_LABEL_WIDTH)),
        pick_list(
            ClickType::all(),
            Some(state.clicker.config().click_type.clone()),
            Message::ClickTypeChanged
        )
        .width(Length::Fixed(UI_INPUT_WIDTH + UI_VALIDATION_WIDTH))
    ]
    .spacing(UI_SPACING_SMALL);

    let is_config_valid = match state.clicker.config().delay_mode {
        DelayMode::CPS => state
            .clicker
            .config()
            .is_valid_cps(state.cps_input.parse::<f64>().unwrap_or(0.0)),
        DelayMode::Jitter => {
            let min_delay = state.min_delay_input.parse::<u64>().unwrap_or(0);
            let max_delay = state.max_delay_input.parse::<u64>().unwrap_or(0);
            state
                .clicker
                .config()
                .is_valid_jitter_range(min_delay, max_delay)
                && min_delay > 0
                && max_delay > 0
        }
    };

    let start_stop_button = if is_config_valid || state.clicker.is_running() {
        button(if state.clicker.is_running() {
            UI_BUTTON_STOP
        } else {
            UI_BUTTON_START
        })
        .on_press(Message::StartStop)
    } else {
        button(UI_BUTTON_START)
    };

    let control_buttons = row![
        start_stop_button,
        button(UI_BUTTON_RESET).on_press(Message::Reset)
    ]
    .spacing(UI_SPACING_MEDIUM);

    let content = column![
        title,
        subtitle,
        Space::with_height(UI_SPACING_LARGE),
        status,
        Space::with_height(UI_SPACING_MEDIUM),
        delay_mode_dropdown,
        Space::with_height(UI_SPACING_SMALL),
        delay_config_inputs,
        Space::with_height(UI_SPACING_SMALL),
        click_button_dropdown,
        Space::with_height(UI_SPACING_SMALL),
        click_type_dropdown,
        Space::with_height(UI_SPACING_SMALL),
        hotkey_input,
        Space::with_height(UI_SPACING_SMALL),
        hotkey_enabled,
        Space::with_height(UI_SPACING_MEDIUM),
        control_buttons,
        Space::with_height(UI_SPACING_MEDIUM),
        text(UI_PERMISSION_NOTE).size(UI_NOTE_SIZE),
        Space::with_height(UI_SPACING_SMALL),
        text(UI_WEBSITE_NOTE).size(UI_NOTE_SIZE),
    ]
    .spacing(UI_SPACING_SMALL);

    container(content)
        .width(Length::Fixed(UI_WINDOW_WIDTH))
        .height(Length::Shrink)
        .center_x(Length::Fill)
        .center_y(Length::Shrink)
        .padding(10)
        .into()
}
