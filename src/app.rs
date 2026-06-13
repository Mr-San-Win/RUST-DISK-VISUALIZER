use iced::{
    executor, Application, Command, Element,
    Theme, widget::{column, row, container, text, Space, button},
    Length,
};
use crate::ui::{overview, duplicates, settings, widgets};
use crate::ui::styles::SidebarStyle;
use crate::core::types::{FileEntry, Settings};
use crate::core::config;
use std::path::PathBuf;
use tracing::{info, error};

pub struct DiskVizApp {
    active: Screen,
    selected_folder: Option<PathBuf>,
    scanning: bool,
    progress_count: usize,
    progress_total: usize,
    files: Vec<FileEntry>,
    duplicates: Vec<Vec<FileEntry>>,
    selected_in_group: Vec<Vec<bool>>,
    delete_confirmation: Option<Vec<PathBuf>>,
    settings: Settings,
    toasts: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Overview,
    Duplicates,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchTo(Screen),
    ChooseFolder,
    FolderChosen(Option<std::path::PathBuf>),
    StartScan,
    #[allow(dead_code)]
    ScanProgress(usize, usize, Option<std::path::PathBuf>),
    ScanFinished(Vec<FileEntry>),
    FindDuplicates,
    DuplicatesFound(Vec<Vec<FileEntry>>),
    SelectDuplicate(usize, usize), // group index, item index
    DeleteSelectedDuplicates,
    ConfirmDelete,
    CancelDelete,
    ExportDuplicatesCSV,
    ExportDuplicatesJSON,
    // Settings
    ToggleTheme(bool),
    FontScaleChanged(f32),
    IgnoreGlobsChanged(String),
    PartialHashKbChanged(String),
    SaveSettings,
    ReloadSettings,
    // Toasts
    Toast(String),
    DismissToast(usize),
}

