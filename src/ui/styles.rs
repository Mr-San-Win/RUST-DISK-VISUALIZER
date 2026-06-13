use iced::{Background, Color, widget::container::StyleSheet, Border, Shadow, Vector};

pub struct SidebarStyle;

impl StyleSheet for SidebarStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                0.12, 0.12, 0.14, 0.85
            ))),
            text_color: None,
            border: Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: Shadow::default(),
        }
    }
}

impl From<SidebarStyle> for iced::theme::Container {
    fn from(_: SidebarStyle) -> Self {
        iced::theme::Container::Custom(Box::new(SidebarStyle))
    }
}

pub struct CardStyle;

struct CardStyleImpl;

impl StyleSheet for CardStyleImpl {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color::from_rgb(
                0.18, 0.18, 0.20
            ))),
            text_color: None,
            border: Border {
                radius: 12.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: card_shadow(),
        }
    }
}

impl From<CardStyle> for iced::theme::Container {
    fn from(_: CardStyle) -> Self {
        iced::theme::Container::Custom(Box::new(CardStyleImpl))
    }
}

pub fn card_shadow() -> Shadow {
    Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, 0.30),
        blur_radius: 14.0,
        offset: Vector::new(0.0, 6.0),
    }
}

pub fn s(base: u16, scale: f32) -> u16 {
    ((base as f32) * scale).round() as u16
}

