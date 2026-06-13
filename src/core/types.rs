use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified: Option<i64>,
    pub is_dir: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Progress {
    pub scanned: u64,
    pub bytes: u64,
    pub current: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme_dark: bool,
    pub font_scale: f32,
    pub ignore_globs: Vec<String>,
    pub partial_hash_kb: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme_dark: true,
            font_scale: 1.0,
            ignore_globs: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
            ],
            partial_hash_kb: 256,
        }
    }
}

