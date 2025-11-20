---

# **DiskViz — Disk Usage Visualizer**

### *A Modern Cross-Platform Rust Desktop Application for Disk Analysis & Duplicate Detection*

![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange)
![Iced](https://img.shields.io/badge/GUI-Iced-blue)
![Status](https://img.shields.io/badge/Project-Active-brightgreen)
![License](https://img.shields.io/badge/License-Academic_Use-lightgrey)

---

## **Project Summary**

**DiskViz** is a modern, cross-platform desktop application for analyzing disk usage, detecting duplicate files, and managing file cleanup.
Built in **Rust** using the **Iced GUI framework**, it provides fast scanning performance, safe deletion mechanisms, persistent configuration, and a clean multi-screen interface.

This project implements robust system-level logic, multi-stage hashing, theme customization, and a responsive UI architecture suitable for academic submission and professional portfolios.

---

## **Feature Overview**

### **Core Functionality**

* **Directory Scanning**: Recursive file system traversal with metadata collection
* **Duplicate Detection**: Three-stage algorithm (size → partial hash → full hash) using Blake3
* **Safe Deletion**: Files moved to Recycle Bin/Trash (platform-specific)
* **Export Results**: CSV and JSON export with user-selected paths
* **Settings Persistence**: JSON configuration stored in system application data directories

---

### **User Interface**

* **Three-Screen Navigation**: Overview, Duplicates, and Settings
* **Light/Dark Theme**: Real-time theme switching
* **Font Scaling**: Adjustable UI text size (1.0× to 1.5×)
* **Toast Notifications**: Non-intrusive on-screen alerts
* **Confirmation Dialogs**: Additional safety for delete actions

---

### **Advanced Features**

* Configurable partial hash size
* Ignore-globs for directory exclusion
* Non-blocking operations with async runtime
* Comprehensive daily-rolling logging system

---

## **Application Screens**

### **Overview Screen**

The primary interface for scanning directories:

* Choose Folder
* Start Scan
* File listing with path and size
* Status and progress indicators

![Overview Screen](assets/Overview.png)

---

### **Duplicates Screen**

Tools for identifying and managing duplicate files:

* Find Duplicates
* Delete Selected (with confirmation)
* Export CSV / Export JSON
* Grouped duplicate sets with checkboxes

![Duplicates Screen](assets/Duplicate.png)

---

### **Settings Screen**

Manage application preferences:

* Theme toggle
* Font scaling
* Ignore glob patterns
* Partial hash size
* Save / Reload settings

![Settings Screen](assets/Setting.png)

---

## **Configuration**

### **Settings File Location**

* **Windows**: `%APPDATA%\SAN\diskviz\config.json`
* **macOS**: `~/Library/Application Support/SAN/diskviz/config.json`
* **Linux**: `~/.config/SAN/diskviz/config.json`

---

### **Log Files**

* **Windows**: `%LOCALAPPDATA%\SAN\diskviz\diskviz.log`
* **macOS**: `~/Library/Application Support/SAN/diskviz/diskviz.log`
* **Linux**: `~/.local/share/SAN/diskviz/diskviz.log`

---

### **Settings Format**

```json
{
  "theme_dark": true,
  "font_scale": 1.0,
  "ignore_globs": ["node_modules", "target"],
  "partial_hash_kb": 256
}
```

---

## **Export Formats**

### **CSV Export**

Columns:

* Group
* Path
* Size (bytes)

### **JSON Export**

```json
[
  [
    {
      "path": "/path/to/file1.txt",
      "size": 1024,
      "modified": 1234567890,
      "is_dir": false
    },
    {
      "path": "/path/to/file2.txt",
      "size": 1024,
      "modified": 1234567890,
      "is_dir": false
    }
  ]
]
```

---

## **Safety Features**

### **Delete Confirmation**

* Explicit confirmation required
* Displays file count
* Files moved to Recycle Bin/Trash
* Restorable after deletion

### **Best Practices**

1. Review duplicate groups before deletion
2. Keep the most important or oldest file
3. Backup sensitive directories
4. Test scanning on smaller folders first

---

## **Project Structure**

```
diskviz/
├── src/
│   ├── main.rs          
│   ├── app.rs           
│   ├── core/            
│   │   ├── scan.rs      
│   │   ├── dedupe.rs    
│   │   ├── export.rs    
│   │   ├── trashcan.rs  
│   │   ├── config.rs    
│   │   ├── logging.rs   
│   │   └── types.rs     
│   └── ui/              
│       ├── overview.rs  
│       ├── duplicates.rs
│       ├── settings.rs  
│       ├── widgets.rs   
│       └── styles.rs    
├── Cargo.toml           
├── README.md            
└── USER_GUIDE.md        
```

---

## **Dependencies**

* iced 0.12
* tokio 1.39
* blake3 1.5.4
* rayon 1.10.0
* ignore 0.4.22
* trash 5.0.0
* rfd 0.14
* humansize 2.1.3
* serde 1.0
* directories 5.0.1

---

## **Author**

**San Win**
Software Engineering Student — KMITL, Thailand

---
