DiskViz – A Modern Disk Usage & Duplicate Analysis Utility

A high-performance, cross-platform storage analysis application built with Rust and Iced.

DiskViz is a fully featured desktop application designed to provide fast, safe, and intuitive disk inspection capabilities.
Built on a robust, asynchronous Rust backend and rendered through the Iced GUI framework, DiskViz combines systems-level performance, interactive visualization, and practical file-management workflows in a single unified tool.

This project demonstrates production-grade engineering practices including non-blocking UI orchestration, multi-stage hashing pipelines, cross-platform filesystem abstraction, persistent configuration management, reusable UI components, and architectural separation between the core engine and GUI layer.

1. Introduction

DiskViz addresses a common yet unserved need in desktop environments:
a user-friendly but technically accurate tool for analyzing disk usage, tracing large files, identifying duplicates, and managing storage safely.

The application provides:

A responsive GUI with clear navigation and accessible controls

Accurate filesystem scanning with metadata extraction

Advanced duplicate detection using Blake3 hashing

Safe file operations that never perform destructive deletes

Structured export options for reporting and analysis

Persistent settings to preserve user preferences

A themeable and scalable interface that adapts to user accessibility needs

DiskViz is not a prototype; it is engineered as a complete utility, meeting academic evaluation criteria and resembling professional-grade tooling.

2. Key Features
2.1 Core Capabilities

Recursive Directory Scanning
Highly optimized traversal with metadata collection (size, modification time, file type).

Three-Stage Duplicate Detection Pipeline

Size comparison

Partial hash (configurable KB)

Full Blake3 hash
This reduces computational overhead while maintaining high accuracy.

Safe Deletion Workflow
Files are never permanently removed; they are moved to the OS-native
Trash/Recycle Bin for user-driven recovery.

Data Export
Export duplicate analysis as:

CSV (tabular)

JSON (structured)
Suitable for audits, reports, or programmatic ingestion.

2.2 User Experience & Interface Design

Three dedicated screens accessible via sidebar navigation:

Overview

Duplicates

Settings

Dynamic Theme System
Instant switching between light and dark modes.

UI Scaling
Adjustable font scaling between 1.0x and 1.5x, improving readability.

Toast Notifications
Clear, unobtrusive system feedback for user actions.

Confirmation Modals
All destructive actions require explicit confirmation.

Custom UI Components
Built from reusable widget modules for consistency.

2.3 Configuration & Persistence

DiskViz includes a structured configuration engine:

Stored as a JSON file in the user's configuration directory

Automatically loaded at startup

Respects OS-specific directory conventions

Supports:

Theme preference

Font scaling

Ignore glob patterns

Partial hash block size

Additional UI parameters

3. Application Screens
3.1 Overview

A high-level entry point for:

Selecting a directory to scan

Initiating analysis

Viewing file tables (path, size, metadata)

Inspecting scan progress in real time

The UI remains fully responsive during long-running scans due to asynchronous task handling.

3.2 Duplicate Analysis

This screen provides:

Grouped duplicate sets

Per-file checkboxes for selective deletion

Export controls

File path and size visualization

Integrated delete-confirmation modal

The duplicate engine is optimized for large datasets and heavily nested directories.

3.3 Settings

Includes:

Theme switch

Font size scaling

Ignored directory patterns (globs)

Partial hash block size

Save & reload settings options

Changes apply immediately and are persisted across sessions.

4. Configuration Details
4.1 File Locations

All settings are stored automatically in a platform-correct location:

Platform	Path
Windows	%APPDATA%\SAN\diskviz\config.json
macOS	~/Library/Application Support/SAN/diskviz/config.json
Linux	~/.config/SAN/diskviz/config.json
4.2 Example Configuration File
{
  "theme_dark": true,
  "font_scale": 1.2,
  "ignore_globs": [".git", "node_modules", "target"],
  "partial_hash_kb": 256
}

5. Export Formats
5.1 CSV Export

Provides a spreadsheet-ready table structure:

Group,Path,Size
1,C:/Users/.../duplicate1.png,152034
1,C:/Users/.../duplicate2.png,152034


Ideal for analytical workflows and documentation.

5.2 JSON Export

Structured export suitable for programmatic data processing:

[
  [
    {"path": "...", "size": 152034, "modified": 1699831247, "is_dir": false},
    {"path": "...", "size": 152034, "modified": 1699831247, "is_dir": false}
  ]
]

6. Architecture Overview
6.1 Project Structure
diskviz/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Global app state, message routing
│   ├── core/
│   │   ├── scan.rs      # Filesystem scanning engine
│   │   ├── dedupe.rs    # Duplicate analysis pipeline
│   │   ├── export.rs    # CSV/JSON output functions
│   │   ├── trashcan.rs  # Safe deletion abstraction
│   │   ├── config.rs    # Configuration persistence
│   │   ├── logging.rs   # Structured logging
│   │   └── types.rs     # Strongly typed DTOs and shared structs
│   └── ui/
│       ├── overview.rs
│       ├── duplicates.rs
│       ├── settings.rs
│       ├── widgets.rs   # Reusable components
│       └── styles.rs    # Custom styling
├── Cargo.toml
├── README.md
└── USER_GUIDE.md


The architecture isolates:

Core logic (pure Rust)

UI rendering (Iced)

Async orchestration (Tokio + command futures)

Data modeling

Config persistence

This separation improves testability, maintainability, and modularity.

7. Building the Application
7.1 Clone the Repository
git clone https://github.com/<your-username>/diskviz.git
cd diskviz

7.2 Build (Debug)
cargo build

7.3 Build (Release)
cargo build --release

7.4 Run
cargo run

7.5 Binary Output

Windows: target/release/diskviz.exe

macOS/Linux: target/release/diskviz

8. Dependencies

DiskViz integrates a curated set of libraries:

Iced – Declarative GUI framework

Tokio – Async runtime

Blake3 – High-performance hashing

Rayon – Parallelism engine

Ignore – Filesystem traversal utilities

RFD – Native file dialogs (async)

Trash – Cross-platform safe deletion

Serde – JSON parsing

Humansize – Human-readable units

Directories – OS-specific directory resolution

9. Author

San Win
Software Engineering Student
King Mongkut’s Institute of Technology Ladkrabang (KMITL), Thailand
