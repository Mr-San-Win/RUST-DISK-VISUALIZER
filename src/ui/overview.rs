use iced::{
    widget::{column, row, scrollable, container, text, button},
    alignment,
    Length,
};
use humansize::{format_size, BINARY};
use crate::app::Message;
use crate::core::types::FileEntry;
use crate::ui::styles::{CardStyle, s};

pub fn view(
    selected: Option<&std::path::Path>,
    scanning: bool,
    files: &Vec<FileEntry>,
    font_scale: f32,
) -> iced::Element<'static, Message> {

    let subtitle = selected
        .map(|p| p.display().to_string())
        .unwrap_or("Select a directory to begin".into());

    let header = column![
        text("Disk Usage Overview")
            .size(s(26, font_scale)),
        text(subtitle).size(s(15, font_scale))
    ]
    .spacing(s(6, font_scale) as f32);

    let actions = container(
        row![
            button("Choose Folder")
                .on_press(Message::ChooseFolder)
                .style(iced::theme::Button::Secondary)
                .padding(12),
            button("Scan")
                .on_press(Message::StartScan)
                .style(iced::theme::Button::Primary)
                .padding(12)
                .width(Length::Shrink),
        ]
        .spacing(s(14, font_scale) as f32)
    )
    .padding(s(20, font_scale) as f32)
    .style(CardStyle);

    let status_text = if scanning {
        "Scanning...".to_string()
    } else {
        format!("{} files indexed", files.len())
    };

    let status = container(
        text(status_text)
            .size(s(15, font_scale))
    )
    .padding(s(20, font_scale) as f32)
    .style(CardStyle);

    let table_header = row![
        text("File")
            .size(s(16, font_scale))
            .width(Length::FillPortion(4)),
        text("Size")
            .size(s(16, font_scale))
            .width(Length::FillPortion(1))
            .horizontal_alignment(alignment::Horizontal::Right),
    ]
    .padding([0, 0, s(8, font_scale) as u16, 0]);

    let table_rows = files.iter().enumerate().map(|(i, f)| {
        let row = row![
            text(f.path.display().to_string())
                .size(s(15, font_scale))
                .width(Length::FillPortion(4)),
            text(format_size(f.size, BINARY))
                .size(s(15, font_scale))
                .width(Length::FillPortion(1))
                .horizontal_alignment(alignment::Horizontal::Right),
        ]
        .padding(s(6, font_scale) as f32);

        if i % 2 == 0 {
            container(row).style(CardStyle).into()
        } else {
            container(row).style(iced::theme::Container::Transparent).into()
        }
    });

    let table = container(
        scrollable(
            column![
                table_header,
                column(table_rows).spacing(s(4, font_scale) as f32)
            ]
            .spacing(s(10, font_scale) as f32)
        )
        .height(Length::Fixed(450.0))
    )
    .padding(s(20, font_scale) as f32)
    .style(CardStyle);

    column![
        header,
        actions,
        status,
        table
    ]
    .spacing(s(24, font_scale) as f32)
    .into()
}
