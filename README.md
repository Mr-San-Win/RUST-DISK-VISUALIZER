DiskViz – Disk Usage Visualizer

DiskViz is a modern, cross-platform desktop application built in Rust using the Iced GUI framework.
It provides fast, safe, and intuitive disk analysis features — including directory scanning, duplicate detection, and export/reporting tools — designed for both technical and non-technical users.

📘 Overview

DiskViz enables you to:

Analyze disk usage with directory-wide scanning

Identify duplicate files using a multi-stage hashing algorithm

Safely delete duplicates (sent to Recycle Bin/Trash, never permanently deleted)

Export scan results to CSV and JSON

Customize the experience with themes, font scaling, and ignore patterns

⚙️ Core Features
Core Functionality

Directory Scanning — Recursive filesystem traversal

Duplicate Detection — (1) size match → (2) partial hash → (3) full hash (Blake3)

Safe Deletion — Files moved to system trash (cross-platform)

CSV & JSON Export — For external reporting

Config Persistence — Settings saved to JSON

User Interface

Three-screen navigation: Overview, Duplicates, Settings

Light/Dark modes

Font scaling (1.0x–1.5x)

Toast notifications

Confirmation dialogs for destructive actions

Advanced Features

Ignore glob patterns (e.g., .git, node_modules, target)

Fully asynchronous operations ensuring no UI freezes

Structured logging for debugging and performance evaluation

🪟 Application Screens
1. Overview

Select and scan folders

View indexed files (path, size)

Real-time progress indicator

2. Duplicates

Find duplicates

Preview duplicate groups

Select & delete duplicates

Export results to CSV and JSON

3. Settings

Theme toggle

Font scale adjustment

Ignore globs input (folder1, folder2, .git, node_modules)

Adjust partial hash size

Save / reload settings

🧩 Configuration
Settings File Location

Saved automatically in system-specific directories:

OS	Path
Windows	%APPDATA%\SAN\diskviz\config.json
macOS	~/Library/Application Support/SAN/diskviz/config.json
Linux	~/.config/SAN/diskviz/config.json
Example Settings
{
  "theme_dark": true,
  "font_scale": 1.0,
  "ignore_globs": [".git", "node_modules", "target"],
  "partial_hash_kb": 256
}

📤 Export Formats
CSV Export

Outputs a tabular list of duplicates:

Group	Path	Size (bytes)
1	C:/folder/file.txt	2048
1	C:/backup/file.txt	2048
JSON Export

Exports duplicates in structured arrays:

[
  [
    {"path": "/path/file1.txt", "size": 2048, "modified": 1699829910},
    {"path": "/path/file2.txt", "size": 2048, "modified": 1699829910}
  ]
]

🛡️ Safety Features

All file deletions require confirmation

Files are always moved to Trash/Recycle Bin

No permanent deletion or data overwrite

Toast notifications for every action

📂 Project Structure
diskviz/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Main app state and message handling
│   ├── core/
│   │   ├── scan.rs      # Directory scanning engine
│   │   ├── dedupe.rs    # Duplicate detection logic
│   │   ├── export.rs    # CSV & JSON export
│   │   ├── trashcan.rs  # Safe file deletion
│   │   ├── config.rs    # Settings loader/saver
│   │   ├── logging.rs   # Logging setup
│   │   └── types.rs     # Common types and structs
│   └── ui/
│       ├── overview.rs
│       ├── duplicates.rs
│       ├── settings.rs
│       ├── widgets.rs
│       └── styles.rs
├── Cargo.toml
├── README.md
└── USER_GUIDE.md

⚙️ Build & Run
Clone
git clone https://github.com/<your-username>/diskviz.git
cd diskviz

Build
cargo build --release

Run
cargo run

Executable Output

Windows → target/release/diskviz.exe

macOS/Linux → target/release/diskviz

📦 Dependencies

Iced — GUI framework

Tokio — Asynchronous runtime

Blake3 — Fast hashing

Rayon — Parallel processing

Ignore — Efficient file traversal

Trash — Cross-platform safe deletion

RFD — Native async file dialogs

Humansiize — Readable file size formatting

Serde — JSON serialization

👤 Author

San Win
Software Engineering Student
King Mongkut’s Institute of Technology Ladkrabang (KMITL), Thailand
