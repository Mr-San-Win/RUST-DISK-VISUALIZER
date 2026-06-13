mod app;
mod core;
mod ui;

use app::DiskVizApp;
use iced::Application;

fn main() -> iced::Result {
    core::logging::init_logging();
    DiskVizApp::run(iced::Settings::default())
}
