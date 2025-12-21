---
layout: default
title: Script Generation
permalink: /scripts/
---

# Script Generation

Build Cleaner can generate a reviewable bash script instead of deleting files directly. This gives you full control over what gets deleted and allows for manual review before execution.

## Generating a Script

```bash
build-cleaner --script cleanup.sh
```

This creates a bash script at the specified path instead of performing any deletions.

## Script Format

The generated script is organized and human-readable:

```bash
#!/bin/bash
# Build Cleaner - Generated Cleanup Script
# Generated: 2024-12-18 15:30:00
#
# Review this script before running!
# Total space to reclaim: 74.5 GB

set -e  # Exit on first error

# ════════════════════════════════════════
# Node.js Projects (8.2 GB)
# ════════════════════════════════════════

# my-react-app (245.3 MB)
rm -rf "/Users/me/work/my-react-app/node_modules"

# my-next-app (312.1 MB)
rm -rf "/Users/me/work/my-next-app/node_modules"

# ════════════════════════════════════════
# Rust Projects (6.1 GB)
# ════════════════════════════════════════

# my-cli (1.2 GB)
rm -rf "/Users/me/work/my-cli/target"

# ════════════════════════════════════════
# AI/ML Caches (32.1 GB)
# ════════════════════════════════════════

# Hugging Face cache (28.4 GB)
rm -rf "/Users/me/.cache/huggingface"

# Ollama models (12.1 GB)
rm -rf "/Users/me/.ollama/models"

# ════════════════════════════════════════
# Package Manager Caches (8.4 GB)
# ════════════════════════════════════════

# npm cache (1.2 GB)
rm -rf "/Users/me/.npm"
# Official cleanup: npm cache clean --force

# pip cache (0.8 GB)
rm -rf "/Users/me/.cache/pip"
# Official cleanup: pip cache purge

# ════════════════════════════════════════
# Container (Docker) (5.8 GB)
# ════════════════════════════════════════

# Docker system prune
# Note: This removes ALL unused images, containers, and volumes
docker system prune -a -f

echo "Cleanup complete!"
```

## Script Features

### 1. Organization by Category

Items are grouped by ecosystem and category with clear headers and subtotals, making it easy to understand what's being cleaned.

### 2. Size Information

Each item shows its size, helping you make informed decisions about what to keep.

### 3. Official Cleanup Commands

For package managers and tools that have official cleanup commands, the script includes them as comments:

```bash
# npm cache (1.2 GB)
rm -rf "/Users/me/.npm"
# Official cleanup: npm cache clean --force
```

This lets you choose between direct deletion or using the official command.

### 4. Safe Defaults

- Scripts use `set -e` to stop on first error
- Paths are quoted to handle spaces
- Docker uses `-f` flag for non-interactive execution

## Workflow

### 1. Generate the Script

```bash
build-cleaner --user-caches --script cleanup.sh
```

### 2. Review the Script

Open and review the script before running:

```bash
# View the script
cat cleanup.sh

# Or open in your editor
code cleanup.sh
vim cleanup.sh
```

### 3. Edit as Needed

Remove or comment out lines for items you want to keep:

```bash
# Keep this one - active project
# rm -rf "/Users/me/work/active-project/node_modules"

# Keep Ollama models - took forever to download
# rm -rf "/Users/me/.ollama/models"
```

### 4. Run the Script

Make it executable and run:

```bash
chmod +x cleanup.sh
./cleanup.sh
```

## Combining with Other Options

Script generation works with all other options:

```bash
# Script for old projects only
build-cleaner --min-age 60 --script old-projects.sh

# Script for large items only
build-cleaner --min-size 1GB --script big-items.sh

# Script for specific ecosystems
build-cleaner --lang node --lang rust --script js-rust-cleanup.sh

# Script for cache categories only
build-cleaner --cache-only --cache-category ai-ml --script ai-cleanup.sh
```

## Tips for Script Review

### Check for Active Projects

Before running, verify that none of your active projects are in the list:

```bash
# Search for specific project
grep "active-project" cleanup.sh
```

### Check Total Size

The script header shows total size to be reclaimed:

```bash
head -10 cleanup.sh
# Shows header with total size
```

### Dry Run the Script

You can modify the script to preview instead of delete:

```bash
# Change rm -rf to echo
sed 's/rm -rf/echo Would delete:/g' cleanup.sh | bash
```

### Partial Execution

Run only certain sections by copying them out:

```bash
# Extract just Node.js section
sed -n '/Node.js Projects/,/^# ═/p' cleanup.sh > node-cleanup.sh
```

## Official Cleanup Commands Reference

Build Cleaner includes these official cleanup commands as alternatives:

| Tool | Official Command |
|------|------------------|
| npm | `npm cache clean --force` |
| Yarn | `yarn cache clean` |
| pip | `pip cache purge` |
| Cargo | `cargo cache -a` (requires cargo-cache) |
| Go | `go clean -cache` |
| Docker | `docker system prune -a` |
| Gradle | `./gradlew cleanBuildCache` |

Using official commands can be safer as they're designed by the tool maintainers, though they may be slower or less thorough than direct deletion.

---

[Back to Home](/) | [Previous: User Caches](../caches/) | [Next: Safety Features](../safety/)
