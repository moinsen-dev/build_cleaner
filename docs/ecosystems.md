---
layout: default
title: Supported Ecosystems
permalink: /ecosystems/
---

# Supported Ecosystems

Build Cleaner automatically detects and cleans build artifacts for 9 programming ecosystems. Each ecosystem is identified by its configuration file(s), ensuring only valid project directories are cleaned.

## Detection Method

Build Cleaner uses a **two-factor detection** approach for safety:

1. **Config File Detection**: A project configuration file must exist (e.g., `package.json`, `Cargo.toml`)
2. **Artifact Detection**: The corresponding artifact directory must also exist (e.g., `node_modules`, `target`)

This prevents accidental deletion of directories that happen to share names with build artifact folders.

## Ecosystem Reference

### Node.js

| Property | Value |
|----------|-------|
| Config File | `package.json` |
| Artifacts | `node_modules/` |
| CLI Filter | `--lang node` |

**What Gets Cleaned:**
- `node_modules/` - All installed npm/yarn/pnpm dependencies

**Example:**
```
my-react-app/
├── package.json       ← Config detected
├── node_modules/      ← Will be cleaned
│   └── (thousands of packages)
└── src/
```

---

### Rust

| Property | Value |
|----------|-------|
| Config File | `Cargo.toml` |
| Artifacts | `target/` |
| CLI Filter | `--lang rust` |

**What Gets Cleaned:**
- `target/` - Compiled artifacts, debug/release builds, incremental compilation cache

**Example:**
```
my-cli/
├── Cargo.toml         ← Config detected
├── target/            ← Will be cleaned
│   ├── debug/
│   └── release/
└── src/
```

**Note:** Rust `target` directories can be very large (multi-GB) due to incremental compilation caches.

---

### Python

| Property | Value |
|----------|-------|
| Config Files | `setup.py`, `pyproject.toml`, `requirements.txt` |
| Artifacts | `__pycache__/`, `.venv/`, `venv/`, `.tox/`, `*.egg-info/` |
| CLI Filter | `--lang python` |

**What Gets Cleaned:**
- `__pycache__/` - Bytecode cache directories (found recursively)
- `.venv/` and `venv/` - Virtual environments
- `.tox/` - Tox testing environments
- `*.egg-info/` - Package metadata

**Example:**
```
my-django-app/
├── pyproject.toml     ← Config detected
├── .venv/             ← Will be cleaned
├── my_app/
│   └── __pycache__/   ← Will be cleaned
└── tests/
    └── __pycache__/   ← Will be cleaned
```

---

### Flutter/Dart

| Property | Value |
|----------|-------|
| Config File | `pubspec.yaml` |
| Artifacts | `build/`, `.dart_tool/` |
| CLI Filter | `--lang flutter` |

**What Gets Cleaned:**
- `build/` - Compiled application builds
- `.dart_tool/` - Dart tooling cache and package config

**Example:**
```
my_flutter_app/
├── pubspec.yaml       ← Config detected
├── build/             ← Will be cleaned
├── .dart_tool/        ← Will be cleaned
└── lib/
```

---

### Java (Maven)

| Property | Value |
|----------|-------|
| Config File | `pom.xml` |
| Artifacts | `target/` |
| CLI Filter | `--lang java-maven` |

**What Gets Cleaned:**
- `target/` - Compiled classes, packaged JARs, test reports

**Example:**
```
my-spring-app/
├── pom.xml            ← Config detected
├── target/            ← Will be cleaned
│   ├── classes/
│   └── my-app.jar
└── src/
```

---

### Java (Gradle)

| Property | Value |
|----------|-------|
| Config Files | `build.gradle`, `build.gradle.kts` |
| Artifacts | `build/`, `.gradle/` |
| CLI Filter | `--lang java-gradle` |

**What Gets Cleaned:**
- `build/` - Compiled classes and outputs
- `.gradle/` - Gradle cache and wrapper files

**Example:**
```
my-android-app/
├── build.gradle.kts   ← Config detected
├── build/             ← Will be cleaned
├── .gradle/           ← Will be cleaned
└── app/
    ├── build.gradle
    └── build/         ← Will be cleaned
```

---

### C/C++

| Property | Value |
|----------|-------|
| Config Files | `CMakeLists.txt`, `Makefile` |
| Artifacts | `build/`, `cmake-build-*/` |
| CLI Filter | `--lang cpp` |

**What Gets Cleaned:**
- `build/` - CMake build directory
- `cmake-build-*/` - CLion/IDE build directories (e.g., `cmake-build-debug`, `cmake-build-release`)

**Example:**
```
my-cpp-project/
├── CMakeLists.txt     ← Config detected
├── build/             ← Will be cleaned
├── cmake-build-debug/ ← Will be cleaned
└── src/
```

---

### .NET

| Property | Value |
|----------|-------|
| Config Files | `*.csproj`, `*.fsproj` |
| Artifacts | `bin/`, `obj/` |
| CLI Filter | `--lang dot-net` |

**What Gets Cleaned:**
- `bin/` - Compiled binaries
- `obj/` - Intermediate build files

**Example:**
```
MyWebApp/
├── MyWebApp.csproj    ← Config detected
├── bin/               ← Will be cleaned
│   └── Debug/
├── obj/               ← Will be cleaned
└── Controllers/
```

---

### Go

| Property | Value |
|----------|-------|
| Config File | `go.mod` |
| Artifacts | Go module cache |
| CLI Filter | `--lang go` |

**What Gets Cleaned:**
- Go module cache (via `go clean -cache` command)

**Note:** Go is handled specially. Rather than deleting directories directly, build-cleaner includes the `go clean -cache` command in generated scripts or executes it directly.

**Example:**
```
my-go-service/
├── go.mod             ← Config detected
├── go.sum
└── main.go
```

---

## Filtering by Ecosystem

Use `--lang` to filter which ecosystems to scan:

```bash
# Only Node.js
build-cleaner --lang node

# Node.js and Rust
build-cleaner --lang node --lang rust

# Everything except Flutter
build-cleaner --lang node --lang rust --lang python --lang java-maven --lang java-gradle --lang cpp --lang dot-net --lang go
```

## Summary Table

| Ecosystem | Config Files | Artifacts Cleaned | Notes |
|-----------|--------------|-------------------|-------|
| Node.js | `package.json` | `node_modules/` | Often very large |
| Rust | `Cargo.toml` | `target/` | Can be multi-GB |
| Python | `setup.py`, `pyproject.toml`, `requirements.txt` | `__pycache__/`, `.venv/`, `venv/`, `.tox/`, `*.egg-info/` | Multiple artifact types |
| Flutter | `pubspec.yaml` | `build/`, `.dart_tool/` | |
| Java Maven | `pom.xml` | `target/` | |
| Java Gradle | `build.gradle`, `build.gradle.kts` | `build/`, `.gradle/` | Includes wrapper cache |
| C/C++ | `CMakeLists.txt`, `Makefile` | `build/`, `cmake-build-*/` | IDE build dirs |
| .NET | `*.csproj`, `*.fsproj` | `bin/`, `obj/` | |
| Go | `go.mod` | Module cache | Uses `go clean` |

---

[Back to Home](/) | [Previous: Usage](../usage/) | [Next: User Caches](../caches/)
