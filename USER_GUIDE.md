# User Guide

## Quick Start

### Step 1: Choose a Folder
1. Launch DiskViz
2. Click **"Choose Folder"** button on the Overview screen
3. Select the directory you want to analyze
4. The selected folder path will be displayed below the buttons

### Step 2: Scan Directory
1. Click **"Scan"** button
2. Wait for scanning to complete (progress shown in status text)
3. View results in the file list table:
   - **Path**: Full file path
   - **Size**: File size in human-readable format (KB, MB, GB)

### Step 3: View Results
- Scroll through the file list to see all scanned files
- Files are displayed in a scrollable table
- Empty state shows "No files scanned yet" if no scan has been performed

## Finding Duplicates

### Step 1: Start Duplicate Detection
1. Navigate to **Duplicates** screen using the top navigation bar
2. Ensure you have scanned files in the Overview screen
3. Click **"Find Duplicates"** button
4. Wait for detection to complete (may take several minutes for large directories)
5. Toast notification will show number of duplicate groups found

### Step 2: Review Duplicate Groups
- Each group shows files with identical content
- Groups are numbered (Group 1, Group 2, etc.)
- Each file in a group displays:
  - Checkbox for selection
  - File path
  - File size

### Step 3: Select Files to Delete
1. Check the boxes next to files you want to delete
2. **Important**: Keep at least one file from each group to avoid data loss
3. Selected files are highlighted

### Step 4: Delete Selected Files
1. Click **"Delete Selected"** button
2. Review the confirmation dialog:
   - Shows number of files to be deleted
   - Reminds that files will be moved to Recycle Bin
3. Click **"Confirm Delete"** to proceed or **"Cancel"** to abort
4. Toast notification confirms deletion or shows errors

### Step 5: Export Results (Optional)
1. Click **"Export CSV"** to save results as CSV file
2. Or click **"Export JSON"** to save as JSON file
3. Choose save location in file dialog
4. Toast notification confirms successful export

## Settings Configuration

### Accessing Settings
1. Click **"Settings"** in the top navigation bar
2. All settings are displayed in a single screen

### Theme Selection
- **Toggle**: Check/uncheck "Theme (Dark)" checkbox
- **Effect**: Application theme changes immediately
- **Options**: Dark theme (default) or Light theme

### Font Scaling
- **Slider**: Adjust font scale from 1.0x to 1.5x
- **Purpose**: Improves accessibility for users with visual impairments
- **Effect**: UI padding and spacing scale proportionally
- **Display**: Current scale value shown next to slider (e.g., "1.25x")

### Ignore Patterns
- **Input**: Comma-separated list of glob patterns
- **Purpose**: Exclude directories from scanning
- **Examples**:
  - `node_modules` - Excludes node_modules directories
  - `target` - Excludes Rust build directories
  - `.git` - Excludes Git repositories
  - `*.tmp` - Excludes temporary files
- **Default**: `node_modules, target`
- **Usage**: Enter patterns separated by commas (e.g., `node_modules, target, .git`)

### Partial Hash Size
- **Input**: Number in KB for partial hash comparison
- **Purpose**: Controls balance between speed and accuracy in duplicate detection
- **Default**: 256 KB
- **Recommendations**:
  - **Smaller (64-128 KB)**: Faster but may have more false positives
  - **Larger (512-1024 KB)**: Slower but more accurate
  - **Default (256 KB)**: Good balance for most use cases

### Saving Settings
1. Click **"Save Settings"** button
2. Settings are saved to `%APPDATA%\diskviz\config.json`
3. Toast notification confirms save or shows errors
4. Settings persist across application restarts

### Reloading Settings
1. Click **"Reload"** button
2. Settings are reloaded from disk
3. Any unsaved changes are discarded
4. Toast notification confirms reload or shows errors

## Safety Notes

### Recycle Bin
- **All deletions** go to Recycle Bin (Windows) or Trash (macOS/Linux)
- Files can be **restored** from Recycle Bin if deleted by mistake
- **Permanent deletion** requires emptying Recycle Bin separately
- **Network drives**: May not support Recycle Bin (check your OS)

### Best Practices
1. **Review before deleting**: Always check duplicate groups before deletion
2. **Keep originals**: Consider keeping the oldest or most recently accessed file
3. **Backup important data**: Always backup critical files before bulk operations
4. **Test with small directories**: Start with small directories to understand behavior
5. **Check Recycle Bin**: Verify files are in Recycle Bin after deletion

### Limitations
- **Read-only files**: Cannot delete read-only files (permission error shown)
- **Locked files**: Files in use by other applications cannot be deleted
- **Network drives**: May have different behavior than local drives
- **Symbolic links**: Currently follows links (may scan same file multiple times)

## Known Limitations

### Performance
- **Large directories**: Scanning directories with millions of files may be slow
- **Network drives**: Significantly slower than local drives
- **Duplicate detection**: Full hash computation is CPU-intensive

### File System
- **Long paths**: Windows 260-character path limit (enable long paths in Windows)
- **Permissions**: Requires read access to scan, write access to delete
- **Special files**: System files and hidden files are included in scans

### UI Limitations
- **No sorting**: File list is not sortable (displayed in scan order)
- **No filtering**: Cannot filter files by name, size, or date
- **No search**: Cannot search within file list
- **No preview**: Cannot preview file contents before deletion

### Export Limitations
- **CSV format**: Basic format with Group, Path, Size columns
- **JSON format**: Raw duplicate groups structure
- **No customization**: Export format cannot be customized

## Troubleshooting

### Scan Shows No Files
- **Check folder selection**: Ensure folder was selected before scanning
- **Check permissions**: Verify you have read access to the directory
- **Check ignore patterns**: Ensure files aren't excluded by ignore patterns

### Duplicate Detection Too Slow
- **Reduce hash size**: Lower partial hash KB in Settings
- **Add ignore patterns**: Exclude large directories (e.g., `node_modules`)
- **Close other applications**: Free up CPU resources

### Files Not Deleting
- **Check permissions**: Ensure write access to file location
- **Check file locks**: Close applications using the files
- **Check Recycle Bin**: Files may be read-only or on network drive

### Settings Not Saving
- **Check permissions**: Ensure write access to `%APPDATA%\diskviz\`
- **Check disk space**: Ensure sufficient disk space
- **Check log file**: Review `%APPDATA%\diskviz\diskviz.log` for errors

### Application Crashes
- **Check log file**: Review `%APPDATA%\diskviz\diskviz.log`
- **Delete config**: Remove `config.json` to reset to defaults
- **Reinstall**: Rebuild application from source

## Keyboard Shortcuts

*Note: Keyboard shortcuts are planned for future versions. Currently, all navigation is mouse-based.*

## Getting Help

1. **Check log files**: `%APPDATA%\diskviz\diskviz.log`
2. **Review settings**: Verify configuration in Settings screen
3. **Check permissions**: Ensure proper file system permissions
4. **Report issues**: Open an issue on GitHub with log file contents

## Tips & Tricks

1. **Batch operations**: Scan multiple directories by changing folder selection
2. **Export before delete**: Export duplicate list before deleting for records
3. **Incremental scanning**: Scan subdirectories separately for better organization
4. **Font scaling**: Increase font scale for better readability on high-DPI displays
5. **Theme switching**: Switch themes based on ambient lighting conditions

