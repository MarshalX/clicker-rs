mod clicker;
mod config;
mod constants;
mod hotkey;
mod icons;
mod timer;
mod ui;

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Size, Subscription, Task};

use clicker::{Clicker, ClickerMessage};
use config::{ClickButton, ClickType, ClickerConfig, DelayMode};
use constants::*;
use hotkey::{HotkeyEvent, HotkeyManager};
use icons::{Icon, ICONS_FONT};
use ui::*;

fn main() -> iced::Result {
    iced::application(APP_TITLE, update, view)
        .subscription(subscription)
        .window_size(Size::new(UI_WINDOW_WIDTH, UI_WINDOW_HEIGHT))
        .font(ICONS_FONT)
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
    WebsiteClick,
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
            if let Err(e) = state.clicker.config_mut().parse_interval_string(&value) {
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
            if let Err(e) = state.clicker.config_mut().parse_cps_string(&value) {
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
                    DelayMode::Cps => {
                        match state
                            .clicker
                            .config_mut()
                            .parse_cps_string(&state.cps_input)
                        {
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

                    return state
                        .clicker
                        .create_error_check_task()
                        .map(Message::ClickerMessage);
                }
            }
        }
        Message::Reset => {
            state.clicker.stop();
            state.status_message = STATUS_RESET.to_string();
        }
        Message::ClickerMessage(clicker_msg) => match clicker_msg {
            ClickerMessage::ClickError(error) => {
                state.status_message = format!("Error: {}", error);
                state.clicker.stop();
            }
            ClickerMessage::NoError => {
                if state.clicker.is_running() {
                    return state
                        .clicker
                        .create_error_check_task()
                        .map(Message::ClickerMessage);
                }
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
        Message::WebsiteClick => {
            if let Err(e) = webbrowser::open(WEBSITE_URL) {
                state.status_message = format!("Failed to open website: {}", e);
            }
        }
    }
    Task::none()
}

fn subscription(state: &State) -> Subscription<Message> {
    state.hotkey_manager.create_subscription()
}

fn view(state: &State) -> Element<'_, Message> {
    let title = text(APP_TITLE).size(UI_TITLE_SIZE);
    let subtitle = text(APP_SUBTITLE).size(UI_SUBTITLE_SIZE);

    // rewrite later
    let status_type =
        if state.status_message.contains("Error:") || state.status_message.contains("Failed") {
            StatusType::Error
        } else if state.status_message.contains("Invalid") || !state.hotkey_manager.is_enabled() {
            StatusType::Warning
        } else if state.status_message.contains("running") {
            StatusType::Info
        } else {
            StatusType::Success
        };

    let status = status_message(status_type, &state.status_message);

    let delay_mode_dropdown = dropdown_row(
        UI_DELAY_MODE_LABEL,
        DelayMode::all(),
        Some(state.clicker.config().delay_mode.clone()),
        Message::DelayModeChanged,
    );

    let delay_config_inputs = match state.clicker.config().delay_mode {
        DelayMode::Cps => {
            let is_valid_cps = state
                .clicker
                .config()
                .is_valid_cps(state.cps_input.parse::<f64>().unwrap_or(0.0));

            let validation = validation_indicator(
                is_valid_cps,
                UI_INTERVAL_VALID_TEXT,
                UI_INTERVAL_INVALID_TEXT,
            );

            column![input_row(
                UI_CPS_LABEL,
                UI_CPS_PLACEHOLDER,
                &state.cps_input,
                Message::CpsChanged,
                validation,
            )]
        }
        DelayMode::Jitter => {
            let min_delay = state.min_delay_input.parse::<u64>().unwrap_or(0);
            let max_delay = state.max_delay_input.parse::<u64>().unwrap_or(0);
            let is_valid_jitter = state
                .clicker
                .config()
                .is_valid_jitter_range(min_delay, max_delay);

            let min_validation = validation_indicator(
                is_valid_jitter && min_delay > 0,
                UI_INTERVAL_VALID_TEXT,
                UI_INTERVAL_INVALID_TEXT,
            );
            let max_validation = validation_indicator(
                is_valid_jitter && max_delay > 0,
                UI_INTERVAL_VALID_TEXT,
                UI_INTERVAL_INVALID_TEXT,
            );

            column![
                input_row(
                    UI_MIN_DELAY_LABEL,
                    UI_MIN_DELAY_PLACEHOLDER,
                    &state.min_delay_input,
                    Message::MinDelayChanged,
                    min_validation,
                ),
                input_row(
                    UI_MAX_DELAY_LABEL,
                    UI_MAX_DELAY_PLACEHOLDER,
                    &state.max_delay_input,
                    Message::MaxDelayChanged,
                    max_validation,
                ),
            ]
            .spacing(UI_SPACING_SMALL)
        }
    };

    let is_valid_hotkey = HotkeyManager::is_valid_hotkey(state.hotkey_manager.get_combination());
    let hotkey_validation = validation_indicator(
        is_valid_hotkey,
        UI_HOTKEY_VALID_TEXT,
        UI_HOTKEY_INVALID_TEXT,
    );
    let hotkey_input = input_row(
        UI_HOTKEY_LABEL,
        UI_HOTKEY_PLACEHOLDER,
        state.hotkey_manager.get_combination(),
        Message::HotkeyChanged,
        hotkey_validation,
    );

    let hotkey_enabled = styled_checkbox(
        UI_HOTKEY_ENABLED_LABEL,
        state.hotkey_manager.is_enabled(),
        Message::HotkeyEnabledChanged,
    );

    let click_button_dropdown = dropdown_row(
        UI_CLICK_BUTTON_LABEL,
        ClickButton::all(),
        Some(state.clicker.config().click_button.clone()),
        Message::ClickButtonChanged,
    );

    let click_type_dropdown = dropdown_row(
        UI_CLICK_TYPE_LABEL,
        ClickType::all(),
        Some(state.clicker.config().click_type.clone()),
        Message::ClickTypeChanged,
    );

    let is_config_valid = match state.clicker.config().delay_mode {
        DelayMode::Cps => state
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
        icon_button(
            if state.clicker.is_running() {
                Icon::Stop
            } else {
                Icon::Play
            },
            if state.clicker.is_running() {
                UI_BUTTON_STOP
            } else {
                UI_BUTTON_START
            },
            Some(Message::StartStop),
        )
    } else {
        icon_button(Icon::Play, UI_BUTTON_START, None)
    };

    let control_buttons = row![
        start_stop_button,
        icon_button(Icon::Reset, UI_BUTTON_RESET, Some(Message::Reset))
    ]
    .spacing(UI_SPACING_MEDIUM);

    let content = column![
        column![title, subtitle].spacing(UI_SPACING_SMALL),
        status,
        section_header(Icon::Timer, UI_SECTION_DELAY_CONFIG),
        column![delay_mode_dropdown, delay_config_inputs,].spacing(UI_SPACING_SMALL),
        section_header(Icon::Click, UI_SECTION_CLICK_CONFIG),
        column![click_button_dropdown, click_type_dropdown,].spacing(UI_SPACING_SMALL),
        section_header(Icon::Keyboard, UI_SECTION_HOTKEY_CONFIG),
        column![hotkey_input, hotkey_enabled].spacing(UI_SPACING_SMALL),
        control_buttons,
        column![
            text(UI_PERMISSION_NOTE).size(UI_NOTE_SIZE),
            clickable_text(UI_WEBSITE_NOTE, UI_NOTE_SIZE, Message::WebsiteClick),
        ]
        .spacing(UI_SPACING_SMALL),
    ]
    .spacing(UI_SPACING_LARGE);

    container(content)
        .width(Length::Fixed(UI_WINDOW_WIDTH))
        .height(Length::Shrink)
        .center_x(Length::Fill)
        .center_y(Length::Shrink)
        .padding(UI_CONTAINER_PADDING)
        .into()
}
