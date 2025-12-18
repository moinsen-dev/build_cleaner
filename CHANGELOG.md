# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
