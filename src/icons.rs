use iced::font::{Family, Font, Stretch, Weight};

pub const ICONS_FONT: &[u8] = include_bytes!("../assets/fonts/icons.ttf");

pub fn icons_font() -> Font {
    Font {
        family: Family::Name("icons"),
        weight: Weight::Normal,
        stretch: Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    // Primary action icons
    Stop,  // stop-fill
    Play,  // play-fill
    Reset, // arrow-clockwise

    // UI category icons
    #[allow(dead_code)]
    Mouse, // mouse3-fill
    Keyboard, // keyboard-fill
    #[allow(dead_code)]
    Settings, // gear-fill
    Timer,    // stopwatch-fill
    Click,    // hand-index-fill
    Info,     // info-circle-fill

    // Status/validation icons
    Valid,   // check-circle-fill
    Invalid, // exclamation-triangle-fill
    Warning, // exclamation-triangle-fill
    Success, // check-circle-fill
    Error,   // exclamation-triangle-fill
}

pub fn get_icon(icon: Icon) -> &'static str {
    match icon {
        Icon::Stop => "\u{F102}",
        Icon::Play => "\u{F103}",
        Icon::Reset => "\u{F10B}",
        Icon::Mouse => "\u{F104}",
        Icon::Keyboard => "\u{F105}",
        Icon::Settings => "\u{F108}",
        Icon::Timer => "\u{F101}",
        Icon::Click => "\u{F107}",
        Icon::Info => "\u{F106}",
        Icon::Valid => "\u{F10A}",
        Icon::Invalid => "\u{F109}",
        Icon::Warning => "\u{F109}",
        Icon::Success => "\u{F10A}",
        Icon::Error => "\u{F109}",
    }
}

pub const ICON_SIZE_SMALL: u16 = 12;
pub const ICON_SIZE_BUTTON: u16 = 14;

pub fn icon_text(icon: Icon, size: u16) -> iced::widget::Text<'static> {
    iced::widget::text(get_icon(icon))
        .font(icons_font())
        .size(size)
}

#[allow(dead_code)]
pub fn button_text_with_icon(icon: Icon, label: &str) -> String {
    format!("{} {}", get_icon(icon), label)
}