impl Application for DiskVizApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let settings = config::load().unwrap_or_default();
        (
            Self {
                active: Screen::Overview,
                selected_folder: None,
                scanning: false,
                progress_count: 0,
                progress_total: 0,
                files: vec![],
                duplicates: vec![],
                selected_in_group: vec![],
                delete_confirmation: None,
                settings,
                toasts: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "DiskViz".into()
    }

    fn theme(&self) -> Theme {
        if self.settings.theme_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }


    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::SwitchTo(screen) => {
                self.active = screen;
            }
            Message::ChooseFolder => {
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new().pick_folder().await
                    },
                    |folder| Message::FolderChosen(folder.map(|f| f.path().to_path_buf())),
                );
            }
            Message::FolderChosen(folder) => {
                self.selected_folder = folder;
            }
            Message::StartScan => {
                if let Some(path) = self.selected_folder.clone() {
                    self.scanning = true;
                    self.progress_count = 0;
                    self.progress_total = 0;
                    let ignore_globs = self.settings.ignore_globs.clone();
                    return Command::perform(
                        async move {
                            crate::core::scan::scan_directory(path, ignore_globs).await
                        },
                        |(files, _progress)| Message::ScanFinished(files),
                    );
                }
            }
            Message::ScanProgress(count, total, _current) => {
                self.progress_count = count;
                self.progress_total = total;
            }
            Message::ScanFinished(files) => {
                self.scanning = false;
                self.files = files;
                self.progress_count = 0;
                self.progress_total = 0;
                self.toasts.push("Scan completed".into());
            }
            Message::FindDuplicates => {
                let files = self.files.clone();
                let partial_hash_kb = self.settings.partial_hash_kb;
                return Command::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::core::dedupe::find_duplicates(&files, partial_hash_kb)
                        })
                        .await
                        .unwrap_or_default()
                    },
                    Message::DuplicatesFound,
                );
            }
            Message::DuplicatesFound(groups) => {
                info!("Found {} duplicate groups", groups.len());
                self.duplicates = groups.clone();
                self.selected_in_group = groups.iter().map(|g| vec![false; g.len()]).collect();
                self.toasts.push(format!("Found {} duplicate groups", groups.len()));
            }
            Message::SelectDuplicate(g, i) => {
                if let Some(group) = self.selected_in_group.get_mut(g) {
                    if let Some(v) = group.get_mut(i) {
                        *v = !*v;
                    }
                }
            }
            Message::DeleteSelectedDuplicates => {
                let mut to_delete = Vec::new();
                for (g_i, group) in self.duplicates.iter().enumerate() {
                    if let Some(selected_group) = self.selected_in_group.get(g_i) {
                        for (i, f) in group.iter().enumerate() {
                            if let Some(&selected) = selected_group.get(i) {
                                if selected {
                                    to_delete.push(f.path.clone());
                                }
                            }
                        }
                    }
                }
                if !to_delete.is_empty() {
                    self.delete_confirmation = Some(to_delete);
                }
            }
            Message::ConfirmDelete => {
                if let Some(paths) = self.delete_confirmation.take() {
                    match crate::core::trashcan::move_to_trash(&paths) {
                        Ok(_) => {
                            info!("Successfully deleted {} files", paths.len());
                            self.toasts.push(format!("Deleted {} file(s)", paths.len()));
                            // Remove deleted files from duplicates and selected state
                            let mut new_duplicates = Vec::new();
                            let mut new_selected = Vec::new();
                            
                            for (g_i, group) in self.duplicates.iter().enumerate() {
                                let mut new_group = Vec::new();
                                let mut new_selected_group = Vec::new();
                                
                                if let Some(selected_group) = self.selected_in_group.get(g_i) {
                                    for (i, f) in group.iter().enumerate() {
                                        if let Some(&was_selected) = selected_group.get(i) {
                                            if !was_selected || !paths.contains(&f.path) {
                                                new_group.push(f.clone());
                                                new_selected_group.push(false);
                                            }
                                        }
                                    }
                                }
                                
                                if new_group.len() > 1 {
                                    new_duplicates.push(new_group);
                                    new_selected.push(new_selected_group);
                                }
                            }
                            
                            self.duplicates = new_duplicates;
                            self.selected_in_group = new_selected;
                        }
                        Err(e) => {
                            error!("Failed to delete files: {}", e);
                            self.toasts.push(format!("Failed to delete files: {}", e));
                        }
                    }
                }
            }
            Message::CancelDelete => {
                self.delete_confirmation = None;
            }
            Message::ExportDuplicatesCSV => {
                let duplicates = self.duplicates.clone();
                return Command::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .set_file_name("duplicates.csv")
                            .save_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    move |path| {
                        if let Some(p) = path {
                            if let Err(e) = crate::core::export::export_duplicates_csv(p, &duplicates) {
                                error!("Failed to export CSV: {}", e);
                                Message::Toast(format!("Failed to export CSV: {}", e))
                            } else {
                                Message::Toast("CSV exported successfully".into())
                            }
                        } else {
                            Message::SwitchTo(Screen::Duplicates)
                        }
                    },
                );
            }
            Message::ExportDuplicatesJSON => {
                let duplicates = self.duplicates.clone();
                return Command::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .set_file_name("duplicates.json")
                            .save_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    move |path| {
                        if let Some(p) = path {
                            if let Err(e) = crate::core::export::export_duplicates_json(p, &duplicates) {
                                error!("Failed to export JSON: {}", e);
                                Message::Toast(format!("Failed to export JSON: {}", e))
                            } else {
                                Message::Toast("JSON exported successfully".into())
                            }
                        } else {
                            Message::SwitchTo(Screen::Duplicates)
                        }
                    },
                );
            }
            // Settings handlers
            Message::ToggleTheme(v) => {
                self.settings.theme_dark = v;
            }
            Message::FontScaleChanged(v) => {
                self.settings.font_scale = v.clamp(1.0, 1.5);
            }
            Message::IgnoreGlobsChanged(s) => {
                let parts = s.split(',')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<_>>();
                self.settings.ignore_globs = parts;
            }
            Message::PartialHashKbChanged(s) => {
                if let Ok(kb) = s.trim().parse::<u64>() {
                    self.settings.partial_hash_kb = kb;
                }
            }
            Message::SaveSettings => {
                match config::save(&self.settings) {
                    Ok(_) => {
                        self.toasts.push("Settings saved".into());
                    }
                    Err(e) => {
                        self.toasts.push(format!("Failed to save settings: {}", e));
                    }
                }
            }
            Message::ReloadSettings => {
                match config::load() {
                    Ok(s) => {
                        self.settings = s;
                        self.toasts.push("Settings reloaded".into());
                    }
                    Err(e) => {
                        self.toasts.push(format!("Failed to reload settings: {}", e));
                    }
                }
            }
            // Toast handlers
            Message::Toast(t) => {
                self.toasts.push(t);
            }
            Message::DismissToast(i) => {
                if i < self.toasts.len() {
                    self.toasts.remove(i);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        // Sidebar navigation
        let sidebar_title = text("DiskViz")
            .size(28)
            .style(iced::theme::Text::Color(iced::Color::from_rgb(1.0, 1.0, 1.0)));
        
        let sidebar_content = column![
            container(sidebar_title)
                .padding(24)
                .width(Length::Fill),
            Space::with_height(Length::Fixed(18.0)),
            widgets::nav_button(
                "Overview",
                matches!(self.active, Screen::Overview),
                Message::SwitchTo(Screen::Overview),
            ),
            widgets::nav_button(
                "Duplicates",
                matches!(self.active, Screen::Duplicates),
                Message::SwitchTo(Screen::Duplicates),
            ),
            widgets::nav_button(
                "Settings",
                matches!(self.active, Screen::Settings),
                Message::SwitchTo(Screen::Settings),
            ),
        ]
        .spacing(18)
        .padding(24);

        // Confirmation dialog overlay (shown at app level)
        if let Some(paths) = &self.delete_confirmation {
            let count = paths.len();
            return container(
                column![
                    text(format!("Delete {} file(s)?", count)).size(20),
                    text("This will move the files to the Recycle Bin."),
                    Space::with_height(Length::Fixed(20.0)),
                    row![
                        button("Cancel").on_press(Message::CancelDelete),
                        button("Confirm Delete").on_press(Message::ConfirmDelete),
                    ]
                    .spacing(10),
                ]
                .spacing(10)
                .padding(20)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into();
        }

        // Main content area
        let content = match self.active {
            Screen::Overview => overview::view(
                self.selected_folder.as_deref(),
                self.scanning,
                &self.files,
                self.settings.font_scale,
            ),
            Screen::Duplicates => duplicates::view(
                &self.duplicates,
                &self.selected_in_group,
                self.settings.font_scale,
            ),
            Screen::Settings => settings::view(&self.settings),
        };

        // Toast row at bottom
        let toast_row = widgets::toast_list(&self.toasts, |i| Message::DismissToast(i));

        // Layout: sidebar on left, main content on right (centered with max width)
        row![
            // Sidebar
            container(sidebar_content)
                .width(Length::Fixed(230.0))
                .height(Length::Fill)
                .style(SidebarStyle),
            // Main Content, centered with max width
            container(
                container(
                    column![
                        content,
                        toast_row,
                    ]
                    .spacing(20)
                )
                .width(Length::Fixed(900.0))
                .padding(30)
                .center_x()
            )
            .width(Length::Fill)
            .height(Length::Fill)
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
