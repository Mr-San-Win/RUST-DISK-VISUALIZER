use iced::{
    widget::{column, row, button, scrollable, checkbox, text, container},
    Element, Length, alignment,
};
use humansize::{format_size, BINARY};
use crate::app::Message;
use crate::core::types::FileEntry;
use crate::ui::styles::{CardStyle, s};

pub fn view(
    groups: &Vec<Vec<FileEntry>>,
    selected: &Vec<Vec<bool>>,
    font_scale: f32,
) -> Element<'static, Message> {

    // Header
    let header = column![
        text("Duplicate Files")
            .size(s(26, font_scale)),
        text("Groups of potentially duplicate files identified by size + hash.")
            .size(s(14, font_scale))
    ]
    .spacing(s(6, font_scale) as f32);

    // Action Bar
    let action_bar = container(
        row![
            button("Find Duplicates")
                .on_press(Message::FindDuplicates)
                .style(iced::theme::Button::Primary)
                .padding(12),
            button("Delete Selected")
                .on_press(Message::DeleteSelectedDuplicates)
                .style(iced::theme::Button::Secondary)
                .padding(12),
            button("Export CSV")
                .on_press(Message::ExportDuplicatesCSV)
                .style(iced::theme::Button::Secondary)
                .padding(12),
            button("Export JSON")
                .on_press(Message::ExportDuplicatesJSON)
                .style(iced::theme::Button::Secondary)
                .padding(12),
        ]
        .spacing(s(16, font_scale) as f32)
    )
    .padding(s(20, font_scale) as f32)
    .style(CardStyle);

    // Duplicate Groups
    let groups_content = if groups.is_empty() {
        column![
            text("No duplicates found. Click 'Find Duplicates' to search.")
                .size(s(14, font_scale)),
        ]
    } else {
        column(
            groups.iter().enumerate().map(|(g_i, group)| {
                let group_items: Vec<Element<'static, Message>> = if let Some(selected_group) = selected.get(g_i) {
                    group.iter().enumerate().map(|(i, f)| {
                        let is_selected = selected_group.get(i).copied().unwrap_or(false);
                        let g_i_clone = g_i;
                        let i_clone = i;
                        row![
                            checkbox("", is_selected)
                                .on_toggle(move |_| Message::SelectDuplicate(g_i_clone, i_clone)),
                            text(f.path.display().to_string())
                                .size(s(15, font_scale))
                                .width(Length::Fixed(400.0)),
                            text(format_size(f.size, BINARY))
                                .size(s(15, font_scale))
                                .width(Length::Fixed(120.0))
                                .horizontal_alignment(alignment::Horizontal::Right),
                        ]
                        .padding(s(6, font_scale) as f32)
                        .into()
                    }).collect()
                } else {
                    vec![]
                };

                container(
                    column![
                        text(format!("Group {}", g_i + 1))
                            .size(s(18, font_scale))
                            .width(Length::Fill),
                        column(group_items).spacing(s(4, font_scale) as f32)
                    ]
                    .spacing(s(10, font_scale) as f32)
                    .padding(s(20, font_scale) as f32)
                )
                .style(CardStyle)
                .into()
            })
            .collect::<Vec<_>>()
        )
        .spacing(s(24, font_scale) as f32)
    };

    column![
        header,
        action_bar,
        container(
            scrollable(groups_content)
                .height(Length::Fill)
        )
        .padding(s(4, font_scale) as f32),
    ]
    .spacing(s(24, font_scale) as f32)
    .into()
}
