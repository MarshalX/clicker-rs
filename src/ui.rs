use iced::widget::{button, checkbox, pick_list, row, text, text_input};
use iced::{Alignment, Color, Element, Length};

use crate::constants::*;
use crate::icons::{icon_text, Icon, ICON_SIZE_BUTTON, ICON_SIZE_SMALL};
use crate::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusType {
    Success,
    Error,
    Warning,
    Info,
}

pub fn validation_indicator<'a>(
    is_valid: bool,
    valid_text: &'a str,
    invalid_text: &'a str,
) -> Element<'a, Message> {
    row![
        icon_text(
            if is_valid { Icon::Valid } else { Icon::Invalid },
            ICON_SIZE_SMALL
        ),
        text(if is_valid { valid_text } else { invalid_text }).size(UI_VALIDATION_SIZE)
    ]
    .spacing(UI_SPACING_TINY)
    .width(Length::Fixed(UI_VALIDATION_WIDTH))
    .align_y(Alignment::Center)
    .into()
}

pub fn icon_button(icon: Icon, label: &str, message: Option<Message>) -> button::Button<Message> {
    let content = row![icon_text(icon, ICON_SIZE_BUTTON), text(label)]
        .spacing(UI_SPACING_SMALL)
        .align_y(Alignment::Center);

    if let Some(msg) = message {
        button(content).on_press(msg)
    } else {
        button(content)
    }
}

pub fn input_row<'a>(
    label: &'a str,
    placeholder: &'a str,
    value: &'a str,
    on_change: fn(String) -> Message,
    validation: Element<'a, Message>,
) -> Element<'a, Message> {
    row![
        text(label).width(Length::Fixed(UI_LABEL_WIDTH)),
        text_input(placeholder, value)
            .on_input(on_change)
            .width(Length::Fixed(UI_INPUT_WIDTH)),
        validation
    ]
    .spacing(UI_SPACING_SMALL)
    .align_y(Alignment::Center)
    .into()
}

pub fn dropdown_row<T>(
    label: &str,
    options: Vec<T>,
    selected: Option<T>,
    on_change: fn(T) -> Message,
) -> Element<Message>
where
    T: std::fmt::Display + Clone + PartialEq + 'static,
{
    row![
        text(label).width(Length::Fixed(UI_LABEL_WIDTH)),
        pick_list(options, selected, on_change)
            .width(Length::Fixed(UI_INPUT_WIDTH + UI_VALIDATION_WIDTH))
    ]
    .spacing(UI_SPACING_SMALL)
    .into()
}

pub fn section_header(icon: Icon, title: &str) -> Element<Message> {
    row![
        icon_text(icon, ICON_SIZE_BUTTON),
        text(title).size(UI_SUBTITLE_SIZE)
    ]
    .spacing(UI_SPACING_SMALL)
    .align_y(Alignment::Center)
    .into()
}

impl StatusType {
    pub fn icon(self) -> Icon {
        match self {
            StatusType::Success => Icon::Success,
            StatusType::Error => Icon::Error,
            StatusType::Warning => Icon::Warning,
            StatusType::Info => Icon::Info,
        }
    }

    pub fn color(self) -> Color {
        match self {
            StatusType::Success => {
                Color::from_rgb(COLOR_SUCCESS[0], COLOR_SUCCESS[1], COLOR_SUCCESS[2])
            }
            StatusType::Error => Color::from_rgb(COLOR_ERROR[0], COLOR_ERROR[1], COLOR_ERROR[2]),
            StatusType::Warning => {
                Color::from_rgb(COLOR_WARNING[0], COLOR_WARNING[1], COLOR_WARNING[2])
            }
            StatusType::Info => Color::from_rgb(COLOR_INFO[0], COLOR_INFO[1], COLOR_INFO[2]),
        }
    }
}

pub fn status_message(status_type: StatusType, message: &str) -> Element<Message> {
    let color = status_type.color();
    row![
        icon_text(status_type.icon(), ICON_SIZE_SMALL).color(color),
        text(message).size(UI_VALIDATION_SIZE).color(color)
    ]
    .spacing(UI_SPACING_TINY)
    .align_y(Alignment::Center)
    .into()
}

pub fn styled_checkbox(
    label: &str,
    is_checked: bool,
    on_toggle: fn(bool) -> Message,
) -> checkbox::Checkbox<Message> {
    checkbox(label, is_checked).on_toggle(on_toggle)
}

pub fn clickable_text(text_content: &str, size: u16, message: Message) -> Element<Message> {
    button(text(text_content).size(size))
        .style(|theme: &iced::Theme, status| {
            let palette = theme.extended_palette();
            button::Style {
                background: None,
                text_color: match status {
                    button::Status::Hovered => palette.primary.strong.color,
                    _ => palette.primary.base.color,
                },
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            }
        })
        .padding(0)
        .on_press(message)
        .into()
}
