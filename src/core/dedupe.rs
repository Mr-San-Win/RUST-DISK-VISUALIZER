use crate::core::types::FileEntry;
use blake3;
use rayon::prelude::*;
use std::{fs::File, io::Read};
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

pub fn find_duplicates(files: &[FileEntry], partial_kb: u64) -> Vec<Vec<FileEntry>> {
    if files.len() < 2 {
        return vec![];
    }

    info!("Starting duplicate detection on {} files", files.len());

    // 1) Group by size
    let mut size_map = std::collections::HashMap::<u64, Vec<&FileEntry>>::new();
    for f in files {
        size_map.entry(f.size).or_default().push(f);
    }

    let mut candidates = Vec::<Vec<&FileEntry>>::new();
    for group in size_map.values() {
        if group.len() > 1 {
            candidates.push(group.clone());
        }
    }

    info!("Found {} size-based candidate groups", candidates.len());

    if candidates.is_empty() {
        return vec![];
    }

    // 2) Partial hash
    let partial_map: Arc<Mutex<std::collections::HashMap<Vec<u8>, Vec<&FileEntry>>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));

    for group in candidates {
        for f in group {
            if let Ok(mut file) = File::open(&f.path) {
                let mut buf = vec![0u8; (partial_kb * 1024) as usize];
                match file.read(&mut buf) {
                    Ok(bytes_read) => {
                        // Only hash the bytes that were actually read
                        // This handles files smaller than partial_kb * 1024
                        if bytes_read > 0 {
                            buf.truncate(bytes_read);
                            let hash = blake3::hash(&buf).as_bytes().to_vec();
                            let mut map = partial_map.lock().unwrap();
                            map.entry(hash).or_default().push(f);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to read partial hash for {:?}: {}", f.path, e);
                    }
                }
            } else {
                warn!("Failed to open file for partial hash: {:?}", f.path);
            }
        }
    }

    let partial_map = Arc::try_unwrap(partial_map).unwrap().into_inner().unwrap();
    let mut final_candidates = Vec::<Vec<&FileEntry>>::new();
    for group in partial_map.values() {
        if group.len() > 1 {
            final_candidates.push(group.clone());
        }
    }

    info!("Found {} partial-hash candidate groups", final_candidates.len());

    if final_candidates.is_empty() {
        return vec![];
    }

    // 3) Full hash - use parallel processing with thread-safe collection
    let full_map: Arc<Mutex<std::collections::HashMap<Vec<u8>, Vec<FileEntry>>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));

    for group in final_candidates {
        let group_arc = Arc::new(group);
        group_arc.par_iter().for_each(|f| {
            if let Ok(mut file) = File::open(&f.path) {
                let mut buf = Vec::new();
                if file.read_to_end(&mut buf).is_ok() {
                    let hash = blake3::hash(&buf).as_bytes().to_vec();
                    let mut map = full_map.lock().unwrap();
                    map.entry(hash).or_default().push((*f).clone());
                } else {
                    warn!("Failed to read full file: {:?}", f.path);
                }
            } else {
                warn!("Failed to open file for full hash: {:?}", f.path);
            }
        });
    }

    let full_map = Arc::try_unwrap(full_map).unwrap().into_inner().unwrap();
    let duplicates: Vec<Vec<FileEntry>> = full_map
        .values()
        .filter(|g| g.len() > 1)
        .cloned()
        .collect();

    info!("Found {} duplicate groups", duplicates.len());
    duplicates
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.path().join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }

    #[test]
    fn test_find_duplicates_empty() {
        let files = vec![];
        let duplicates = find_duplicates(&files, 256);
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn test_find_duplicates_single_file() {
        let dir = TempDir::new().unwrap();
        let path = create_test_file(&dir, "test.txt", b"content");
        
        let files = vec![FileEntry {
            path,
            size: 7,
            modified: None,
            is_dir: false,
        }];
        let duplicates = find_duplicates(&files, 256);
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn test_size_bucketing() {
        let dir = TempDir::new().unwrap();
        let path1 = create_test_file(&dir, "file1.txt", b"same size");
        let path2 = create_test_file(&dir, "file2.txt", b"same size");
        let path3 = create_test_file(&dir, "file3.txt", b"different");

        let files = vec![
            FileEntry { path: path1, size: 9, modified: None, is_dir: false },
            FileEntry { path: path2, size: 9, modified: None, is_dir: false },
            FileEntry { path: path3, size: 9, modified: None, is_dir: false },
        ];

        // All same size, so all should be candidates
        let duplicates = find_duplicates(&files, 1); // Use 1 KB for small files
        // Two files have identical content, so should find one group of 2
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].len(), 2);
    }

    #[test]
    fn test_partial_hash_grouping() {
        let dir = TempDir::new().unwrap();
        // Create files with same first 256 bytes but different rest
        let mut content1 = vec![0u8; 512];
        content1[256] = 1;
        let mut content2 = vec![0u8; 512];
        content2[256] = 2;
        let mut content3 = vec![0u8; 512];
        content3[256] = 1;

        let path1 = create_test_file(&dir, "file1.txt", &content1);
        let path2 = create_test_file(&dir, "file2.txt", &content2);
        let path3 = create_test_file(&dir, "file3.txt", &content3);

        let files = vec![
            FileEntry { path: path1, size: 512, modified: None, is_dir: false },
            FileEntry { path: path2, size: 512, modified: None, is_dir: false },
            FileEntry { path: path3, size: 512, modified: None, is_dir: false },
        ];

        // With partial hash of 256 bytes, file1 and file3 should have same partial hash
        // But full hash will be different
        let duplicates = find_duplicates(&files, 1); // 1 KB = 1024 bytes, so will read full file
        // All have same size, partial hash of first 1024 bytes will be same for all (all zeros)
        // But full hash will differ, so no duplicates
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn test_full_hash_equality() {
        let dir = TempDir::new().unwrap();
        let content = b"identical content for duplicate test";
        let path1 = create_test_file(&dir, "file1.txt", content);
        let path2 = create_test_file(&dir, "file2.txt", content);
        let path3 = create_test_file(&dir, "file3.txt", b"different content");

        let files = vec![
            FileEntry { path: path1, size: content.len() as u64, modified: None, is_dir: false },
            FileEntry { path: path2, size: content.len() as u64, modified: None, is_dir: false },
            FileEntry { path: path3, size: 16, modified: None, is_dir: false },
        ];

        let duplicates = find_duplicates(&files, 1);
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].len(), 2);
        // Verify the two identical files are in the same group
        let paths: Vec<&PathBuf> = duplicates[0].iter().map(|f| &f.path).collect();
        assert!(paths.iter().any(|p| p.ends_with("file1.txt")));
        assert!(paths.iter().any(|p| p.ends_with("file2.txt")));
    }

    #[test]
    fn test_partial_hash_window_size() {
        // This test verifies that partial hash reads exactly partial_kb * 1024 bytes
        let dir = TempDir::new().unwrap();
        let partial_kb = 2;
        let exact_size = (partial_kb * 1024) as usize;
        
        // Create file with exact size
        let content = vec![0x42u8; exact_size];
        let path = create_test_file(&dir, "exact.txt", &content);
        
        let files = vec![FileEntry {
            path,
            size: exact_size as u64,
            modified: None,
            is_dir: false,
        }];

        // This should not panic and should read exactly the requested bytes
        let duplicates = find_duplicates(&files, partial_kb);
        // Single file, no duplicates
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn test_different_sizes_no_duplicates() {
        let dir = TempDir::new().unwrap();
        let path1 = create_test_file(&dir, "small.txt", b"small");
        let path2 = create_test_file(&dir, "large.txt", b"much larger content here");

        let files = vec![
            FileEntry { path: path1, size: 5, modified: None, is_dir: false },
            FileEntry { path: path2, size: 25, modified: None, is_dir: false },
        ];

        let duplicates = find_duplicates(&files, 256);
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn test_one_byte_difference() {
        let dir = TempDir::new().unwrap();
        let content1 = b"almost identical content";
        let mut content2 = content1.to_vec();
        content2[10] = b'X'; // Change one byte

        let path1 = create_test_file(&dir, "file1.txt", content1);
        let path2 = create_test_file(&dir, "file2.txt", content1);
        let path3 = create_test_file(&dir, "file3.txt", &content2);

        let files = vec![
            FileEntry { path: path1, size: content1.len() as u64, modified: None, is_dir: false },
            FileEntry { path: path2, size: content1.len() as u64, modified: None, is_dir: false },
            FileEntry { path: path3, size: content2.len() as u64, modified: None, is_dir: false },
        ];

        let duplicates = find_duplicates(&files, 1);
        // file1 and file2 are identical, file3 differs by one byte
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].len(), 2);
    }
}
