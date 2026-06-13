use crate::core::types::Settings;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

fn config_dir() -> Result<PathBuf> {
    let proj = ProjectDirs::from("com", "SAN", "diskviz")
        .context("cannot determine config dir")?;
    let dir = proj.config_dir().to_path_buf();
    fs::create_dir_all(&dir).ok();
    Ok(dir)
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

pub fn load() -> Result<Settings> {
    let p = config_path()?;
    if !p.exists() {
        return Ok(Settings::default());
    }
    let bytes = fs::read(&p)?;
    let s: Settings = serde_json::from_slice(&bytes)?;
    Ok(s)
}

pub fn save(s: &Settings) -> Result<()> {
    let p = config_path()?;
    let data = serde_json::to_vec_pretty(s)?;
    fs::write(p, data)?;
    Ok(())
}

// Test helper functions removed - using direct path-based functions instead

// Test-only functions
#[cfg(test)]
pub fn load_from_path(path: &PathBuf) -> Result<Settings> {
    if !path.exists() {
        return Ok(Settings::default());
    }
    let bytes = fs::read(path)?;
    let s: Settings = serde_json::from_slice(&bytes)?;
    Ok(s)
}

#[cfg(test)]
pub fn save_to_path(s: &Settings, path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_vec_pretty(s)?;
    fs::write(path, data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_settings_serialization() {
        let settings = Settings {
            theme_dark: false,
            font_scale: 1.25,
            ignore_globs: vec!["test".into(), "temp".into()],
            partial_hash_kb: 512,
        };

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: Settings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings.theme_dark, deserialized.theme_dark);
        assert_eq!(settings.font_scale, deserialized.font_scale);
        assert_eq!(settings.ignore_globs, deserialized.ignore_globs);
        assert_eq!(settings.partial_hash_kb, deserialized.partial_hash_kb);
    }

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.theme_dark, true);
        assert_eq!(settings.font_scale, 1.0);
        assert_eq!(settings.partial_hash_kb, 256);
        assert!(settings.ignore_globs.contains(&"node_modules".to_string()));
    }

    #[test]
    fn test_config_save_and_load_round_trip() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("config.json");

        let original_settings = Settings {
            theme_dark: false,
            font_scale: 1.5,
            ignore_globs: vec!["custom".into()],
            partial_hash_kb: 128,
        };

        // Save
        save_to_path(&original_settings, &config_path).unwrap();
        assert!(config_path.exists());

        // Load
        let loaded_settings = load_from_path(&config_path).unwrap();

        assert_eq!(original_settings.theme_dark, loaded_settings.theme_dark);
        assert_eq!(original_settings.font_scale, loaded_settings.font_scale);
        assert_eq!(original_settings.ignore_globs, loaded_settings.ignore_globs);
        assert_eq!(original_settings.partial_hash_kb, loaded_settings.partial_hash_kb);
    }

    #[test]
    fn test_config_load_nonexistent_returns_default() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("nonexistent.json");

        let settings = load_from_path(&config_path).unwrap();
        assert_eq!(settings.theme_dark, Settings::default().theme_dark);
        assert_eq!(settings.font_scale, Settings::default().font_scale);
    }

    #[test]
    fn test_config_path_creation() {
        let dir = TempDir::new().unwrap();
        let config_dir = dir.path().join("config");
        fs::create_dir_all(&config_dir).unwrap();
        let config_path = config_dir.join("config.json");

        let settings = Settings::default();
        save_to_path(&settings, &config_path).unwrap();

        assert!(config_path.exists());
        assert!(config_dir.exists());
    }

    #[test]
    fn test_config_invalid_json_handling() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("invalid.json");
        fs::write(&config_path, b"invalid json content").unwrap();

        let result = load_from_path(&config_path);
        assert!(result.is_err());
    }
}
