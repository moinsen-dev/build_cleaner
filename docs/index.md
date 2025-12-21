---
layout: default
title: Build Cleaner
---

# Build Cleaner

A fast, safe CLI tool to recursively find and delete build artifacts and caches across multiple programming ecosystems. **Reclaim hundreds of gigabytes of disk space with a single command.**

## The Problem

As developers, we accumulate massive amounts of build artifacts and cached data:

- `node_modules` directories scattered across dozens of projects
- Rust `target` folders consuming gigabytes each
- AI/ML model caches (Hugging Face, Ollama) taking 50GB+
- IDE caches, package manager caches, Xcode derived data...

Manually tracking and cleaning these is tedious and error-prone.

## The Solution

Build Cleaner scans your projects and caches, shows you exactly what's consuming space, and safely removes what you choose.

```bash
$ build-cleaner --user-caches ~/work

Total space to reclaim: 74.5 GB
```

## Quick Start

```bash
# Install via Homebrew
brew tap moinsen-dev/tap
brew install build-cleaner

# Scan current directory
build-cleaner

# Preview what would be deleted
build-cleaner --dry-run

# Include user caches (AI models, package managers, IDEs)
build-cleaner --user-caches

# Generate a script to review before running
build-cleaner --script cleanup.sh
```

## Key Features

### Multi-Ecosystem Support

Automatically detects and cleans build artifacts for **9 programming ecosystems**:

| Ecosystem | Artifacts Cleaned |
|-----------|-------------------|
| Node.js | `node_modules/` |
| Rust | `target/` |
| Python | `__pycache__/`, `.venv/`, `venv/`, `.tox/`, `*.egg-info/` |
| Flutter | `build/`, `.dart_tool/` |
| Java (Maven) | `target/` |
| Java (Gradle) | `build/`, `.gradle/` |
| C/C++ | `build/`, `cmake-build-*/` |
| .NET | `bin/`, `obj/` |
| Go | (uses `go clean -cache`) |

### User Cache Cleaning

Clean system-wide developer caches with `--user-caches`:

- **AI/ML Models**: Hugging Face, Ollama, PyTorch, LM Studio
- **Package Managers**: npm, Yarn, pnpm, Cargo, pip, CocoaPods, Maven, Gradle
- **Development**: Xcode DerivedData, Device Support, Archives
- **IDEs**: JetBrains, VS Code, VS Code Server
- **Containers**: Docker (with system prune)

### Safety First

- **Two-factor detection**: Only cleans directories that have BOTH a config file and artifact directory
- **Dry-run mode**: Preview everything before deletion
- **Interactive selection**: Choose exactly what to clean
- **Script generation**: Generate a reviewable bash script instead of direct deletion

## Example Output

```
$ build-cleaner --user-caches --dry-run ~/work

Preview of cleanable artifacts (dry-run mode)

   1. my-app (Node.js) -> node_modules (245.3 MB)
   2. my-cli (Rust) -> target (1.2 GB)
   3. my-flutter-app (Flutter) -> build, .dart_tool (156.8 MB)
   ...

Summary
  * 42 projects with 78 artifact directories

  By ecosystem:
     15 Flutter         12.4 GB
     20 Node.js          8.2 GB
      4 Rust             6.1 GB
      3 Python           0.3 GB

  * 12 user caches

  User caches:
      3 AI/ML           32.1 GB
      6 Package managers 8.4 GB
      2 IDE              1.2 GB
      1 Container        5.8 GB

  Largest items:
   1. Hugging Face                    28.4 GB
   2. Ollama models                   12.1 GB
   3. my-big-project/target            4.2 GB

  Total space to reclaim: 74.5 GB
```

## Documentation

- [Installation](installation/) - Install via Homebrew, Cargo, or from source
- [Usage Guide](usage/) - Complete CLI reference and examples
- [Supported Ecosystems](ecosystems/) - Details on each programming language support
- [User Caches](caches/) - Understanding and cleaning user-level caches
- [Script Generation](scripts/) - Generate reviewable cleanup scripts
- [Safety Features](safety/) - How build-cleaner keeps your data safe

## License

MIT License - see [LICENSE](https://github.com/moinsen-dev/build_cleaner/blob/main/LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request on [GitHub](https://github.com/moinsen-dev/build_cleaner).
