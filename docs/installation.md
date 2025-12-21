---
layout: default
title: Installation
permalink: /installation/
---

# Installation

Build Cleaner can be installed via multiple methods depending on your platform and preferences.

## Homebrew (Recommended for macOS/Linux)

The easiest way to install build-cleaner is via Homebrew:

```bash
brew tap moinsen-dev/tap
brew install build-cleaner
```

To upgrade to the latest version:

```bash
brew upgrade build-cleaner
```

## Cargo (Rust)

If you have Rust installed, you can install directly from crates.io:

```bash
cargo install build-cleaner
```

This will compile the binary and install it to `~/.cargo/bin/`.

## From Source

Clone the repository and build from source:

```bash
git clone https://github.com/moinsen-dev/build_cleaner
cd build_cleaner
cargo build --release
```

The binary will be available at `target/release/build-cleaner`. You can copy it to a directory in your PATH:

```bash
# macOS/Linux
sudo cp target/release/build-cleaner /usr/local/bin/

# Or add to your local bin
cp target/release/build-cleaner ~/.local/bin/
```

## Pre-built Binaries

Download pre-built binaries from the [GitHub Releases](https://github.com/moinsen-dev/build_cleaner/releases) page.

Available platforms:
- **macOS (Apple Silicon)**: `build-cleaner-aarch64-apple-darwin.tar.gz`
- **macOS (Intel)**: `build-cleaner-x86_64-apple-darwin.tar.gz`
- **Linux (ARM64)**: `build-cleaner-aarch64-unknown-linux-gnu.tar.gz`
- **Linux (x86_64)**: `build-cleaner-x86_64-unknown-linux-gnu.tar.gz`

### Installing a Pre-built Binary

```bash
# Download (example for macOS Apple Silicon)
curl -LO https://github.com/moinsen-dev/build_cleaner/releases/latest/download/build-cleaner-aarch64-apple-darwin.tar.gz

# Extract
tar -xzf build-cleaner-aarch64-apple-darwin.tar.gz

# Move to PATH
sudo mv build-cleaner /usr/local/bin/

# Verify installation
build-cleaner --version
```

## Verifying Installation

After installation, verify that build-cleaner is working:

```bash
build-cleaner --version
# Output: build-cleaner 0.1.0

build-cleaner --help
# Shows full usage information
```

## System Requirements

- **Operating System**: macOS 10.15+ or Linux (glibc 2.31+)
- **Architecture**: x86_64 (Intel) or aarch64 (ARM64/Apple Silicon)
- **Disk Space**: ~5 MB for the binary
- **Dependencies**: None (statically linked)

## Shell Completion

Build Cleaner uses clap for CLI parsing and can generate shell completions. (Coming in a future release)

## Updating

### Homebrew

```bash
brew upgrade build-cleaner
```

### Cargo

```bash
cargo install build-cleaner --force
```

### From Source

```bash
cd build_cleaner
git pull
cargo build --release
```

## Uninstalling

### Homebrew

```bash
brew uninstall build-cleaner
brew untap moinsen-dev/tap  # Optional: remove the tap
```

### Cargo

```bash
cargo uninstall build-cleaner
```

### Manual Installation

Simply remove the binary:

```bash
sudo rm /usr/local/bin/build-cleaner
# or
rm ~/.local/bin/build-cleaner
```

---

[Back to Home](/) | [Next: Usage Guide](../usage/)
