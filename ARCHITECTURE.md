# Architecture Documentation

## Overview

DiskViz is built using the Iced GUI framework, following the Model-View-Update (MVU) pattern. The application uses Rust's async runtime (Tokio) for non-blocking file system operations and maintains a clean separation between UI, business logic, and data persistence.

## Iced Application Flow

### State Management

The application state is managed in `DiskVizApp` struct (`src/app.rs`):

```rust
pub struct DiskVizApp {
    active: Screen,                    // Current screen (Overview/Duplicates/Settings)
    selected_folder: Option<PathBuf>,  // Selected directory for scanning
    scanning: bool,                     // Scanning in progress flag
    files: Vec<FileEntry>,             // Scanned file list
    duplicates: Vec<Vec<FileEntry>>,   // Duplicate groups
    selected_in_group: Vec<Vec<bool>>, // Selection state for duplicates
    delete_confirmation: Option<Vec<PathBuf>>, // Pending deletion paths
    settings: Settings,                 // Application settings
    toasts: Vec<String>,               // Toast notification messages
}
```

### Message System

All user interactions and events are represented as `Message` enum variants:

- **Navigation**: `SwitchTo(Screen)`
- **File Operations**: `ChooseFolder`, `StartScan`, `ScanFinished`
- **Duplicates**: `FindDuplicates`, `SelectDuplicate`, `DeleteSelectedDuplicates`
- **Settings**: `ToggleTheme`, `FontScaleChanged`, `SaveSettings`
- **Notifications**: `Toast(String)`, `DismissToast(usize)`

### Update Loop

The `update()` method handles all messages:

1. **Pattern Matching**: Matches incoming message to appropriate handler
2. **State Mutation**: Updates application state based on message
3. **Command Generation**: Returns `Command` for async operations or side effects
4. **Non-Blocking**: Uses `Command::perform()` for async file operations

### View Rendering

The `view()` method:
1. Renders navigation bar
2. Dispatches to screen-specific view functions
3. Renders toast notifications at bottom
4. Applies theme and font scaling from settings

## Module Structure

### Core Modules (`src/core/`)

#### `types.rs`
- **Purpose**: Shared type definitions
- **Key Types**:
  - `FileEntry`: File metadata (path, size, modified time)
  - `Settings`: Application configuration
  - `Progress`: Scan progress tracking (for future use)

#### `scan.rs`
- **Purpose**: Directory scanning logic
- **Function**: `scan_directory(path: PathBuf) -> (Vec<FileEntry>, Vec<Progress>)`
- **Implementation**:
  - Uses `ignore::WalkBuilder` for file system traversal
  - Runs in `tokio::task::spawn_blocking()` for non-blocking execution
  - Collects file metadata (size, modification time)
  - Returns file entries and progress updates

#### `dedupe.rs`
- **Purpose**: Duplicate file detection
- **Function**: `find_duplicates(files: &[FileEntry], partial_kb: u64) -> Vec<Vec<FileEntry>>`
- **Three-Stage Pipeline**:
  1. **Size Grouping**: Groups files by size (O(n) hash map)
  2. **Partial Hash**: Blake3 hash of first N KB (default 256 KB)
  3. **Full Hash**: Complete Blake3 hash for size+partial-hash matches
- **Parallelization**: Uses `rayon` for parallel full hash computation
- **Thread Safety**: Uses `Arc<Mutex<>>` for concurrent hash map updates

#### `trashcan.rs`
- **Purpose**: Safe file deletion
- **Function**: `move_to_trash(paths: &[PathBuf]) -> Result<()>`
- **Implementation**: Uses `trash` crate for platform-specific Recycle Bin/Trash integration
- **Error Handling**: Returns `anyhow::Result` with detailed error messages

#### `export.rs`
- **Purpose**: Export duplicate results
- **Functions**:
  - `export_duplicates_csv(path, groups) -> Result<()>`
  - `export_duplicates_json(path, groups) -> Result<()>`
- **Implementation**: Uses `csv` and `serde_json` for serialization

#### `config.rs`
- **Purpose**: Settings persistence
- **Functions**:
  - `load() -> Result<Settings>`
  - `save(settings: &Settings) -> Result<()>`
- **Storage**: JSON file in `%APPDATA%\diskviz\config.json`
- **Path Resolution**: Uses `directories::ProjectDirs` for cross-platform paths

#### `logging.rs`
- **Purpose**: Logging initialization
- **Function**: `init_logging()`
- **Implementation**:
  - Uses `tracing` for structured logging
  - Daily rolling log files in data directory
  - Falls back to console if file logging fails
  - Supports `RUST_LOG` environment variable

### UI Modules (`src/ui/`)

