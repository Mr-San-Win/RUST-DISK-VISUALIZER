use iced::{
    widget::{column, row, text, checkbox, text_input, button, slider, Space, container},
    Element, Length,
};
use crate::app::Message;
use crate::core::types::Settings;
use crate::ui::styles::{CardStyle, s};

pub fn view(settings: &Settings) -> Element<'static, Message> {
    let ignore_str = if settings.ignore_globs.is_empty() {
        "".into()
    } else {
        settings.ignore_globs.join(", ")
    };

    // Header
    let header = text("Settings")
        .size(s(26, settings.font_scale));

    // Settings Card
    let settings_card = container(
        column![
            checkbox("Theme (Dark)", settings.theme_dark)
                .on_toggle(Message::ToggleTheme),
            Space::with_height(Length::Fixed((s(16, settings.font_scale) as f32).into())),
            row![
                text("Font Scale")
                    .size(s(15, settings.font_scale))
                    .width(Length::Shrink),
                slider(1.0..=1.5, settings.font_scale, |v| Message::FontScaleChanged(v))
                    .step(0.05)
                    .width(Length::Fill),
                text(format!("{:.2}x", settings.font_scale))
                    .size(s(15, settings.font_scale))
                    .width(Length::Shrink)
            ]
            .spacing(s(12, settings.font_scale) as f32)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center),
            Space::with_height(Length::Fixed((s(16, settings.font_scale) as f32).into())),
            column![
                text("Ignore globs (comma-separated)")
                    .size(s(14, settings.font_scale)),
                text_input("e.g., target,node_modules,.git", &ignore_str)
                    .on_input(Message::IgnoreGlobsChanged)
            ]
            .spacing(s(6, settings.font_scale) as f32),
            Space::with_height(Length::Fixed((s(16, settings.font_scale) as f32).into())),
            row![
                text("Partial hash KB")
                    .size(s(15, settings.font_scale)),
                text_input("256", &settings.partial_hash_kb.to_string())
                    .on_input(Message::PartialHashKbChanged)
            ]
            .spacing(s(10, settings.font_scale) as f32),
            Space::with_height(Length::Fixed((s(16, settings.font_scale) as f32).into())),
            row![
                button("Save Settings")
                    .on_press(Message::SaveSettings)
                    .style(iced::theme::Button::Primary)
                    .padding(s(12, settings.font_scale) as f32),
                button("Reload")
                    .on_press(Message::ReloadSettings)
                    .style(iced::theme::Button::Secondary)
                    .padding(s(12, settings.font_scale) as f32)
            ]
            .spacing(s(16, settings.font_scale) as f32),
        ]
        .spacing(s(16, settings.font_scale) as f32)
        .padding(s(20, settings.font_scale) as f32)
    )
    .style(CardStyle);

    column![
        header,
        settings_card,
    ]
    .spacing(s(24, settings.font_scale) as f32)
    .into()
}
