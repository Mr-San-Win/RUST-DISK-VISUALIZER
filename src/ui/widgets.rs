// Custom widgets module
use iced::{
    widget::{Container, row, button, Text, column},
    Element, Length,
};

pub fn toast<'a, Message: Clone + 'a>(
    message: String,
    on_dismiss: Message,
) -> Element<'a, Message> {
    Container::new(
        row![
            Text::new(message),
            button("×").on_press(on_dismiss),
        ]
        .spacing(8)
    )
    .padding(8)
    .style(iced::theme::Container::Box)
    .into()
}

pub fn toast_list<'a, Message: Clone + 'a>(
    toasts: &[String],
    on_dismiss: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message> {
    if toasts.is_empty() {
        return iced::widget::Space::with_height(Length::Fixed(0.0)).into();
    }

    let elements: Vec<Element<'a, Message>> = toasts
        .iter()
        .enumerate()
        .map(|(i, msg)| toast(msg.clone(), on_dismiss(i)))
        .collect();

    column(elements).spacing(5).into()
}

pub fn nav_button<'a, Message: Clone + 'a>(
    label: &'a str,
    active: bool,
    on_press: Message,
) -> Element<'a, Message> {
    let btn = button(label).on_press(on_press);
    
    // Style the button based on active state
    let styled_btn = if active {
        btn.style(iced::theme::Button::Primary)
    } else {
        btn.style(iced::theme::Button::Secondary)
    };
    
    Container::new(styled_btn)
        .width(Length::Fill)
        .padding(10)
        .into()
}
