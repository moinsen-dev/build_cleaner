---
layout: default
title: Safety Features
permalink: /safety/
---

# Safety Features

Build Cleaner is designed with safety as a top priority. Multiple layers of protection ensure you never accidentally delete important data.

## Two-Factor Detection

The most fundamental safety feature is **two-factor detection**. A directory is only identified as a build artifact when BOTH conditions are met:

1. **A project configuration file exists** (e.g., `package.json`, `Cargo.toml`)
2. **The corresponding artifact directory exists** (e.g., `node_modules`, `target`)

### Why This Matters

Without this protection, a directory named `build` or `target` could be mistakenly identified as a build artifact even if it contains important user data.

**Example - Safe Detection:**
```
my-project/
├── package.json      ← Config file present ✓
└── node_modules/     ← Artifact present ✓
                      → DETECTED as cleanable
```

**Example - Not Detected (Protected):**
```
my-data/
└── build/            ← No config file!
                      → NOT detected (could be user data)
```

## Dry-Run Mode

Always preview what will be deleted before actually deleting:

```bash
build-cleaner --dry-run
```

This shows everything that would be cleaned without making any changes. It's recommended to **always run dry-run first** when scanning a new directory.

### What Dry-Run Shows

- Full list of detected projects and caches
- Size of each item
- Total space to be reclaimed
- Breakdown by ecosystem and category

## Confirmation Prompts

By default, build-cleaner asks for confirmation before deleting:

```
Found 42 projects with 74.5 GB of artifacts.
Delete these artifacts? [y/N]
```

You must explicitly type `y` or `yes` to proceed.

To skip confirmation (for automation), use `--yes`:

```bash
build-cleaner --yes
```

**Use `--yes` with caution** - combine it with `--dry-run` first to verify.

## Interactive Mode

Select exactly what to clean with interactive mode:

```bash
build-cleaner --interactive
```

This presents a multi-select interface:

```
Select items to clean (Space to toggle, Enter to confirm):

[x] my-react-app - node_modules (245 MB)
[ ] my-cli - target (1.2 GB)          ← Deselected - will be kept
[x] old-project - node_modules (312 MB)
[ ] important-project - build (156 MB) ← Deselected - will be kept
```

Only selected items are deleted.

## Script Generation

The safest approach is generating a script for review:

```bash
build-cleaner --script cleanup.sh
```

This creates a bash script that you can:
1. Open and read every command
2. Remove lines for items you want to keep
3. Run only after thorough review

See [Script Generation](../scripts/) for details.

## Exclusion Patterns

Exclude specific directories from scanning:

```bash
build-cleaner --exclude important-project
build-cleaner --exclude archived --exclude legacy
```

Excluded directories are skipped entirely during scanning.

## Filtering for Precision

Combine filters to limit scope:

```bash
# Only old, large items
build-cleaner --min-age 60 --min-size 500MB

# Only specific ecosystems
build-cleaner --lang node

# Only specific cache categories
build-cleaner --cache-only --cache-category package
```

## What Build Cleaner Does NOT Delete

Build Cleaner is conservative by design:

- **Source code** - Never touches your actual code files
- **Configuration files** - `package.json`, `Cargo.toml`, etc. are never deleted
- **Git repositories** - `.git` directories are never touched
- **User documents** - Only looks in specified directories
- **System files** - No system directories are scanned

## Warnings for Sensitive Caches

Some caches have special warnings:

### IDE Caches
```
Warning: JetBrains caches - close all JetBrains IDEs before cleaning
```

### Large Model Caches
```
Warning: Hugging Face cache (28 GB) - models will need to be re-downloaded
```

### Docker
```
Warning: Docker prune removes ALL unused images
```

## Recovery

### Re-downloading

Most cleaned content can be re-downloaded:
- `node_modules` → `npm install`
- `target` → `cargo build`
- AI models → Re-download via respective tools
- Package caches → Automatically rebuilt on next use

### Git Clean Fallback

For project artifacts in git repositories, you can also use:

```bash
git clean -fdx  # Removes untracked files and directories
```

This is another safe approach since anything not in git can be regenerated.

## Recommended Safety Workflow

1. **Dry-run first**
   ```bash
   build-cleaner --dry-run ~/work
   ```

2. **Review the output** - Make sure nothing unexpected is listed

3. **Generate a script** (for important cleanups)
   ```bash
   build-cleaner --script cleanup.sh
   ```

4. **Review the script** - Read through and remove anything you want to keep

5. **Run the script**
   ```bash
   chmod +x cleanup.sh
   ./cleanup.sh
   ```

## Troubleshooting

### "Why wasn't my project detected?"

- Check that the config file exists (e.g., `package.json`)
- Check that the artifact directory exists (e.g., `node_modules`)
- Both must be present for detection

### "I accidentally deleted something!"

- Project artifacts: Run the appropriate build command (`npm install`, `cargo build`, etc.)
- Package caches: Will be rebuilt automatically on next install
- AI models: Re-download through the respective tool

### "How can I be extra safe?"

1. Always use `--dry-run` first
2. Use `--script` and review before running
3. Use `--interactive` for granular control
4. Back up critical data before any cleanup

---

[Back to Home](/) | [Previous: Script Generation](../scripts/)
