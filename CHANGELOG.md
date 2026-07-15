# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-07-15

### Added

- Cleanup priority sorting for interactive project selection (`-i`): projects
  that are both large and long untouched are ranked to the top of the list
  in both the terminal (`dialoguer` multi-select) and the web UI
- "Last touched X days ago" display alongside each project's size in
  interactive selection lists

## [0.3.0] - 2026-04-15

### Added

- Unit tests for scanner, cleaner, and cache_scanner modules

### Changed

- Updated repository URL to moinsen-dev org
- Improved documentation with usage examples

## [0.2.0] - 2025-12-21

### Added

- **Web UI Dashboard** (`--serve`)
  - Built-in web interface for interactive scanning and cleanup
  - Auto-opens browser by default (disable with `--no-open`)
  - Configurable port with `--port` (default: 8080)

- **Chart.js Visualizations**
  - Donut chart: Size breakdown by ecosystem (Node, Rust, Python, etc.) - clickable
  - Bar chart: Top projects by size - clickable to view details
  - Bar chart: Size breakdown by cache category (AI/ML, Package, IDE, etc.)
  - Top 10 largest items display with progress bars

- **Interactive Web Features**
  - Real-time scan progress via Server-Sent Events (SSE)
  - Live display of folders being scanned and discoveries as they happen
  - Project detail modal: Click any project to see exactly which artifact folders will be deleted
  - Deletion preview panel: Before cleanup, see complete list of all directories to be removed
  - Checkbox selection for projects and caches
  - Execute cleanup directly from browser
  - Generate cleanup scripts from selection
  - Dark/light theme support (follows system preference)
  - Responsive design for mobile devices

### Technical

- Added axum web framework for HTTP server
- Server-Sent Events (SSE) endpoint for real-time progress streaming
- Embedded static assets via rust-embed (single binary distribution)
- Async tokio runtime with broadcast channels for concurrent operations
- RESTful API endpoints for scan, results, cleanup, and script generation

## [0.1.0] - 2024-12-18

### Added

- Initial release of build-cleaner
- **Multi-ecosystem scanning**: Support for 9 programming ecosystems
  - Node.js (`node_modules/`)
  - Rust (`target/`)
  - Python (`__pycache__/`, `.venv/`, `venv/`, `.tox/`, `*.egg-info/`)
  - Flutter (`build/`, `.dart_tool/`)
  - Java Maven (`target/`)
  - Java Gradle (`build/`, `.gradle/`)
  - C/C++ (`build/`, `cmake-build-*/`)
  - .NET (`bin/`, `obj/`)
  - Go (via `go clean -cache`)

- **User cache cleaning** (`--user-caches`, `--cache-only`)
  - 25+ cache types organized into 5 categories:
  - AI/ML: Hugging Face, Ollama, PyTorch hub, LM Studio
  - Package managers: npm, Yarn, pnpm, Cargo, Gradle, Maven, pip, CocoaPods, RubyGems, Pub
  - Development: Xcode DerivedData, Device Support, Archives
  - IDE: JetBrains cache, VSCode cache, VSCode Server
  - Container: Docker (with `docker system prune -a` command)

- **Script generation** (`--script <PATH>`)
  - Generate reviewable bash scripts instead of direct deletion
  - Grouped by category with size totals
  - Includes official cleanup commands as comments

- **Interactive mode** (`--interactive`)
  - Multi-select interface to choose which items to clean
  - Shows warnings for caches requiring apps to be closed

- **Filtering options**
  - `--lang <TYPE>`: Filter by project type
  - `--cache-category <CATEGORY>`: Filter cache categories
  - `--min-size <SIZE>`: Only artifacts above threshold
  - `--min-age <DAYS>`: Only projects not modified in N days

- **Dry-run mode** (`--dry-run`)
  - Preview what would be deleted without making changes

- **Comprehensive summary output**
  - Breakdown by ecosystem and category
  - Top 10 largest items with medal icons
  - Total space to reclaim

- **Safety features**
  - Two-factor detection (config file + artifact directory)
  - Confirmation prompts before deletion
  - Official cleanup commands as alternatives

### Technical

- Built with Rust for performance and safety
- Parallel filesystem scanning with Rayon
- Cross-platform support (macOS, Linux)
- Homebrew formula for easy installation
