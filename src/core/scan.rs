use crate::core::types::{FileEntry, Progress};
use ignore::WalkBuilder;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

pub async fn scan_directory(path: PathBuf, ignore_globs: Vec<String>) -> (Vec<FileEntry>, Vec<Progress>) {
    // Use spawn_blocking since WalkBuilder is synchronous
    let path_clone = path.clone();
    let (results, progress_updates) = tokio::task::spawn_blocking(move || {
        let mut local_results = Vec::new();
        let mut local_progress = Vec::new();
        let mut local_scanned = 0u64;
        let mut local_bytes = 0u64;

        for entry in WalkBuilder::new(&path_clone).hidden(false).build() {
            match entry {
                Ok(e) => {
                    // Check if the file/directory name matches any ignore pattern
                    if let Some(name) = e.file_name().to_str() {
                        if ignore_globs.iter().any(|g| name.contains(g)) {
                            continue;
                        }
                    }

                    // Also check if any directory name in the path matches ignore patterns
                    let should_skip = e.path().components().any(|component| {
                        if let std::path::Component::Normal(name_os) = component {
                            if let Some(name) = name_os.to_str() {
                                ignore_globs.iter().any(|g| name.contains(g))
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    });

                    if should_skip {
                        continue;
                    }

                    if let Ok(meta) = e.metadata() {
                        if meta.is_file() {
                            let size = meta.len();
                            local_scanned += 1;
                            local_bytes += size;
                            
                            let modified = meta.modified()
                                .ok()
                                .and_then(|t| {
                                    t.duration_since(UNIX_EPOCH)
                                        .ok()
                                        .map(|d| d.as_secs() as i64)
                                });

                            local_results.push(FileEntry {
                                path: e.path().to_path_buf(),
                                size,
                                modified,
                                is_dir: false,
                            });

                            local_progress.push(Progress {
                                scanned: local_scanned,
                                bytes: local_bytes,
                                current: Some(e.path().to_path_buf()),
                            });
                        }
                    }
                }
                Err(_) => {}
            }
        }

        (local_results, local_progress)
    }).await.unwrap_or_default();

    (results, progress_updates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, create_dir};
    use std::io::Write;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_scan_empty_directory() {
        let dir = TempDir::new().unwrap();
        let (files, _progress) = scan_directory(dir.path().to_path_buf(), vec![]).await;
        assert_eq!(files.len(), 0);
    }

    #[tokio::test]
    async fn test_scan_single_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        let (files, _progress) = scan_directory(dir.path().to_path_buf(), vec![]).await;
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, file_path);
        assert_eq!(files[0].size, 12);
        assert_eq!(files[0].is_dir, false);
    }

    #[tokio::test]
    async fn test_scan_multiple_files() {
        let dir = TempDir::new().unwrap();
        let file1 = dir.path().join("file1.txt");
        let file2 = dir.path().join("file2.txt");
        File::create(&file1).unwrap().write_all(b"content1").unwrap();
        File::create(&file2).unwrap().write_all(b"content2 longer").unwrap();

        let (files, _progress) = scan_directory(dir.path().to_path_buf(), vec![]).await;
        assert_eq!(files.len(), 2);
        let sizes: Vec<u64> = files.iter().map(|f| f.size).collect();
        assert!(sizes.contains(&8));
        assert!(sizes.contains(&15));
    }

    #[tokio::test]
    async fn test_scan_nested_directories() {
        let dir = TempDir::new().unwrap();
        let subdir = dir.path().join("subdir");
        create_dir(&subdir).unwrap();
        
        let file1 = dir.path().join("file1.txt");
        let file2 = subdir.join("file2.txt");
        File::create(&file1).unwrap().write_all(b"root").unwrap();
        File::create(&file2).unwrap().write_all(b"nested").unwrap();

        let (files, _progress) = scan_directory(dir.path().to_path_buf(), vec![]).await;
        assert_eq!(files.len(), 2);
    }

    #[tokio::test]
    async fn test_scan_file_sizes_aggregation() {
        let dir = TempDir::new().unwrap();
        let sizes = vec![100, 200, 300];
        for (i, &size) in sizes.iter().enumerate() {
            let file = dir.path().join(format!("file{}.txt", i));
            let content = vec![0u8; size];
            File::create(&file).unwrap().write_all(&content).unwrap();
        }

        let (files, progress) = scan_directory(dir.path().to_path_buf(), vec![]).await;
        assert_eq!(files.len(), 3);
        
        let total_size: u64 = files.iter().map(|f| f.size).sum();
        assert_eq!(total_size, 600);

        // Check progress tracking
        assert_eq!(progress.len(), 3);
        assert_eq!(progress[0].scanned, 1);
        assert_eq!(progress[1].scanned, 2);
        assert_eq!(progress[2].scanned, 3);
        assert_eq!(progress[2].bytes, 600);
    }

    #[tokio::test]
    async fn test_scan_only_files_not_directories() {
        let dir = TempDir::new().unwrap();
        let subdir = dir.path().join("subdir");
        create_dir(&subdir).unwrap();
        
        let file = dir.path().join("file.txt");
        File::create(&file).unwrap().write_all(b"content").unwrap();

        let (files, _progress) = scan_directory(dir.path().to_path_buf(), vec![]).await;
        // Should only include file, not directory
        assert_eq!(files.len(), 1);
        assert!(!files[0].is_dir);
    }
}
