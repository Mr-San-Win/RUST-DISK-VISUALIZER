use crate::core::types::FileEntry;
use std::path::PathBuf;
use tracing::info;

pub fn export_duplicates_csv(path: PathBuf, groups: &[Vec<FileEntry>]) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path(&path)?;
    
    wtr.write_record(&["Group", "Path", "Size"])?;
    
    for (group_idx, group) in groups.iter().enumerate() {
        for f in group {
            wtr.write_record(&[
                (group_idx + 1).to_string(),
                f.path.display().to_string(),
                f.size.to_string(),
            ])?;
        }
    }
    
    wtr.flush()?;
    info!("Exported duplicates to CSV: {:?}", path);
    Ok(())
}

pub fn export_duplicates_json(path: PathBuf, groups: &[Vec<FileEntry>]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(groups)?;
    std::fs::write(&path, json)?;
    info!("Exported duplicates to JSON: {:?}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::fs;

    fn create_test_groups() -> Vec<Vec<FileEntry>> {
        vec![
            vec![
                FileEntry {
                    path: PathBuf::from("file1.txt"),
                    size: 100,
                    modified: Some(1234567890),
                    is_dir: false,
                },
                FileEntry {
                    path: PathBuf::from("file2.txt"),
                    size: 100,
                    modified: Some(1234567890),
                    is_dir: false,
                },
            ],
            vec![
                FileEntry {
                    path: PathBuf::from("file3.txt"),
                    size: 200,
                    modified: Some(1234567891),
                    is_dir: false,
                },
            ],
        ]
    }

    #[test]
    fn test_export_csv_content() {
        let groups = create_test_groups();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        export_duplicates_csv(path.clone(), &groups).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        
        assert_eq!(lines.len(), 4); // Header + 3 data rows
        assert!(lines[0].contains("Group"));
        assert!(lines[0].contains("Path"));
        assert!(lines[0].contains("Size"));
        assert!(lines[1].contains("1")); // Group 1
        assert!(lines[1].contains("file1.txt"));
        assert!(lines[2].contains("1")); // Group 1
        assert!(lines[2].contains("file2.txt"));
        assert!(lines[3].contains("2")); // Group 2
    }

    #[test]
    fn test_export_json_content() {
        let groups = create_test_groups();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        export_duplicates_json(path.clone(), &groups).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let parsed: Vec<Vec<FileEntry>> = serde_json::from_str(&content).unwrap();
        
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].len(), 2);
        assert_eq!(parsed[1].len(), 1);
        assert_eq!(parsed[0][0].path, PathBuf::from("file1.txt"));
        assert_eq!(parsed[0][1].path, PathBuf::from("file2.txt"));
    }

    #[test]
    fn test_export_csv_empty_groups() {
        let groups: Vec<Vec<FileEntry>> = vec![];
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        export_duplicates_csv(path.clone(), &groups).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 1); // Only header
    }

    #[test]
    fn test_export_json_round_trip() {
        let groups = create_test_groups();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        export_duplicates_json(path.clone(), &groups).unwrap();
        let parsed: Vec<Vec<FileEntry>> = serde_json::from_str(
            &fs::read_to_string(&path).unwrap()
        ).unwrap();

        assert_eq!(parsed.len(), groups.len());
        assert_eq!(parsed[0].len(), groups[0].len());
        assert_eq!(parsed[0][0].size, groups[0][0].size);
    }
}
