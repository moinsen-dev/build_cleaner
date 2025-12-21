---
layout: default
title: Web UI Dashboard
permalink: /web-ui/
---

# Web UI Dashboard

Build-cleaner includes a built-in web dashboard that provides an interactive, visual interface for scanning and cleaning build artifacts.

## Starting the Web UI

```bash
# Launch web dashboard (auto-opens browser)
build-cleaner --serve

# Use a custom port
build-cleaner --serve --port 3000

# Don't auto-open browser
build-cleaner --serve --no-open
```

By default, the web server starts on `http://127.0.0.1:8080` and automatically opens your default browser.

## Features

### Interactive Scan Controls

The dashboard provides a user-friendly interface for configuring scans:

- **Path Input**: Enter the directory path to scan
- **User Caches Toggle**: Include user-level caches (AI models, package managers, IDEs)
- **Cache Only Mode**: Skip project scanning, only scan user caches
- **Language Filter**: Filter by specific ecosystems (Node, Rust, Python, etc.)
- **Cache Category Filter**: Filter by cache type (AI/ML, Package, Dev, IDE, Container)
- **Minimum Size**: Only show items above a certain size threshold

### Real-time Progress

The web UI uses Server-Sent Events (SSE) for live progress updates:

- **Live folder scanning**: See which directory is currently being scanned
- **Discovery notifications**: Get instant feedback when projects are found (e.g., "Found Node.js project: my-app (245 MB)")
- **Running totals**: Watch the project count and total size increase as scanning progresses
- **Completion notification**: Clear indication when the scan finishes

### Chart Visualizations

The dashboard includes interactive Chart.js-powered visualizations:

1. **By Ecosystem (Donut Chart)**
   - Breakdown of project artifact sizes by ecosystem
   - **Click any segment** to scroll to and highlight projects of that ecosystem
   - Includes Node.js, Rust, Python, Flutter, Java, C++, .NET, Go

2. **Top Projects by Size (Bar Chart)**
   - Shows the 10 largest projects found
   - **Click any bar** to open the project detail modal
   - Quickly identify which projects are consuming the most space

3. **By Cache Category (Bar Chart)**
   - Breakdown of user cache sizes by category
   - Only shown when user caches are included in the scan
   - Includes AI/ML, Package Managers, Development, IDE, Container

### Top 10 Largest Items

A ranked list of the largest artifacts and caches found, with:
- Medal icons for top 3 (gold, silver, bronze)
- Type badges showing the ecosystem or cache type
- Inline size display

### Project Details Modal

Click on any project row or chart element to open a detailed view:

- **Project information**: Name, ecosystem type, full path
- **Artifacts list**: Exact directories that will be deleted with individual sizes
- **Quick action**: Select or deselect the project for cleanup directly from the modal

### Deletion Preview

Before any cleanup action, a **Deletion Preview** panel appears showing:

- **Complete file list**: Every directory that will be permanently deleted
- **Grouped by project/cache**: Easy to see what belongs to which item
- **Size information**: Individual and total sizes to be reclaimed
- **Visual warning**: Red-styled panel makes it clear this is destructive

This ensures you always know exactly what will be deleted before confirming.

### Item Selection

Both projects and caches are listed with checkboxes:

- **Projects**: Click the row to see details, or use the checkbox to select for cleanup
- **Caches**: Shows cache icon, label, description, size, and any warnings

Use the "Select All" and "Deselect All" buttons to quickly manage selections.

### Actions

- **Clean Selected**: Delete selected items directly (with confirmation dialog)
- **Generate Script**: Create a bash script for selected items (opens in modal for copy/save)

## API Endpoints

The web UI communicates with a REST API:

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/status` | Get current scanning state |
| GET | `/api/scan?path=...` | Trigger a new scan |
| GET | `/api/progress` | SSE stream for real-time scan progress |
| GET | `/api/results` | Get scan results |
| POST | `/api/clean` | Execute cleanup for selected items |
| POST | `/api/generate-script` | Generate cleanup script |

### SSE Progress Events

The `/api/progress` endpoint streams JSON events during scanning:

```json
{
  "current_path": "/path/being/scanned",
  "projects_found": 5,
  "caches_found": 2,
  "total_size": 1073741824,
  "complete": false,
  "message": "Found Node.js project: my-app (245 MB)"
}
```

### Query Parameters for /api/scan

- `path` - Directory path to scan (required)
- `user_caches` - Include user caches (true/false)
- `cache_only` - Only scan caches (true/false)
- `lang` - Filter by ecosystem (can be repeated)
- `cache_category` - Filter by cache category (can be repeated)
- `min_size` - Minimum size filter (e.g., "100MB")
- `min_age` - Minimum age in days

### Request Body for /api/clean

```json
{
  "project_ids": [0, 2, 5],
  "cache_ids": [1, 3]
}
```

IDs correspond to the index in the scan results arrays.

## Theming

The web UI automatically adapts to your system's color scheme preference:

- **Dark Mode**: Dark background with light text (default)
- **Light Mode**: Light background with dark text

The theme follows the `prefers-color-scheme` CSS media query.

## Security Notes

- The server only binds to `127.0.0.1` (localhost) by default
- No authentication is required (local use only)
- File deletion operations require explicit user action
- All paths are validated before deletion

## Use Cases

### Quick Visual Overview

```bash
build-cleaner --serve
```

Great for getting a visual understanding of what's consuming disk space.

### Remote Headless Server

```bash
build-cleaner --serve --port 8080 --no-open
```

Start the server without opening a browser, useful for SSH sessions.

### Custom Port

```bash
build-cleaner --serve --port 3000
```

Use a custom port if 8080 is already in use.

---

[Back to Home](/) | [Previous: Safety](../safety/)
