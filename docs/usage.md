---
layout: default
title: Usage Guide
permalink: /usage/
---

# Usage Guide

This guide covers all the ways to use build-cleaner, from basic scanning to advanced filtering and script generation.

## Basic Usage

### Scan Current Directory

```bash
build-cleaner
```

This scans the current directory recursively for build artifacts across all supported ecosystems.

### Scan a Specific Directory

```bash
build-cleaner /path/to/projects
```

### Preview Mode (Dry Run)

See what would be deleted without actually deleting anything:

```bash
build-cleaner --dry-run
# or
build-cleaner -n
```

This is the recommended way to first explore what build-cleaner finds.

### Skip Confirmation

To skip the confirmation prompt and delete immediately:

```bash
build-cleaner --yes
# or
build-cleaner -y
```

**Use with caution!** Consider using `--dry-run` first.

## CLI Options Reference

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
  -p, --lang <TYPE>                Filter by project type
      --min-age <DAYS>             Only projects not modified in N days
      --min-size <SIZE>            Only artifacts above this size
  -v, --verbose                    Enable verbose output
  -u, --user-caches                Also scan user-level caches
      --cache-only                 Only scan user caches, skip projects
      --cache-category <CATEGORY>  Filter cache categories
      --script <PATH>              Generate shell script instead of deleting
  -h, --help                       Print help
  -V, --version                    Print version
```

## Filtering Options

### By Language/Ecosystem

Clean only specific ecosystems with `--lang` (can be used multiple times):

```bash
# Only Node.js projects
build-cleaner --lang node

# Only Rust and Python projects
build-cleaner --lang rust --lang python
```

Available ecosystem filters:
- `node` - Node.js (`node_modules`)
- `rust` - Rust (`target`)
- `python` - Python (`__pycache__`, `.venv`, etc.)
- `flutter` - Flutter (`build`, `.dart_tool`)
- `java-maven` - Java Maven (`target`)
- `java-gradle` - Java Gradle (`build`, `.gradle`)
- `cpp` - C/C++ (`build`, `cmake-build-*`)
- `dot-net` - .NET (`bin`, `obj`)
- `go` - Go

### By Age

Only clean projects not modified in the last N days:

```bash
# Projects untouched for 30+ days
build-cleaner --min-age 30

# Old projects from 90+ days ago
build-cleaner --min-age 90
```

This is useful for cleaning old projects while keeping active ones intact.

### By Size

Only show artifacts above a certain size:

```bash
# Only artifacts larger than 100MB
build-cleaner --min-size 100MB

# Only artifacts larger than 1GB
build-cleaner --min-size 1GB
```

Supported size formats: `KB`, `MB`, `GB`

### Exclude Patterns

Skip certain directories during scanning:

```bash
# Skip any directory named "important-project"
build-cleaner --exclude important-project

# Skip multiple patterns
build-cleaner --exclude archived --exclude legacy
```

## Interactive Mode

Select which items to clean interactively:

```bash
build-cleaner --interactive
# or
build-cleaner -i
```

This presents a multi-select interface where you can:
- Navigate with arrow keys
- Select/deselect with Space
- Confirm with Enter

## User Cache Cleaning

### Include User Caches

Add user-level caches to the scan:

```bash
build-cleaner --user-caches
# or
build-cleaner -u
```

This includes AI model caches, package manager caches, IDE caches, etc.

### Cache Only Mode

Skip project scanning and only look at user caches:

```bash
build-cleaner --cache-only
```

### Filter Cache Categories

Focus on specific cache categories:

```bash
# Only AI/ML caches (Hugging Face, Ollama, etc.)
build-cleaner --cache-only --cache-category ai-ml

# Only package manager caches
build-cleaner --cache-only --cache-category package

# Multiple categories
build-cleaner --cache-only --cache-category ai-ml --cache-category ide
```

Available categories:
- `ai-ml` - AI/ML models (Hugging Face, Ollama, PyTorch, LM Studio)
- `package` - Package managers (npm, Cargo, pip, etc.)
- `dev` - Development tools (Xcode DerivedData, etc.)
- `ide` - IDE caches (JetBrains, VS Code)
- `container` - Container tools (Docker)

## Script Generation

Generate a bash script instead of deleting directly:

```bash
build-cleaner --script cleanup.sh
```

This creates a reviewable script that you can:
1. Open and review the exact commands
2. Edit to remove items you want to keep
3. Run when ready

```bash
# Generate the script
build-cleaner --user-caches --script cleanup.sh

# Review it
cat cleanup.sh

# Make executable and run
chmod +x cleanup.sh
./cleanup.sh
```

See [Script Generation](../scripts/) for more details.

## Verbose Output

Get detailed information about what's being scanned:

```bash
build-cleaner --verbose
# or
build-cleaner -v
```

## Combining Options

Options can be combined for precise control:

```bash
# Dry run on old Node.js projects over 500MB
build-cleaner --dry-run --lang node --min-age 60 --min-size 500MB

# Interactive mode with user caches, only AI and package managers
build-cleaner -i --user-caches --cache-category ai-ml --cache-category package

# Generate script for everything, excluding certain directories
build-cleaner --user-caches --exclude important --script full-cleanup.sh
```

## Common Workflows

### Weekly Cleanup

```bash
# Preview what's accumulated
build-cleaner --dry-run --user-caches ~/work

# If it looks good, clean it
build-cleaner --yes --user-caches ~/work
```

### Before a Backup

```bash
# Clean old projects before backing up
build-cleaner --yes --min-age 30 ~/work
```

### Recovering Disk Space Urgently

```bash
# Find the biggest offenders
build-cleaner --dry-run --user-caches --min-size 1GB ~/

# Interactively select what to remove
build-cleaner -i --user-caches --min-size 500MB ~/
```

### Safe Approach with Script

```bash
# Generate a script
build-cleaner --user-caches --script ~/cleanup.sh

# Review and edit
vim ~/cleanup.sh

# Run when ready
~/cleanup.sh
```

---

[Back to Home](/) | [Previous: Installation](../installation/) | [Next: Ecosystems](../ecosystems/)
