# build-cleaner

A fast, safe CLI tool to recursively find and delete build artifacts and caches across multiple programming ecosystems. Reclaim hundreds of gigabytes of disk space with a single command.

## Features

- **Multi-ecosystem support**: Node.js, Rust, Python, Flutter, Java (Maven/Gradle), C/C++, .NET, Go
- **User cache cleaning**: AI/ML models (Hugging Face, Ollama), package managers, IDE caches, Docker
- **Safe by default**: Two-factor detection (config file + artifact directory) prevents accidental deletion
- **Dry-run mode**: Preview what would be deleted without making changes
- **Interactive mode**: Select which projects/caches to clean
- **Script generation**: Generate a reviewable shell script instead of direct deletion
- **Comprehensive summary**: See breakdown by ecosystem, category, and largest items
- **Web UI dashboard**: Built-in web interface with Chart.js visualizations

## Installation

### Homebrew (macOS/Linux)

```bash
brew tap moinsen-dev/tap
brew install build-cleaner
```

### Cargo

```bash
cargo install build-cleaner
```

### From Source

```bash
git clone https://github.com/moinsen-dev/build_cleaner
cd build_cleaner
cargo build --release
# Binary is at target/release/build-cleaner
```

### Using Makefile

```bash
git clone https://github.com/moinsen-dev/build_cleaner
cd build_cleaner

# Install to ~/bin (no sudo required)
make install-local

# Or install to /usr/local/bin (requires sudo)
make install

# See all available targets
make help
```

## Quick Start

**Reclaim disk space in 3 commands:**

```bash
# 1. Preview everything that can be cleaned in your projects folder (safe — nothing is deleted)
build-cleaner --dry-run ~/projects

# 2. Also include user-level caches (AI models, npm/cargo/pip caches, IDE caches)
build-cleaner --dry-run --user-caches ~/projects

# 3. When ready, delete everything (prompts for confirmation)
build-cleaner ~/projects
```

**Other common workflows:**

```bash
# Clean only stale Node.js and Rust projects (not touched in 30+ days)
build-cleaner --lang node --lang rust --min-age 30 ~/projects

# Generate a shell script to review before running
build-cleaner --script cleanup.sh ~/projects && cat cleanup.sh

# Open the web dashboard for a visual overview
build-cleaner --serve ~/projects
```

## Usage

### Basic Usage

```bash
# Scan current directory for build artifacts
build-cleaner

# Scan a specific directory
build-cleaner /path/to/projects

# Preview what would be deleted (dry-run)
build-cleaner --dry-run

# Skip confirmation prompts
build-cleaner --yes
```

### User Cache Cleaning

```bash
# Scan projects AND user caches (AI models, package managers, IDEs)
build-cleaner --user-caches

# Only scan user caches, skip project build artifacts
build-cleaner --cache-only

# Filter by cache category
build-cleaner --cache-only --cache-category ai-ml
build-cleaner --cache-only --cache-category package
build-cleaner --cache-only --cache-category ai-ml --cache-category ide
```

### Script Generation

```bash
# Generate a shell script instead of deleting directly
build-cleaner --script cleanup.sh

# Review and edit the script, then run it
chmod +x cleanup.sh
./cleanup.sh
```

### Web UI Dashboard

```bash
# Launch web dashboard (auto-opens browser)
build-cleaner --serve

# Use a custom port
build-cleaner --serve --port 3000

# Don't auto-open browser
build-cleaner --serve --no-open
```

The web UI provides:
- **Real-time scan progress**: See which folders are being scanned and discoveries as they happen
- **Interactive charts**: Clickable donut/bar charts showing size by ecosystem and top projects
- **Project details**: Click any project to see exactly which artifact folders will be deleted
- **Deletion preview**: Before cleanup, see a complete list of all directories to be removed
- **Checkbox selection**: Select individual projects/caches or use "Select All"
- **Script generation**: Generate a reviewable shell script from the UI

### Filtering Options

```bash
# Only scan Node.js and Rust projects
build-cleaner --lang node --lang rust

# Only projects not modified in 30+ days
build-cleaner --min-age 30

# Only artifacts larger than 100MB
build-cleaner --min-size 100MB

# Interactive selection mode
build-cleaner --interactive

# Verbose output
build-cleaner --verbose
```

## Supported Ecosystems

| Ecosystem | Config File | Artifacts Cleaned |
|-----------|-------------|-------------------|
| Node.js | `package.json` | `node_modules/` |
| Rust | `Cargo.toml` | `target/` |
| Python | `setup.py`, `pyproject.toml`, `requirements.txt` | `__pycache__/`, `.venv/`, `venv/`, `.tox/`, `*.egg-info/` |
| Flutter | `pubspec.yaml` | `build/`, `.dart_tool/` |
| Java (Maven) | `pom.xml` | `target/` |
| Java (Gradle) | `build.gradle`, `build.gradle.kts` | `build/`, `.gradle/` |
| C/C++ | `CMakeLists.txt`, `Makefile` | `build/`, `cmake-build-*/` |
| .NET | `*.csproj`, `*.fsproj` | `bin/`, `obj/` |
| Go | `go.mod` | (uses `go clean -cache`) |

## User Cache Categories

| Category | Icon | Examples |
|----------|------|----------|
| AI/ML | `--cache-category ai-ml` | Hugging Face models, Ollama, PyTorch hub, LM Studio |
| Package Managers | `--cache-category package` | npm, Yarn, pnpm, Cargo, Gradle, Maven, pip, CocoaPods, RubyGems |
| Development | `--cache-category dev` | Xcode DerivedData, Device Support, Archives |
| IDE | `--cache-category ide` | JetBrains cache, VSCode cache, VSCode Server |
| Container | `--cache-category container` | Docker (uses `docker system prune -a`) |

