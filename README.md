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

```
$ build-cleaner --user-caches --dry-run ~/work

🔍 Preview of cleanable artifacts (dry-run mode)

   1. 📦 my-app (Node.js) → node_modules (245.3 MB)
   2. 🦀 my-cli (Rust) → target (1.2 GB)
   3. 🦋 my-flutter-app (Flutter) → build, .dart_tool (156.8 MB)
   ...

────────────────────────────────────────────────────────────
📊 Summary
────────────────────────────────────────────────────────────
  • 42 projects with 78 artifact directories

  📂 By ecosystem:
     🦋  15 Flutter         12.4 GB
     📦  20 Node.js          8.2 GB
     🦀   4 Rust             6.1 GB
     🐍   3 Python           0.3 GB

  • 12 user caches

  🗄️ User caches:
     🤖   3 AI/ML           32.1 GB
     📦   6 Package managers 8.4 GB
     💻   2 IDE              1.2 GB
     🐳   1 Container        5.8 GB

  🏆 Largest items:
   🥇  1. 🤗 Hugging Face                    28.4 GB
   🥈  2. 🦙 Ollama models                   12.1 GB
   🥉  3. 🦀 my-big-project/target            4.2 GB
   ...

────────────────────────────────────────────────────────────
  💾 Total space to reclaim: 74.5 GB
────────────────────────────────────────────────────────────
```

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
