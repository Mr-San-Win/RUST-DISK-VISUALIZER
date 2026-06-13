use diskviz::core::dedupe;
use diskviz::core::scan;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio;

#[tokio::test]
async fn test_integration_duplicate_detection() {
    let dir = TempDir::new().unwrap();
    
    // Create two identical files
    let content = b"identical content for duplicate test";
    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");
    
    File::create(&file1).unwrap().write_all(content).unwrap();
    File::create(&file2).unwrap().write_all(content).unwrap();

    // Scan directory
    let (files, _progress) = scan::scan_directory(dir.path().to_path_buf(), vec![]).await;
    assert_eq!(files.len(), 2);

    // Find duplicates
    let duplicates = dedupe::find_duplicates(&files, 1); // Use 1 KB for small files

    // Should find one group of 2 duplicates
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].len(), 2);
    
    // Verify the files are the same
    let paths: Vec<&PathBuf> = duplicates[0].iter().map(|f| &f.path).collect();
    assert!(paths.iter().any(|p| p.ends_with("file1.txt")));
    assert!(paths.iter().any(|p| p.ends_with("file2.txt")));
}

#[tokio::test]
async fn test_integration_duplicate_with_different_file() {
    let dir = TempDir::new().unwrap();
    
    // Create two identical files and one different file
    let identical_content = b"identical content";
    let different_content = b"different content";
    
    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");
    let file3 = dir.path().join("file3.txt");
    
    File::create(&file1).unwrap().write_all(identical_content).unwrap();
    File::create(&file2).unwrap().write_all(identical_content).unwrap();
    File::create(&file3).unwrap().write_all(different_content).unwrap();

    // Scan directory
    let (files, _progress) = scan::scan_directory(dir.path().to_path_buf(), vec![]).await;
    assert_eq!(files.len(), 3);

    // Find duplicates
    let duplicates = dedupe::find_duplicates(&files, 1);

    // Should find one group of 2 duplicates (file1 and file2)
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].len(), 2);
    
    // Verify file3 is not in the duplicate group
    let paths: Vec<&PathBuf> = duplicates[0].iter().map(|f| &f.path).collect();
    assert!(!paths.iter().any(|p| p.ends_with("file3.txt")));
}

#[tokio::test]
async fn test_integration_off_by_one_byte_difference() {
    let dir = TempDir::new().unwrap();
    
    // Create two identical files and one with one byte difference
    let content1 = b"almost identical content";
    let mut content2 = content1.to_vec();
    content2[10] = b'X'; // Change one byte
    
    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");
    let file3 = dir.path().join("file3.txt");
    
    File::create(&file1).unwrap().write_all(content1).unwrap();
    File::create(&file2).unwrap().write_all(content1).unwrap();
    File::create(&file3).unwrap().write_all(&content2).unwrap();

    // Scan directory
    let (files, _progress) = scan::scan_directory(dir.path().to_path_buf(), vec![]).await;
    assert_eq!(files.len(), 3);

    // Find duplicates
    let duplicates = dedupe::find_duplicates(&files, 1);

    // Should find one group of 2 duplicates (file1 and file2)
    // file3 differs by one byte, so it's not a duplicate
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].len(), 2);
}

#[tokio::test]
async fn test_integration_no_duplicates_different_sizes() {
    let dir = TempDir::new().unwrap();
    
    // Create files with different sizes
    let file1 = dir.path().join("small.txt");
    let file2 = dir.path().join("large.txt");
    
    File::create(&file1).unwrap().write_all(b"small").unwrap();
    File::create(&file2).unwrap().write_all(b"much larger content here").unwrap();

    // Scan directory
    let (files, _progress) = scan::scan_directory(dir.path().to_path_buf(), vec![]).await;
    assert_eq!(files.len(), 2);

    // Find duplicates
    let duplicates = dedupe::find_duplicates(&files, 256);

    // Should find no duplicates (different sizes)
    assert_eq!(duplicates.len(), 0);
}