#### `overview.rs`
- **Purpose**: Main scanning interface
- **Components**:
  - Folder selection button
  - Scan button
  - File list table (path, size)
  - Progress indicator

#### `duplicates.rs`
- **Purpose**: Duplicate management interface
- **Components**:
  - Find Duplicates button
  - Duplicate groups with checkboxes
  - Delete Selected button
  - Export buttons (CSV/JSON)
  - Confirmation dialog overlay

#### `settings.rs`
- **Purpose**: Application settings
- **Components**:
  - Theme toggle checkbox
  - Font scale slider
  - Ignore globs text input
  - Partial hash KB input
  - Save/Reload buttons

#### `widgets.rs`
- **Purpose**: Reusable UI components
- **Components**:
  - `toast()`: Individual toast notification
  - `toast_list()`: Toast notification list

## Background Tasks

### Command::perform Pattern

All async operations use `Command::perform()`:

```rust
Command::perform(
    async move {
        // Async operation
        tokio::task::spawn_blocking(|| {
            // CPU-intensive or blocking operation
        }).await
    },
    |result| Message::OperationComplete(result)
)
```

### Non-Blocking Operations

- **Directory Scanning**: Runs in `spawn_blocking` to avoid blocking async runtime
- **Duplicate Detection**: CPU-intensive hashing runs in blocking task
- **File Dialogs**: Uses `rfd::AsyncFileDialog` for async file selection
- **Export Operations**: File I/O runs synchronously but wrapped in async command

## Deduplication Pipeline

### Stage 1: Size Grouping
```
Input: Vec<FileEntry>
Process: Group by file.size
Output: HashMap<u64, Vec<&FileEntry>>
Filter: Keep groups with len > 1
```

### Stage 2: Partial Hash
```
Input: Size-matched groups
Process: 
  - Read first N KB of each file
  - Compute Blake3 hash
  - Group by hash
Output: HashMap<Vec<u8>, Vec<&FileEntry>>
Filter: Keep groups with len > 1
```

### Stage 3: Full Hash
```
Input: Partial-hash-matched groups
Process:
  - Read entire file
  - Compute full Blake3 hash (parallel)
  - Group by hash
Output: HashMap<Vec<u8>, Vec<FileEntry>>
Filter: Keep groups with len > 1 (true duplicates)
```

### Performance Characteristics
- **Time Complexity**: O(n) for size grouping, O(n*k) for hashing (k = file size)
- **Space Complexity**: O(n) for file metadata, O(n) for hash maps
- **Parallelization**: Full hash stage uses rayon for CPU-bound work

## Logging & Config Paths

### Log Files
- **Windows**: `%APPDATA%\diskviz\diskviz.log`
- **macOS**: `~/Library/Application Support/diskviz/diskviz.log`
- **Linux**: `~/.local/share/diskviz/diskviz.log`
- **Rotation**: Daily rolling (appender creates new file each day)

### Config Files
- **Windows**: `%APPDATA%\diskviz\config.json`
- **macOS**: `~/Library/Application Support/diskviz/config.json`
- **Linux**: `~/.config/diskviz/config.json`

### Path Resolution
Uses `directories::ProjectDirs::from("com", "SAN", "diskviz")`:
- Config: `project_dirs.config_dir()`
- Data: `project_dirs.data_dir()`

## Dependency Notes

### Core Dependencies
- **iced 0.12**: GUI framework with tokio executor
- **tokio 1.39**: Async runtime for non-blocking operations
- **blake3 1.5.4**: Fast cryptographic hashing
- **rayon 1.10.0**: Data parallelism for duplicate detection
- **ignore 0.4.22**: Efficient file system traversal
- **trash 5.0.0**: Platform-specific Recycle Bin integration
- **directories 5.0.1**: Cross-platform path resolution
- **tracing 0.1.40**: Structured logging

### Key Features
- **Async/Await**: All file operations are non-blocking
- **Error Handling**: `anyhow` for error propagation, `thiserror` for custom errors
- **Serialization**: `serde` + `serde_json` for config and export
- **File Dialogs**: `rfd` for native file dialogs

## Design Patterns

### Model-View-Update (MVU)
- **Model**: `DiskVizApp` state
- **View**: `view()` method and UI modules
- **Update**: `update()` method handling messages

### Command Pattern
- All side effects return `Command<Message>`
- Commands are executed by Iced runtime
- Allows async operations without blocking UI

### Strategy Pattern
- Settings allow runtime configuration of algorithms
- Partial hash size configurable
- Ignore patterns configurable

## Future Enhancements

- Real-time progress updates during scanning
- Multi-threaded scanning with progress streaming
- Advanced filtering and sorting
- File preview functionality
- Network drive support
- Batch operations
- Undo/redo functionality

