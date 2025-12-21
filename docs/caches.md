---
layout: default
title: User Caches
permalink: /caches/
---

# User Caches

Beyond project build artifacts, build-cleaner can scan and clean user-level caches that accumulate over time. These caches are often the biggest consumers of disk space, especially AI/ML model caches.

## Enabling User Cache Scanning

```bash
# Include user caches alongside project scanning
build-cleaner --user-caches

# Only scan user caches (skip projects)
build-cleaner --cache-only

# Filter to specific categories
build-cleaner --cache-only --cache-category ai-ml
```

## Cache Categories

### AI/ML Caches (`--cache-category ai-ml`)

AI and machine learning model caches are often the largest consumers of disk space.

| Cache | Location | Typical Size |
|-------|----------|--------------|
| Hugging Face | `~/.cache/huggingface/` | 10-100+ GB |
| Ollama | `~/.ollama/models/` | 5-50+ GB |
| PyTorch Hub | `~/.cache/torch/` | 1-20 GB |
| LM Studio | `~/.cache/lm-studio/` | 5-100+ GB |

**Warning:** Deleting model caches means models will need to be re-downloaded when next used.

**Example:**
```bash
# Preview AI/ML caches
build-cleaner --cache-only --cache-category ai-ml --dry-run

# Output:
# Hugging Face cache    28.4 GB
# Ollama models         12.1 GB
# PyTorch hub            2.3 GB
# Total: 42.8 GB
```

---

### Package Manager Caches (`--cache-category package`)

Package managers cache downloaded packages to speed up future installs.

| Cache | Location | Notes |
|-------|----------|-------|
| npm | `~/.npm/` | Node.js packages |
| Yarn | `~/.cache/yarn/` | Yarn v1 cache |
| Yarn Berry | `~/.yarn/berry/cache/` | Yarn v2+ cache |
| pnpm | `~/.local/share/pnpm/store/` | pnpm content-addressable store |
| Cargo | `~/.cargo/registry/`, `~/.cargo/git/` | Rust crates |
| pip | `~/.cache/pip/` | Python packages |
| Maven | `~/.m2/repository/` | Java Maven dependencies |
| Gradle | `~/.gradle/caches/` | Java Gradle dependencies |
| CocoaPods | `~/Library/Caches/CocoaPods/` | iOS dependencies (macOS) |
| RubyGems | `~/.gem/` | Ruby gems |
| Pub | `~/.pub-cache/` | Dart/Flutter packages |

**Official Cleanup Commands:**
Many package managers have official cleanup commands. Build-cleaner includes these as comments in generated scripts:

```bash
# npm
npm cache clean --force

# pip
pip cache purge

# Cargo
cargo cache -a  # requires cargo-cache

# Docker
docker system prune -a
```

---

### Development Tool Caches (`--cache-category dev`)

Development tools and SDKs often maintain large caches.

| Cache | Location | Notes |
|-------|----------|-------|
| Xcode DerivedData | `~/Library/Developer/Xcode/DerivedData/` | Build outputs (macOS) |
| Xcode Device Support | `~/Library/Developer/Xcode/iOS DeviceSupport/` | Device symbols (macOS) |
| Xcode Archives | `~/Library/Developer/Xcode/Archives/` | App archives (macOS) |
| CoreSimulator | `~/Library/Developer/CoreSimulator/` | iOS Simulator data |

**Note:** Xcode caches can easily reach 50+ GB on active iOS development machines.

**Warning for Xcode Archives:** Archives contain signed builds and may be needed for crash symbolication. Consider keeping recent archives.

---

### IDE Caches (`--cache-category ide`)

IDE caches speed up code analysis but can grow large.

| Cache | Location | Notes |
|-------|----------|-------|
| JetBrains | `~/Library/Caches/JetBrains/` | IntelliJ, WebStorm, etc. |
| VS Code Cache | `~/Library/Caches/com.microsoft.VSCode/` | macOS |
| VS Code Server | `~/.vscode-server/` | Remote development |

**Warning:** Cleaning IDE caches may slow down IDE startup and code analysis temporarily.

---

### Container Caches (`--cache-category container`)

Container tools can accumulate large amounts of cached data.

| Cache | Command | Notes |
|-------|---------|-------|
| Docker | `docker system prune -a` | Images, containers, volumes |

**Warning:** Docker cleanup removes ALL unused images, which may require re-pulling images.

**Special Handling:** Docker is handled via the `docker system prune -a` command rather than direct deletion, giving you more control and using Docker's official cleanup mechanism.

---

## Example: Full Cache Analysis

```bash
$ build-cleaner --cache-only --dry-run

User Caches Analysis:

AI/ML:
  Hugging Face cache           28.4 GB
  Ollama models                12.1 GB
  PyTorch hub                   2.3 GB

Package Managers:
  npm cache                     1.2 GB
  Cargo registry                3.4 GB
  Maven repository              2.1 GB
  Gradle caches                 4.5 GB
  pip cache                     0.8 GB

Development:
  Xcode DerivedData            15.2 GB
  Xcode Device Support          8.4 GB
  CoreSimulator                 6.1 GB

IDE:
  JetBrains cache               2.1 GB
  VS Code cache                 0.4 GB

Container:
  Docker (docker system prune)  5.8 GB

──────────────────────────────────────
Total user caches: 92.8 GB
```

## Filtering Cache Categories

Focus on specific categories:

```bash
# Only AI/ML models
build-cleaner --cache-only --cache-category ai-ml

# Package managers and IDE caches
build-cleaner --cache-only --cache-category package --cache-category ide

# Everything except containers
build-cleaner --cache-only --cache-category ai-ml --cache-category package --cache-category dev --cache-category ide
```

## Safety Considerations

### Caches That Require Apps to Be Closed

Some caches should only be deleted when their associated applications are closed:

- **JetBrains caches**: Close all JetBrains IDEs first
- **VS Code cache**: Close VS Code first
- **Xcode caches**: Close Xcode first

Build-cleaner shows warnings for these caches in interactive mode.

### Caches That Will Cause Re-downloads

Deleting these caches means content will need to be re-downloaded:

- **AI/ML models**: Large models (10+ GB each) will need re-downloading
- **Package manager caches**: Packages re-downloaded on next install
- **Docker images**: Images re-pulled on next run

### Recommended Workflow

1. **Preview first:**
   ```bash
   build-cleaner --cache-only --dry-run
   ```

2. **Generate a script:**
   ```bash
   build-cleaner --cache-only --script cleanup.sh
   ```

3. **Review and edit the script** to remove caches you want to keep

4. **Run the script:**
   ```bash
   chmod +x cleanup.sh
   ./cleanup.sh
   ```

---

[Back to Home](/) | [Previous: Ecosystems](../ecosystems/) | [Next: Script Generation](../scripts/)