## Example Output

Real output from `build-cleaner --dry-run ~/projects` on a typical development machine:

```
$ build-cleaner --dry-run ~/projects

🔍 Preview of cleanable artifacts: (dry-run mode - nothing will be deleted)

   1. 🦀 my-api (Rust) → target (44.9 GB)
   2. 🦀 my-cli (Rust) → target (4.1 GB)
   3. 🦋 my-app (Flutter) → build, .dart_tool, Pods (3.8 GB)
   4. 📦 frontend (Node.js) → node_modules, .next (3.1 GB)
   5. 🐍 ml (Python) → .pytest_cache, .venv, __pycache__ (2.4 GB)
   6. 📦 no_thanks (Node.js) → node_modules (2.2 GB)
   7. 📦 dashboard (Node.js) → node_modules, .next (1.9 GB)
   8. 🦋 example (Flutter) → build, .dart_tool (1.5 GB)
   9. 📦 question-forge (Node.js) → node_modules, .next (1.5 GB)
  10. 🦋 mobile-app (Flutter) → build, .dart_tool, Pods (1.5 GB)
  ...
 120. 🦋 tiny_package (Flutter) → .dart_tool (37 B)

────────────────────────────────────────────────────────────
📊 Summary
────────────────────────────────────────────────────────────
  • 120 projects with 350 artifact directories

  📂 By ecosystem:
     🦀   9 Rust           102.0 GB
     📦  60 Node.js         64.8 GB
     🦋  30 Flutter         31.0 GB
     🐍  20 Python          16.7 GB
     ☕   1 Gradle         133.6 MB

  🏆 Largest projects:
   🥇  1. 🦀 my-api                              44.9 GB
   🥈  2. 🦀 my-cli                               4.1 GB
   🥉  3. 🦋 my-app                               3.8 GB
       4. 🦀 local_secrets_orchestrator           3.6 GB
       5. 📦 frontend                             3.1 GB
       6. 🐍 ml                                   2.4 GB
     → Top 6 = 62.9 GB (29% of total)

────────────────────────────────────────────────────────────
  💾 Total space to reclaim: 214.6 GB
────────────────────────────────────────────────────────────
```

## CLI Reference

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `[DIRECTORY]` | — | `.` | Directory to scan for projects |
| `--dry-run` | `-n` | off | Preview what would be deleted without deleting |
| `--yes` | `-y` | off | Skip confirmation prompts |
| `--interactive` | `-i` | off | Interactively choose which projects to clean |
| `--include <PATTERN>` | — | — | Additional directory patterns to clean (repeatable) |
| `--exclude <PATTERN>` | — | — | Directory patterns to skip (repeatable) |
| `--lang <TYPE>` | `-p` | all | Filter by ecosystem: `node`, `rust`, `python`, `flutter`, `java-maven`, `java-gradle`, `cpp`, `dot-net`, `go` (repeatable) |
| `--min-age <DAYS>` | — | — | Only include artifacts not modified in the last N days |
| `--min-size <SIZE>` | — | — | Only include artifacts above this size (e.g. `100MB`, `1GB`) |
| `--verbose` | `-v` | off | Enable verbose output |
| `--user-caches` | `-u` | off | Also scan user-level caches (AI models, package managers, IDEs) |
| `--cache-only` | — | off | Only scan user caches, skip project build artifacts |
| `--cache-category <CAT>` | — | all | Filter cache categories: `ai-ml`, `package`, `dev`, `ide`, `container` (repeatable) |
| `--script <PATH>` | — | — | Generate a shell script instead of deleting directly |
| `--serve` | — | off | Launch the web UI dashboard |
| `--port <PORT>` | — | `8080` | Port for the web UI server |
| `--no-open` | — | off | Do not auto-open the browser when starting the web UI |
| `--help` | `-h` | — | Print help |
| `--version` | `-V` | — | Print version |

## CLI Options

```
Usage: build-cleaner [OPTIONS] [DIRECTORY]

Arguments:
  [DIRECTORY]  Directory to scan for projects [default: .]

Options:
  -n, --dry-run                    Preview mode - show what would be deleted
  -y, --yes                        Skip confirmation prompts
  -i, --interactive                Interactive selection mode
      --include <PATTERN>          Additional directory patterns to clean
      --exclude <PATTERN>          Directory patterns to skip
  -p, --lang <TYPE>                Filter by project type (node, rust, python, flutter, java-maven, java-gradle, cpp, dot-net, go)
      --min-age <DAYS>             Only projects not modified in N days
      --min-size <SIZE>            Only artifacts above this size (e.g., "100MB", "1GB")
  -v, --verbose                    Enable verbose output
  -u, --user-caches                Also scan user-level caches
      --cache-only                 Only scan user caches, skip projects
      --cache-category <CATEGORY>  Filter cache categories (ai-ml, package, dev, ide, container)
      --script <PATH>              Generate shell script instead of deleting
      --serve                      Launch web UI dashboard
      --port <PORT>                Port for web UI server [default: 8080]
      --no-open                    Don't auto-open browser when starting web UI
  -h, --help                       Print help
  -V, --version                    Print version
```

## Safety Features

1. **Two-factor detection**: A directory is only considered a build artifact if BOTH:
   - A project configuration file exists (e.g., `package.json`, `Cargo.toml`)
   - The artifact directory exists (e.g., `node_modules/`, `target/`)

2. **Dry-run by default awareness**: The tool shows a clear summary before any deletion

3. **Confirmation prompts**: Unless `--yes` is specified, you must confirm before deletion

4. **Script generation**: Use `--script` to generate a reviewable script instead of direct deletion

5. **Official cleanup commands**: For some caches (npm, pip, Docker), the generated script includes official cleanup commands as comments

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
