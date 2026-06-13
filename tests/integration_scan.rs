use diskviz::core::scan;
use std::fs::{File, create_dir};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio;

#[tokio::test]
async fn test_integration_scan_various_files() {
    let dir = TempDir::new().unwrap();
    
    // Create files of various sizes
    let small_file = dir.path().join("small.txt");
    File::create(&small_file).unwrap().write_all(b"small").unwrap();
    
    let medium_file = dir.path().join("medium.txt");
    let medium_content = vec![0u8; 1024];
    File::create(&medium_file).unwrap().write_all(&medium_content).unwrap();
    
    let large_file = dir.path().join("large.txt");
    let large_content = vec![0u8; 10 * 1024];
    File::create(&large_file).unwrap().write_all(&large_content).unwrap();

    let (files, _progress) = scan::scan_directory(dir.path().to_path_buf(), vec![]).await;

    assert_eq!(files.len(), 3);
    
    let sizes: Vec<u64> = files.iter().map(|f| f.size).collect();
    assert!(sizes.contains(&5));
    assert!(sizes.contains(&1024));
    assert!(sizes.contains(&(10 * 1024)));
}

#[tokio::test]
async fn test_integration_scan_nested_structure() {
    let dir = TempDir::new().unwrap();
    
    // Create nested directory structure
    let subdir1 = dir.path().join("level1");
    create_dir(&subdir1).unwrap();
    
    let subdir2 = subdir1.join("level2");
    create_dir(&subdir2).unwrap();
    
    // Create files at different levels
    let root_file = dir.path().join("root.txt");
    let level1_file = subdir1.join("level1.txt");
    let level2_file = subdir2.join("level2.txt");
    
    File::create(&root_file).unwrap().write_all(b"root").unwrap();
    File::create(&level1_file).unwrap().write_all(b"level1").unwrap();
    File::create(&level2_file).unwrap().write_all(b"level2").unwrap();

    let (files, _progress) = scan::scan_directory(dir.path().to_path_buf(), vec![]).await;

    assert_eq!(files.len(), 3);
    
    let paths: Vec<&PathBuf> = files.iter().map(|f| &f.path).collect();
    assert!(paths.iter().any(|p| p.ends_with("root.txt")));
    assert!(paths.iter().any(|p| p.ends_with("level1.txt")));
    assert!(paths.iter().any(|p| p.ends_with("level2.txt")));
}

#[tokio::test]
async fn test_integration_scan_count_and_sizes() {
    let dir = TempDir::new().unwrap();
    
    // Create multiple files with known sizes
    let file_sizes = vec![100, 200, 300, 400, 500];
    for (i, &size) in file_sizes.iter().enumerate() {
        let file = dir.path().join(format!("file{}.txt", i));
        let content = vec![0u8; size];
        File::create(&file).unwrap().write_all(&content).unwrap();
    }

    let (files, progress) = scan::scan_directory(dir.path().to_path_buf(), vec![]).await;

    assert_eq!(files.len(), 5);
    
    let total_size: u64 = files.iter().map(|f| f.size).sum();
    assert_eq!(total_size, 1500);
    
    // Verify progress tracking
    assert_eq!(progress.len(), 5);
    assert_eq!(progress.last().unwrap().scanned, 5);
    assert_eq!(progress.last().unwrap().bytes, 1500);
}

#[tokio::test]
async fn test_integration_scan_empty_directories_ignored() {
    let dir = TempDir::new().unwrap();
    
    // Create empty subdirectories
    let empty_dir1 = dir.path().join("empty1");
    let empty_dir2 = dir.path().join("empty2");
    create_dir(&empty_dir1).unwrap();
    create_dir(&empty_dir2).unwrap();
    
    // Create one file
    let file = dir.path().join("file.txt");
    File::create(&file).unwrap().write_all(b"content").unwrap();

    let (files, _progress) = scan::scan_directory(dir.path().to_path_buf(), vec![]).await;

    // Should only count the file, not directories
    assert_eq!(files.len(), 1);
    assert!(!files[0].is_dir);
}

