# DiskViz — Disk Usage & Duplicate File Analyzer

**DiskViz** is a modern, cross-platform desktop utility built with **Rust** and the **Iced** GUI framework.  
It provides high-performance directory scanning, duplicate file detection, and safe file operations through a clean, responsive, non-blocking interface.

---

## ⚡ Key Features

### **Disk Analysis**
- Recursive directory scanning with metadata collection  
- Human-readable size formatting  
- Progress indicators for long operations  

### **Duplicate Detection**
- Multi-stage pipeline:   
  `size match → partial hash → full Blake3 hash`  
- Groups identical files for review  
- Safe deletion using system Trash/Recycle Bin  

### **User Interface**
- Three functional screens: **Overview**, **Duplicates**, **Settings**  
- Responsive sidebar navigation  
- Light/Dark themes  
- Adjustable font scaling (1.0×–1.5×)  
- Toast notifications and confirmation dialogs  

### **Data & Settings**
- Export duplicate results to **CSV** or **JSON**  
- Persistent settings stored locally  
- Custom ignore-glob patterns (e.g., `.git`, `node_modules`, `target`)  

---

## 🛠 Build & Run

### **Clone the Repository**
```bash
git clone <repository-url>
cd diskviz
Build (Release Mode)
bash
Copy code
cargo build --release
Run the Application
bash
Copy code
cargo run
Binary Output
Windows: target/release/diskviz.exe

macOS/Linux: target/release/diskviz

⚙ Configuration
DiskViz stores user preferences in a JSON configuration file.

Config File Locations
Platform	Path
Windows	%APPDATA%\SAN\diskviz\config.json
macOS	~/Library/Application Support/SAN/diskviz/config.json
Linux	~/.config/SAN/diskviz/config.json

Example Config
json
Copy code
{
  "theme_dark": true,
  "font_scale": 1.0,
  "ignore_globs": [".git", "node_modules", "target"],
  "partial_hash_kb": 256
}
📁 Project Structure
txt
Copy code
diskviz/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Application state & message handling
│   ├── core/            # Scanning, dedupe, export, config, logging
│   └── ui/              # Overview, Duplicates, Settings, widgets, styles
├── Cargo.toml
├── README.md
└── USER_GUIDE.md
👤 Author
San Win
Software Engineering Student — KMITL
