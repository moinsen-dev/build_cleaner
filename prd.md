Product Requirements Document: Multi-Language Build Directory Cleaner (Rust CLI)

Overview and Goals

The Multi-Language Build Directory Cleaner is a Rust-based command-line tool that recursively locates and safely deletes build artifacts across various development ecosystems. Its primary goal is to help developers reclaim disk space and reduce clutter by removing compiled binaries, dependency directories, and other generated files that can be recreated. The tool targets common build outputs from Flutter, Rust, Go, JavaScript/Node.js, Python, Java, C/C++, and .NET projects. It runs on macOS and Linux (no Windows support) and emphasizes safety — offering preview/dry-run capabilities and confirmation prompts to avoid accidental data loss. In summary, this product aims to provide a fast, convenient, and secure way for individual developers to clean multiple project directories in one go, instead of manually running separate clean commands for each project or writing ad-hoc scripts.

Goals and Objectives:
	•	Disk Space Reclamation: Free up potentially gigabytes of space by deleting large dependency folders (e.g. Node’s node_modules) and build caches (e.g. Rust’s target folder) across projects ￼ ￼.
	•	Unified Cross-Project Cleaning: Enable a single command to clean build artifacts in many projects of different types, without needing ecosystem-specific commands (like cargo clean, flutter clean, etc.) one-by-one.
	•	Developer Productivity: Save time and reduce human error by automating what is otherwise a tedious manual process (finding directories and running rm -rf on each). The tool should be easy to use, with a polished CLI experience (color output, progress spinners, etc.) that feels friendly and modern.
	•	Safety and Control: Provide mechanisms like a dry-run mode to preview deletions, interactive prompts to select/deselect targets, and configurable ignore/include rules so that no important user files are accidentally removed. Intelligent heuristics will be employed to detect only true build artifacts, avoiding directories that might contain user data.

By achieving these goals, the product will help developers maintain clean working directories and manage disk usage proactively across multiple languages and frameworks.

Detailed Use Cases

The following use cases illustrate how individual developers might use the tool in practice:
	•	Use Case 1 – Reclaiming Disk Space Across Projects: A developer has dozens of projects (a mix of Node.js, Python, Rust, and Flutter) in a workspace directory. Over time, each project’s build artifacts (like node_modules, __pycache__, target folders, etc.) have accumulated, consuming tens of gigabytes. The developer runs the tool at the root of the workspace with a dry-run first to see what would be removed. The tool scans recursively, finds all projects and their build directories, and reports that (for example) 12 Node.js node_modules directories and 5 Rust target directories can be cleaned, totaling ~30 GB. Satisfied, the developer runs the tool without dry-run and confirms the deletion, freeing up that space.
	•	Use Case 2 – Multi-Project Cleanup Before Backup: Before backing up or zipping a collection of projects, a developer wants to remove all ephemeral build files to shrink the size. They run the cleaner in confirm mode (non-interactively, with a “yes to all” flag) on a given parent directory. The tool automatically skips any non-ephemeral directories and quickly removes all identified build artifacts, ensuring the backup contains only source code and essential files.
	•	Use Case 3 – Selective Cleaning in Interactive Mode: A developer working on multiple projects is low on disk space but wants to keep artifacts for projects they updated in the last week (to avoid re-downloading dependencies). They run the tool in interactive mode with a filter to skip recently-used projects. The tool might present a list of found projects and their artifact sizes (e.g. “Project A – 1.2 GB (Node modules), last modified 2 months ago; Project B – 500 MB (Rust target), last modified 3 days ago”, etc.). The developer uses the interactive prompt to select only Project A for cleaning and leaves Project B untouched (since it’s recent). The UI might show checkboxes or a collapsible tree of files for each project, allowing the user to inspect contents before confirming deletion.
	•	Use Case 4 – Routine Cleanup Task: As a maintenance practice, a developer schedules a cron job or simply runs the cleaner manually every month on their projects directory. They use flags to automatically skip confirmation and to exclude certain directories (for example, perhaps a specific project’s build folder they always want to keep). The tool runs quickly (thanks to parallel scanning) and prints a summary of how much space was freed each time.
	•	Use Case 5 – Post-Upgrade Cleanup: After upgrading toolchains (e.g., new Rust version or Node version), a developer may want to purge old artifacts that are no longer needed (like old Cargo build outputs tied to an older compiler). The cleaner can be run with appropriate flags to target outdated artifacts. For example, Rust’s cargo-sweep tool can remove build files for toolchains that aren’t installed anymore ￼. Our tool could expose a similar option or simply rely on the fact that a fresh build will regenerate needed files, so a full clean is safe after upgrades.

These use cases highlight the need for flexibility (interactive vs. automated use), broad language coverage, and safeguards to ensure only the intended files are removed.

Functional Requirements

This section outlines what features and behaviors the CLI tool must support.

1. Multi-Language Build Artifact Detection:
The tool must identify build/output directories for a variety of ecosystems. It should recognize a project’s language by the presence of certain files (configuration/manifests) and, in those projects, locate the standard directories or files that can be safely deleted. The supported ecosystems and their typical ephemeral build artifacts should include, at minimum:
	•	Node.js/JavaScript: Detect by presence of package.json (or node_modules folder itself). Clean the node_modules/ directory and other build artifacts like dist/ or build/ (for front-end bundles, if any). The node_modules folder is usually the largest and should be the primary target ￼. (Note: Lockfiles like package-lock.json or yarn.lock are not to be deleted since they are source-of-truth, not build outputs.)
	•	Rust: Detect by Cargo.toml in a directory. Clean the target/ directory (Cargo build output) ￼. This includes target/debug/, target/release/, etc., which can be quite large. Ensure not to delete Cargo.lock or source files.
	•	Python: Detect by markers of a Python project (such as a setup.py, pyproject.toml, requirements.txt, or presence of a __pycache__ folder). Clean Python bytecode caches and build outputs: e.g. __pycache__/ directories, the build/ and dist/ folders (if present from packaging), any *.egg-info directories, .pytest_cache/ from tests, and possibly remove virtual environment folders (venv/ or .venv) if the user considers those disposable ￼. (Virtual envs can be sizable, but deletion should be confirmed as it means the env would need to be recreated.)
	•	Flutter/Dart: Detect by pubspec.yaml or other Flutter config in a directory. Clean the build/ directory (which contains built app binaries/intermediates for Flutter) and the .dart_tool/ directory (Dart/Flutter tool cache) – the same targets that the flutter clean command would remove ￼. If an iOS Flutter project contains a Pods/ directory (for iOS dependencies), consider that for cleaning as well (since it can be regenerated via CocoaPods). For non-Flutter pure Dart packages, remove build/ and .dart_tool/ as well ￼.
	•	Java: Detect by build system files (e.g. a pom.xml for Maven or build.gradle/build.gradle.kts for Gradle). Clean the standard output directories: for Maven, the target/ folder; for Gradle (and others) the build/ folder ￼. This includes subfolders like build/classes/, build/libs/ (JAR files) etc. Also remove any generated files like .classpath or .project if they’re by-products (though those are usually small).
	•	C/C++: Detect by typical build configuration files (like a Makefile or CMakeLists.txt) in a directory. Clean known build output directories such as build/ (common convention for out-of-source builds) or any cmake-build-*/ directories (as generated by CLion or CMake presets) ￼. Also consider cleaning compiler output subfolders like Debug/, Release/ if present, and object files (*.o, *.obj) or compiled libraries (*.a, *.so) if they reside in a separate output directory. Important: The tool should avoid deleting source files in C/C++ projects (obviously), and not remove any important files like CMakeLists.txt. Only the build artifacts (which are usually under a dedicated build directory or sometimes in a bin/ or lib/ output folder) should be removed.
	•	.NET (C# and others using MSBuild): Detect by presence of .csproj or .sln files (or other project files like .vbproj/.fsproj). Clean the bin/ and obj/ directories, which are where compiled assemblies and intermediate objects reside for .NET projects ￼. This includes subdirectories like bin/Debug/, bin/Release/, obj/Debug/, etc. These can be safely deleted as they will be rebuilt by Visual Studio or the .NET CLI on demand. (NuGet package caches are global, not in project, so out of scope except perhaps a note in docs).
	•	Go: Detect by the presence of a Go module file (go.mod) or other Go project structure. By default, Go does not create large build directories in the project folder (builds produce a binary or use a global build cache). However, if the project vendored its dependencies (there will be a vendor/ directory alongside go.mod), that vendor/ folder can be removed and regenerated with go mod vendor ￼. The tool should delete vendor/ when appropriate. Additionally, if the project produces a binary in a known location (e.g., some projects might put build outputs in a bin/ folder), those could be cleaned too. The Go build cache (in $GOCACHE) is outside the project and not handled by this tool (since it’s a global cache, not project-specific).
	•	Others (future consideration): While not required for the initial scope, the architecture should allow adding more languages easily. For example, PHP (vendor/ via Composer), Ruby (gems in vendor/bundle), Swift (SwiftPM’s .build/ folder), etc., could be added with similar pattern matching ￼. These are out of scope for the first release but noted for extensibility.

We summarize the supported ecosystems and their cleanable directories in the table below:

Ecosystem	Project Indicators (Detection)	Folders/Files to Clean (Build Artifacts)
Node.js / JS	package.json, node_modules/ folder	node_modules/, dist/, build/ (bundled outputs)
Rust	Cargo.toml	target/ (incl. target/debug/, target/release/)
Python	pyproject.toml, setup.py, requirements.txt, etc.	__pycache__/, .pytest_cache/, build/, dist/, *.egg-info, .tox/, .coverage files, venv/ or .venv/ envs ￼
Flutter/Dart	pubspec.yaml (Flutter), or Dart project files	build/ (Flutter build outputs), .dart_tool/ (tool cache) ￼, */ios/Pods/ (if Flutter iOS dependencies)
Java (Maven)	pom.xml	target/ (compiled classes and JARs) ￼
Java (Gradle)	build.gradle, settings.gradle	build/ (compiled classes, libs, etc.) ￼
C/C++	Makefile, CMakeLists.txt, etc.	build/ directory, cmake-build-* directories ￼, possibly bin/ and obj/ if used for outputs
.NET (C#)	*.csproj, *.sln	bin/ (binaries), obj/ (intermediate objects) ￼
Go	go.mod (+ possibly main.go)	vendor/ (if present for vendored deps) ￼; project binaries (if any output folder specified)

(Sources: Standard conventions from official tools and community best practices. Flutter’s flutter clean targets build/ and .dart_tool ￼; Rust, Node, Python, Go support as described in clean-dev-dirs tool ￼; Java, C#, C++ patterns as suggested in open-source project docs ￼.)

2. Deletion Functionality:
For each detected project and its artifacts, the tool will execute deletion of those files/folders. This includes:
	•	Recursively deleting directories like the ones listed above. It must handle large directories reliably (e.g. a node_modules with thousands of files) without choking. Rust’s standard library or optimized crates should be used for safe recursive deletion (e.g. using std::fs::remove_dir_all or an equivalent).
	•	Proper error handling: if a file is locked or a permission is denied, the tool should catch the error, report it in the output (without crashing), and continue with other deletions. It should not halt the entire cleaning process due to one stuck file. Instead, log the issue and skip that folder if necessary (possibly with a warning).
	•	No partial deletes without warning: e.g., if only some files in a folder could be removed, the tool should alert the user that the folder couldn’t be completely cleaned.

3. Safety Features (Dry-Run, Confirmation, Rules):
Safety is paramount. The tool must default to a safe behavior to prevent accidental data loss:
	•	Dry-Run Mode: A --dry-run flag that performs a scan and shows the potential deletion targets and their sizes, without actually deleting anything ￼ ￼. This preview allows users to verify the scope. In dry-run output, the tool should clearly mark it as a simulation (e.g. “Preview mode – nothing will be deleted”) and list each project and what would be removed.
	•	Interactive Confirmation: By default (if not run in an auto-confirm mode), after scanning and listing the targets, the tool should ask for user confirmation before proceeding to delete. This can be a simple “Do you want to proceed? (y/N)” prompt listing the total space to be freed. Alternatively, an interactive selection mode can be triggered (--interactive flag) where the user is presented with each project in a list (perhaps with checkboxes or numbered choices) and can choose which ones to clean ￼. This is especially useful if the user wants fine-grained control (e.g., only delete certain projects’ artifacts). Interactive mode will likely provide a text UI to navigate the list of found items.
	•	“No Prompt” Option: For scripting or power users, a --yes (or -y) flag can bypass the confirmation and proceed to deletion automatically ￼. When this is used, the tool should still output what it’s deleting (unless a quiet mode is also specified), but it will not pause for user input.
	•	Whitelist/Blacklist (Include/Exclude): The tool should allow configuration of additional patterns to include or exclude:
	•	Exclusion (Blacklist): An --exclude or --skip option to skip certain directories or patterns during scanning ￼. For example, a user might want to ensure anything under a directory named backup/ is never touched, even if it matches a known pattern, so they could exclude “backup” or a specific path. By default, the tool should also skip system or irrelevant directories (e.g. .git/ directories, which are not build artifacts and should never be removed). In fact, internal logic will already avoid scanning into .git or other VCS directories as they are not relevant to builds. The skip option just gives users extra control.
	•	Inclusion (Whitelist): A flag or config file to allow custom directory names to be treated as disposable build folders. For instance, if a particular C++ project outputs to a directory called build-x86/ (non-standard name), the user could whitelist that pattern so the tool recognizes and deletes it. By default the tool covers common names (as listed in the table above), but this feature covers edge cases. Whitelisting might be configured via a CLI flag like --include <pattern> or via a config file (e.g., a dotfile where users can list additional directories to clean).
	•	Heuristic Checks: The tool will implement internal heuristics to double-check before deletion:
	•	Only delete directories that match known safe patterns and are in the context of a recognized project. For example, a directory named “build” will not be removed unless we have detected a build config file in that parent directory (to ensure it’s truly a build output, not a coincidental folder). This two-factor detection (config + artifact) greatly reduces risk of false positives ￼ ￼.
	•	If a directory is enormous (e.g. several GBs) but doesn’t match known patterns or lacks an accompanying config file, the tool will not automatically delete it. It may either ignore it or flag it to the user for review. (This prevents scenarios like accidentally deleting a user’s big folder that isn’t a build cache).
	•	The tool never deletes source code files. It targets directories that are generally listed in .gitignore for each ecosystem (which is a good proxy for “generated files”). For instance, it won’t delete a .env file, or a database/ folder containing user data, because those wouldn’t match the known artifact patterns or pass the config heuristic.
	•	No dangerous filesystem operations: The implementation should ensure it doesn’t follow symlinks to outside the target tree. If a build folder is a symlink to some other location, the tool should either skip it or carefully handle it (to avoid deleting files outside the intended scope).
	•	Possibly integrate a “trash” option (optional): instead of permanent deletion, move files to system trash/recycle (on macOS, moving to ~/.Trash). This could be a safety net feature (--trash flag), though by default it can delete permanently; this is a nice-to-have for extra cautious users.
	•	Logging and Confirmation of Actions: After deletion, the tool should output a summary of what was done – e.g., list each directory removed and total space freed. In dry-run it already would list, but in real run, confirming again what was actually deleted gives transparency. In case of any errors (files not deleted), those should be reported in the output along with suggestions (e.g., “Some files could not be removed, possibly due to permissions. Run with sudo or check file locks.”).

4. CLI Options and Commands:
The tool is a single binary with subcommands or flags controlling behavior. No complex subcommands are expected (likely a single command with flags). Below is a matrix of key CLI flags and arguments:

Option/Flag	Short	Description
Positional [DIR]	–	The directory to scan for projects. If omitted, defaults to the current working directory. You can specify a parent folder under which all projects will be searched recursively.
--dry-run	(none)	Runs the tool in preview mode – it will NOT delete anything, but will output all the would-be targets and their sizes ￼ ￼. Use this to review what would happen in a real run.
--yes	-y	Automatic yes to confirmation. Use this to skip the interactive confirmation prompt and immediately proceed with deletion of all found items ￼. (Useful for scripting or running in CI.)
--interactive	-i	Interactive selection mode. Presents each found project and allows the user to choose which to clean (e.g., via an interactive menu with arrow keys & spacebar selection). If this flag is used, it implies dry-run (show list) until the user confirms selections ￼.
--include <pattern>	–	(Optional) Specify additional directory name patterns to clean. For example, --include build-x86 would treat any build-x86/ directories as removable. This can be repeated for multiple patterns.
--exclude <pattern>	–	Skip directories that match the given pattern from scanning/deletion ￼. Can be used to protect certain paths. E.g., --exclude myapp/node_modules to avoid deleting a specific folder, or --exclude .venv if you don’t want virtualenvs removed.
--lang <types>	-p	Filter by project type(s) to clean ￼. For example, --lang node,python would only target Node.js and Python projects, ignoring others. By default, all supported types are scanned. Valid values might be node, rust, python, java, cpp, dotnet, flutter, or all. This helps if you only want to clean certain languages.
--min-age <days>	–	(Optional) Only consider projects that have not been modified in the last N days. Recent projects (with file modifications more recent than this threshold) will be skipped. For instance, --min-age 7 skips anything updated in the last week. This prevents cleaning projects you’re actively working on. (Could also be phrased as --keep-days meaning “keep projects modified in last N days” ￼.)
--min-size <size>	–	(Optional) Only consider artifact directories above a certain size for deletion. For example, --min-size 100MB will skip cleaning any build dirs smaller than 100 MB ￼. This ensures you focus on space-saving targets. Supports human-readable sizes (e.g., “500MB”, “2GiB”).
--verbose	-v	Enable verbose output. This could list every file being deleted and print warnings for any errors encountered, etc. Also might show scanning progress details. Useful for debugging or if you want full transparency.
--version	–	Display the tool’s version.
--help	-h	Show help message with available options.

(Flags --min-age and --min-size represent the intelligent filtering features for time and size; these are inspired by existing tools ￼ but can be adjusted or dropped based on complexity. They are nice-to-have for v1 if feasible.)

5. Polished Console Output:
The output of the tool should be user-friendly and informative:
	•	When scanning, show a spinner or progress bar indicating work in progress (especially if scanning a large directory tree which could take a few seconds). For example, a spinner with text “Scanning for projects…” that turns into a success indicator when done.
	•	Once scanning is complete, display a summary of found projects. For each project, show:
	•	A short name or path of the project (perhaps the folder name).
	•	The type of project (language) with a label or icon (for instance, prefix each line with an emoji or icon representing the language: Rust 🦀, Node.js 📦, Python 🐍, etc., or simply “[Rust]”, “[Node]”, etc.) ￼ ￼.
	•	The size of the build artifacts that will be removed (e.g. “Size: 1.2 GB”). If multiple directories per project are to be removed (say a Python project might have both __pycache__ and build/), either sum them up or list them indented under the project.
	•	Color-code the output for readability ￼:
	•	Use one color for the project name/path, another for the size, etc. For example, project names in cyan, sizes in yellow, warnings in red, success messages in green. Ensure the colors have good contrast on dark/light terminals.
	•	If using icons or emojis for project type, color those or use them consistently (could use a library like colored for ANSI colors ￼).
	•	If interactive mode, present a UI: likely text-based (no graphical UI), possibly using an interactive prompt library to allow moving up/down and selecting entries. For instance, similar to how npkill works – npkill is a Node tool that lists all node_modules folders on disk with their sizes and lets the user navigate and delete with keyboard ￼. Our tool can take inspiration from that: a list with controls to select which ones to delete. The interactive interface might show a collapsible tree: e.g., by pressing an expand key, user can see a tree of subdirectories/files inside a particular artifact directory to inspect what’s inside (this would be a bonus usability feature).
	•	After confirmation, during deletion, show progress:
	•	Could simply print each directory as it’s being deleted (with maybe a checkmark when done).
	•	Or a progress bar overall like “Deleting 5 of 12 projects…”.
	•	If deletion is fast, a simple sequential output is fine; if slow, a progress bar or spinner per directory is nicer.
	•	Final output: a summary line such as “✅ Cleaned 10 projects, freed 4.4 GB of space.” ￼. This gives the user a satisfying confirmation of the outcome.
	•	In verbose mode, list any directories skipped due to exclude rules or because they were recent/small (if using those filters), so the user knows those were intentionally left untouched.

6. Configuration File (Optional/Nice-to-have):
Though not strictly required, consider supporting a config file (e.g., ~/.config/build-cleaner.toml or similar) where users can define default settings: custom include/exclude patterns, default flags (like always use --min-size 10MB by default), or to extend the known patterns for project types. This can help power users tailor the tool without specifying flags every time. The tool should still be fully functional without any config (sensible defaults).

Non-Functional Requirements

These are requirements related to performance, compatibility, and other quality attributes:
	•	Performance: The tool should be fast in scanning and deletion operations. Scanning should utilize parallelism where possible, since searching through many directories can be IO-bound. Using Rust’s concurrency (e.g. threads or async) along with efficient filesystem traversal libraries is expected (for instance, using rayon for parallel directory walks ￼). The goal is that scanning tens of thousands of files should only take a few seconds on an SSD. Deletion speed is primarily IO-limited, but by deleting whole directories with a single OS call (remove_dir_all), it should be efficient. If needed, deletion of many directories can also be parallelized (with caution to not overload disk).
	•	Resource Usage: Memory footprint should be modest – the tool might accumulate a list of found projects, but it should not need to load all file metadata into memory. It can stream the traversal. CPU usage mainly from scanning should be reasonable; it may spike all cores briefly if using multi-threaded scan (which is fine, it’s a short-lived utility run).
	•	Safety/Stability: The program must be stable and avoid crashes, even on unusual inputs. It should handle:
	•	Very long paths or deeply nested directories (common in node_modules). Use Rust’s robust path handling to avoid issues with path length.
	•	Special characters in file names.
	•	Permissions issues (non-fatal errors should be caught and reported, not crash the program).
	•	If any operation fails, it should not leave the system in an unsafe state. Partially deleted directories are generally okay (some files removed, some not) and can be retried, but no corruption of user data should occur (the tool doesn’t modify any files, only deletes whole files/dirs).
	•	Platform Support: Must run on Linux and macOS. It should account for minor differences (for example, filesystem case-sensitivity on macOS vs Linux, APFS peculiarities, etc.). It is not required to support Windows, so we won’t handle Windows-specific paths or ACL issues. However, it should be able to run in WSL (Windows Subsystem for Linux) since that’s essentially a Linux environment – not officially targeted but likely will work if Linux support is good.
	•	Extensibility: The design should allow adding new project types easily. For example, the code might use an enum for project type and a trait or modular functions for detection logic per type ￼ ￼. Adding a new language (say PHP’s vendor directories) would involve adding its detection (look for composer.json + vendor/) and listing its artifact patterns. This modular approach ensures the tool can grow beyond the initially supported ecosystems without a full rewrite.
	•	Maintainability: The codebase should be clean and well-structured. Using popular Rust crates for common tasks (Clap for CLI parsing, Walkdir or similar for file walking, Rayon for parallelism, etc.) is encouraged to avoid reinventing the wheel ￼. Unit tests or integration tests should cover detection logic (e.g., given a dummy folder structure, does it correctly identify and list the right directories?) and possibly a dry-run scenario. Since this is a potentially destructive tool, testing is important.
	•	Usability & UX: Although it’s a CLI tool, it should be approachable:
	•	A clear --help output explaining all options.
	•	Descriptive error messages. e.g., if a user tries to run it on a directory they don’t have permissions for, it should inform “Permission denied on directory X, try running with elevated permissions or skip that directory.”
	•	When nothing is found to clean, it should say so (“No build artifacts found in the specified path.”) rather than silently doing nothing.
	•	When in doubt or if any potential misuse is detected (like running on root /), it should prompt extra caution (maybe require --yes in that case explicitly).
	•	Reliability of Detection: The heuristics for finding project directories should err on the side of caution. False negatives (missing some folders to clean) are better than false positives (deleting something it shouldn’t). For instance, if a build folder doesn’t perfectly match known patterns, the tool might skip it – that’s acceptable because the user can always clean that manually or configure an include. But under no circumstances should it delete a directory that contains user-edited source or configuration. We will leverage known file markers (like presence of pom.xml or Cargo.toml) to increase confidence that a folder is a true project with recreatable outputs ￼.
	•	Concurrency and Thread-Safety: If using multithreading (Rayon, etc.), ensure that shared data (like the list of results) is handled safely (e.g., use thread-safe data structures or collect results via channels). The deletion phase could potentially run in parallel for multiple projects, but it might also be okay sequentially. We must avoid race conditions (for example, two threads trying to delete nested directories that might overlap – the scanning logic should structure projects as separate, non-overlapping units).
	•	Logging and Debugging: Possibly include a debug mode (maybe tied to --verbose) that can output internal steps to a log or console for troubleshooting. This can help if a user is trying to understand why a directory wasn’t deleted, etc., by seeing, for example, “Skipping X because no known config file found” in verbose output.
	•	License and Open-Source: (If relevant) The tool is intended to be likely open-source (since it’s a dev tool). Ensure that any third-party crates used are permissively licensed (MIT/Apache, etc.). We may dual-license our tool under MIT/Apache-2.0 as is common in Rust (like the clean-dev-dirs tool does ￼).

Supported Ecosystems and Patterns for Cleaning

(This section provides more detail on each supported ecosystem’s cleaning rules, expanding on the summary table above.)

Flutter/Dart: The tool will identify Flutter projects by looking for a pubspec.yaml file (which is present in every Dart/Flutter project). If found, it will remove the build/ directory and the hidden .dart_tool/ directory in that project ￼. These contain compiled Dart code, Flutter build intermediates, and dependency caches. This is exactly what the flutter clean command does (we mimic Flutter’s official clean behavior) ￼. Additionally, if the Flutter project’s iOS subdirectory contains a Pods/ folder (from CocoaPods), and possibly an .symlinks/ folder or Flutter.framework (in iOS build), those can be deleted as they are regenerated by pod install or Flutter. (We must be careful to only delete ios/Pods if it exists; some Flutter projects may not have it if no iOS plugins.) For plain Dart (non-Flutter) packages, the convention (per Dart’s tools like dart_clean) is also to remove build/ and .dart_tool/ ￼.

Rust: Rust projects are detected by Cargo.toml. We then remove the target/ directory. Within target, any build artifacts (for all target triples, debug/release profiles, etc.) are wiped. This is equivalent to running cargo clean on that project, but our advantage is we can do it for many projects recursively in one go. Rust’s Cargo does not automatically clean up old artifacts, so target/ can accumulate large files over time ￼. Tools like cargo-sweep address this by removing old artifacts and even provide a recursive search mode ￼; our tool aligns with the idea of recursive cleaning but will typically remove the entire target folder for simplicity (perhaps in the future we can integrate smarter cleaning of only stale artifacts, but initial version will prefer the safe route of cleaning all to guarantee maximal space free).

Node.js / JavaScript: Node projects are indicated by presence of package.json. We remove node_modules/ which is the directory for installed dependencies. This folder is notorious for growing huge and is safe to delete because it can be re-installed via npm/yarn/pnpm. There are existing tools that focus on this, e.g. npkill, which scans for node_modules directories and lets users delete them interactively ￼. Our tool effectively provides similar functionality for Node, integrated with other languages. We also remove other common build outputs in frontend projects: e.g. a dist/ folder (often contains compiled/minified assets), or a generic build/ folder (some frameworks put build output there). However, caution: if a Node project uses “build” as a directory for source (unlikely), our config-file heuristic (checking for package.json and maybe checking if “build” contains .js or just generated files) will help ensure it’s a build output. Perhaps requiring that build/ has no source code and mostly generated content (like .js files that are transpiled outputs) – but implementing that check might be complex, so initially we assume if package.json exists and there’s a build/ directory, it’s probably output of a build script (common in React apps, etc.). The tool will not delete config files like webpack.config.js or similar, obviously.

Python: Python’s variety of build artifacts includes compiled bytecode caches and packaging outputs:
	•	__pycache__ directories: found in virtually every Python package directory after running code. These can be safely removed (they will be regenerated as needed). The tool should find and delete all __pycache__ folders within a project.
	•	Packaging/build outputs: If the project has been built (e.g. via python setup.py bdist or modern build tool), there may be a build/ directory and a dist/ directory with wheels or distributions. These are typically disposable because you can rebuild the package. Remove both if present.
	•	Egg info: Installing or building might create a *.egg-info directory (name varies with the package). Remove those as they are metadata caches.
	•	.pytest_cache: created by pytest runs, safe to delete.
	•	Virtual environments: Many projects include a venv or .venv folder (especially if using venv for dependencies instead of a global interpreter). These can be quite large (since they contain copies of the Python interpreter and installed packages). The tool can remove them to save space, but the user will have to recreate the env. Because some developers might treat a venv as semi-permanent, we could consider not deleting virtualenvs by default unless a flag is set or they are explicitly included. However, our detection logic (from an existing project example) does list venv/.venv among things to clean ￼. Perhaps we include it by default since it’s usually re-creatable (requirements.txt or Pipfile is the source of truth). We should call out in documentation that virtual environments will be deleted if found, unless excluded.
	•	Project detection: We look for standard files like pyproject.toml (PEP 517+modern Python projects), setup.py or setup.cfg (legacy packaging), Pipfile/poetry.lock (if using Pipenv or Poetry), or even requirements.txt ￼. If one of these is present, it confirms it’s a Python project and thus the above directories are fair game for cleaning.

Java: Two main build systems:
	•	Maven (pom.xml): If a pom.xml is present, and especially if a target/ directory is present, we remove target/ ￼. That will include classes, JARs, WARs, etc. Maven’s clean goal does exactly that (deletes target).
	•	Gradle (build.gradle or gradlew script): Remove the build/ directory ￼. This contains compiled classes (build/classes), jars (build/libs), and other generated resources. We should also remove the Gradle build cache directory if it’s inside the project (though normally Gradle’s cache is global, not in project, so not applicable).
	•	If a project has both (e.g. a multi-module with a pom and Gradle, unlikely), we handle accordingly. Also, some Java projects might use other tools (Ant, etc.), but those often have custom build directories – not in initial scope unless we find an Ant build file and guess the output (could be build/ as well by convention, or defined in build.xml).
	•	We should be careful not to delete source files like src/ directory. Our patterns (target/, build/) are distinct from source (src).

C/C++:
	•	The presence of Makefile could indicate a make-based build. Often such projects either build in-place (producing .o and binary in the same directory) or have a convention like using a /build directory if the user set it up. We can’t blindly remove every .o file scattered around (that could be risky and it’s hard to identify systematically without potentially matching false positives). So a safer approach: if we see a dedicated build directory (common in CMake or manual conventions), remove that entirely.
	•	CMake projects: Presence of CMakeLists.txt. Often developers do out-of-source builds (i.e., they create a separate build directory at some path, like build/ or with CMake’s default on some IDEs cmake-build-debug/ etc.). We can detect a CMake build directory by known file presence: a CMakeCache.txt file inside it, or a subfolder CMakeFiles/. Possibly our scanning can catch that: if CMakeLists.txt exists in a folder and that same folder has a subdirectory named build or cmake-build-<something>, that subdirectory is the artifact dir to remove ￼. We should also consider make projects: if Makefile exists and a build/ exists, likely safe to assume it’s build output (though not guaranteed).
	•	Another approach: treat build/ as a generic disposable dir in any project that also has some recognized build config (this covers C/C++ and others). This is essentially what we do, with exceptions for languages that use build/ for source (rare).
	•	In some cases, C/C++ build artifacts might be in bin/ or lib/ directories at project root (especially for installed outputs). However, sometimes those might be checked in (like a repository might include a bin/ directory for scripts). It’s tricky. We may skip removing bin/ for C/C++ unless we have high confidence it’s build output (like maybe if it contains .exe or .out files and a Makefile exists).
	•	We can document that initial version focuses on obvious build directories, and not exhaustive removal of every object file, to avoid mistakes.

.NET (C# and friends):
	•	Detection via *.csproj or *.sln. If found, remove bin/ and obj/ subdirectories ￼ in that project directory (and in any project subdirectories if it’s a solution containing multiple projects). For a .NET solution, the root might have a .sln but actual csproj files in subfolders; we should catch those subfolders too. Possibly treat each csproj as a project.
	•	These bin and obj directories are safe to delete – Visual Studio’s Clean operation typically removes them, and developers often do this to troubleshoot or ensure a full rebuild ￼ ￼. We basically automate doing a recursive clean across all projects instead of one solution at a time.
	•	NuGet caches (global) and user files (like appsettings, etc.) are not touched.

Go:
	•	If go.mod is present, we consider it a Go project. The primary thing to delete is vendor/ if it exists ￼, because that’s basically a copy of dependencies. Many Go projects don’t vendor by default (since Go modules allow on-the-fly fetching or caching elsewhere), but if they do, it can be sizable. We detect vendor/ and remove it.
	•	Go doesn’t have a standard “build directory” – compiled binaries either end up in the current directory or in $GOPATH/bin. If a Go project has produced a binary in the project folder, we can’t generically identify it except by file type (ELF or Mach-O binary). That might be overkill to identify and remove automatically (and a binary might be important or named generically). So we won’t try to remove standalone binaries. The user can always manually delete those if needed.
	•	The Go build cache (go build cache) is stored outside (usually in ~/Library/Caches/go-build on macOS or $GOCACHE env), which this tool will not handle as it focuses on project directories, not user cache dirs.

Edge Cases in Detection:
	•	Projects that combine languages (e.g. a Node.js project that has a subfolder which is a Rust project for NAPI module). Our tool might detect two separate projects nested. It should ideally handle that gracefully (it might list the Rust sub-project and the Node project separately, possibly even listing the same parent path twice with different artifact targets). That’s fine, as long as it doesn’t double delete anything. We should ensure if one artifact is inside another (like nested node_modules inside a larger repo), we either handle it as part of one project or individually but not conflict. Possibly the scanner will find the top-level first and then skip descending into known artifact dirs to avoid redundant processing.
	•	Very large monorepos: e.g., a Google-style monorepo with multiple projects. The scanner should not assume a single “root” config file; it should find all occurrences of known project patterns in subdirectories. It might detect dozens of sub-projects. This is intended.

Safety and Heuristic Mechanisms

Safety is built into functional requirements, but here we detail the approach and reasoning behind detection heuristics and protective measures:

1. Two-Factor Project Detection: The tool will use combined signals to identify a project’s build artifacts:
	•	Signal A: Config/Manifest File Presence. Each supported ecosystem has one or more well-known manifest files (see table above, e.g. Cargo.toml, package.json, pom.xml, etc.). The presence of such a file in a directory strongly indicates that directory is the root of a project of that type.
	•	Signal B: Artifact Directory Presence. The expected build artifact directory (or file) should exist for that project. For example, if Cargo.toml is present, we expect a target/ directory to be present (otherwise, if the project was never built, there’s nothing to clean!). If package.json is present, a node_modules/ directory likely indicates the project has been built or dependencies installed. By requiring both the config and the artifact directory to exist, we avoid picking up directories that coincidentally have a name like a config file or a directory name. This is exactly how one open-source tool approaches detection to avoid misidentification ￼ ￼.
	•	In some cases, an artifact dir might exist without the config (e.g., a stray node_modules copied somewhere). We will generally ignore those unless --include explicitly forces it, because it might be out-of-context and we can’t be certain it’s safe. Conversely, if a config exists but no artifact dir, there’s nothing to do (skip it).
	•	Using this approach, we dramatically reduce risk: e.g., we won’t delete a build/ directory in a folder that lacks any recognizable build config file (maybe that build is something else). And we won’t delete a directory just because it has a config if the expected artifact isn’t there (meaning maybe the user hasn’t run a build – nothing to delete anyway).

2. Default Skip Rules: The scanner will ignore certain directories by default to limit scope:
	•	Version Control Dirs: .git/, .hg/, .svn/, etc., will be skipped entirely during scan. These are never targets for cleanup by our tool, and skipping them also speeds up the scan.
	•	System/Library Dirs: If the user accidentally points the tool at a very high-level directory (like / or their home folder), the tool should avoid descending into obviously inappropriate paths. E.g., on macOS, directories like /System, /Applications should be skipped (since no user projects there). On Linux, directories like /usr, /lib etc. (unless the user specifically goes there, which they shouldn’t). We might hardcode a safety stop: if the given root path is / or something suspicious, require a --force to proceed, to prevent catastrophic misuse.
	•	User-Installed SDKs: As a special case, consider what dart_clean’s documentation warned: if someone has the Flutter SDK itself inside the scanned directory, it might have a .dart_tool that is actually the SDK’s cache, not a project’s ￼. Deleting that could break the SDK. Our tool should detect that scenario (for instance, the Flutter SDK directory has a structure where flutter/bin, etc. — we can identify it and skip). Similarly for Node, if someone scans the global npm cache or .nvm directory, skip those. In general, we assume the user points to a code projects folder, but we add these checks just in case.
	•	The user can also add extra skip patterns via --exclude. For example, if their projects directory contains some archives or data directories named in a way that might confuse the tool, they can exclude them.

3. Confirmation and Abort: Before deletion (except in --yes mode), the user gets a final confirmation prompt listing what will be removed and asking to proceed. This is a safety net. If the list looks wrong to the user, they can abort and nothing is deleted. In interactive mode, the user has fine control to exclude certain items. This addresses the “intelligent detection” aspect by putting a human in the loop for final decision when not absolutely sure. We assume users will typically trust the defaults after seeing it work a few times, but it’s important for first-run.

4. File Content Heuristics (Future/Advisory): In edge cases, we could incorporate additional checks such as:
	•	If a directory named build contains a lot of source files (e.g. .cpp or .dart files) and not typical compiled files, maybe issue a warning or skip it, suspecting it’s not purely a build output.
	•	If a node_modules directory is extremely small (maybe just one or two modules, unusual) without a package.json, it could be something else — but realistically node_modules is unique enough.
	•	These are secondary heuristics and may not be implemented in v1 due to complexity. Our primary heuristic (config + known name) should suffice in most cases.

5. Transaction Safety: Deletion operations will essentially be atomic per directory (remove the whole folder). We should ensure we construct the correct absolute path and double-check it before deletion. Possibly, an extra safety: before deletion, the code can verify that the path about to be removed matches the expected pattern (as a last sanity check). E.g., ensure the directory name is one of the known artifact names or in the include list, and not “.” or “/” or something dangerous. This protects against any bug that might calculate a wrong path.

6. Handling of Symbolic Links: If an artifact directory is a symlink (e.g., node_modules symlinked to a central location — some might do this), by default remove_dir_all will remove the symlink itself, not the target (which is good). We should not traverse into symlinked directories that point elsewhere because that could delete out-of-scope files. Our scanner likely should either treat symlinks as files (not dive in), or provide an option to follow them if the user really wants (but default off). This detail will be noted in docs: symlinked artifact directories are removed as links, not as the actual target directory (to be safe).

7. Testing of Safety: As part of development, create dummy scenarios to test safety:
	•	A folder named build in a non-project context (should not be deleted).
	•	A Cargo.toml present but no target directory (nothing should happen).
	•	A tricky case like a project named “myapp.node_modules” (unlikely, but ensure pattern matching doesn’t go haywire – likely we match whole directory names).
	•	Ensure that excluding .git and such indeed prevents any deletion in those.

By combining these safety mechanisms, the tool aims to provide “intelligent detection” — focusing on true build artifacts — and avoid any deletion of user-created content. The user’s trust is paramount, so these safeguards will be clearly documented and enabled by default.

Proposed Architecture and Components (Rust Implementation)

The tool will be implemented in Rust for performance and safety. Here we outline the high-level architecture and key components/modules:

1. CLI Interface Module: Using a crate like Clap (Command Line Argument Parser) for parsing arguments ￼. This will define the CLI flags (--dry-run, --interactive, etc.) and help text. Clap can also handle config file loading if we choose (via derive or builder patterns). The CLI module will parse args into a Config struct (containing booleans for dryRun, interactive, etc., and lists for include/exclude patterns, selected languages, etc.).

2. Scanner Module: This is responsible for walking through the filesystem from the specified root directory to find projects and their artifact directories.
	•	It may use a crate like Walkdir or simply std::fs with recursion. To speed it up, we can use Rayon to parallelize the scanning of subdirectories ￼. For example, get an iterator of subdirectories and use par_iter to check them concurrently.
	•	The scanner will enforce skip rules (don’t descend into .git or exclude patterns).
	•	For each directory encountered, it will try to detect if it’s a project root. This can be done by checking for known config files in that directory. If a config file is found, then check for corresponding artifact directories within that directory.
	•	Alternatively, approach it from the other side: look for known artifact directory names and verify their parent contains a config. For example, whenever we see a directory named “node_modules”, check if its parent has package.json. This might be efficient using filesystem search (like using glob patterns). But implementing our own search is fine since we have finite known names.
	•	We can implement specific detect functions, e.g. detect_rust_project(dir) -> Option<Project> that checks Cargo.toml and target in dir, similarly for others. These could be encapsulated in a ProjectType enum as in the clean-dev-dirs project ￼.
	•	A Project struct could hold: type (Rust/Node/etc), path to project root, and a list of artifact directories (with their sizes perhaps).
	•	The scanner should compute the size of each artifact directory (for reporting). This can be done by iterating its contents. Since size calculation could be heavy if there are many files, we might only do it in dry-run or when listing. But to decide filters like --min-size, we do need to know sizes. We can parallelize size computation too.
	•	Use of threads: e.g., spawn tasks for each found artifact to sum file sizes.

3. Data Structures:
	•	ProjectType enum (Rust, Node, Python, etc.) to tag projects ￼.
	•	Project struct with fields: name (maybe derived from directory or config), type, path, artifacts: Vec<PathBuf> (the directories/files to delete), and maybe total_size. Name extraction could be done by reading config (e.g., Cargo.toml [package].name, or for Node, package.json’s name, etc.) – this is a nice touch for output clarity but not strictly necessary. The clean-dev-dirs does name extraction for some (they mention Python name from pyproject, Go module name from go.mod ￼). We can include that as a non-critical feature.
	•	Config struct for user options (from CLI).

4. Filter Logic:
	•	After scanning, apply filters: if --min-age given, drop any project whose last modified time is within that range (we can get the project folder’s mtime or the artifact folder’s mtime for last build time).
	•	If --min-size given, drop any whose size is below threshold.
	•	If --lang filter is given, drop projects not matching those types ￼.
	•	These filters narrow down the list to actually clean.
	•	We should apply filters after detection to ensure scanning isn’t too complicated by them (the scanner finds everything, then we filter, except skip patterns which we apply during scan to avoid going in).

5. User Interaction Module: Depending on mode:
	•	Dry-run mode: Just output the list of projects and artifacts, with sizes.
	•	Normal (non-interactive) mode: Output list, then prompt yes/no. Use Rust STDIN to read user input. If “yes”, proceed to deletion; if “no” or just Enter (default no), abort.
	•	Interactive mode: For a richer interface, we might use a crate like Dialoguer or Inquire which supports multi-select prompts in the terminal. This would present a scrollable list with checkboxes. Given potentially many projects, ensure the UI can handle it (maybe use paging or a curses-like interface). Dialoguer’s MultiSelect could work by providing a list of strings (each string can be the project info line, perhaps colored or with an icon).
	•	In interactive selection, after user makes choices, we refine the list to those selected.

6. Deletion Module:
	•	Implements the actual removal of files. Likely straightforward calls to fs::remove_dir_all for each artifact directory. Wrap these in error handling.
	•	If performance is a concern, could spawn threads to delete different projects in parallel (especially if each is large, parallel deletion could speed up if on different disk parts, but on a single disk it may just cause more head movement – on SSD it might be fine though). Maybe limit parallel deletes to a small number to avoid IO contention.
	•	Ensure to delete the correct path (for safety, might log “Deleting {path}…” before doing it in verbose mode).
	•	After deletion of each project’s artifacts, optionally remove now-empty parent directories if they were strictly build artifact containers. However, usually these artifact dirs are within the project which itself remains. We do not delete project directories themselves, just contents, so nothing should become empty except the artifact folder we removed entirely.
	•	After all deletions, compute total space freed (we will have sizes from before, but could also sum sizes of removed items if we trust those computations).

7. Output Formatting:
	•	Use Colored or similar crate to apply ANSI colors to strings ￼.
	•	Use Indicatif for progress bars/spinners ￼. For example, show a progress bar with the number of directories scanned vs total (if total is known; if not, a spinner might suffice).
	•	Use emojis or icons for project type – could define in the ProjectType enum Display trait (like 🦀 for Rust, 🐢 for Go perhaps, 🐍 for Python, 📦 for Node, etc.) ￼.
	•	Structure the output with clear headings, maybe something like:

Found 15 projects to clean:
 1. 🦀 rust-project (Rust) – target/ (2.3 GB)
 2. 📦 web-frontend (Node.js) – node_modules/ (856 MB)
 3. 🐍 ml-project (Python) – __pycache__/, build/ (1.2 GB)
Total potential space to free: 4.4 GB

(Similar to the sample output snippet in an existing tool ￼.)

	•	Then ask “[Proceed? y/N]” if not auto-confirm.

8. Comparisons and Architecture Influence:
	•	We note that a similar Rust tool clean-dev-dirs has a modular architecture for adding languages, and uses Clap, Rayon, Colored, Indicatif ￼. We will take a similar approach given its success.
	•	For interactive UI, that project used a simple prompt approach (or did it? It mentions interactive but likely not a full TUI). If needed, we might integrate a crate like crossterm or ratatui (formerly tui-rs) for a full-fledged text UI with collapsible sections. However, that significantly increases complexity. A simpler approach is to list and prompt.
	•	Since our product emphasizes a “polished” console UI, investing in a user-friendly interactive mode is worthwhile even if it adds a dependency.

9. Components Summary:
	•	main.rs: Entry point, parse CLI, orchestrate calls to scanner and deletion.
	•	cli.rs: CLI arg definitions (Clap derive or builder style).
	•	scanner.rs: Functions to scan directories. Possibly separate functions for each language detection.
	•	project.rs: Definition of Project struct and ProjectType enum, with helper methods (like fn size(&self) -> u64 to sum artifact sizes, fn display_name(&self) to format name).
	•	cleaner.rs: Functions to perform deletion given a Project (or given an artifact path).
	•	output.rs (optional): Helper for formatting output messages, maybe spinner management (Indicatif integration).
	•	If interactive complexity grows, possibly an interactive.rs for the UI logic.

10. Error Handling & Logging:
	•	Use Rust’s Result and Error patterns to propagate errors from filesystem operations up to main, where they can be logged. Non-fatal errors (like failed deletions) should not use panic, just log and continue.
	•	Possibly integrate a logging crate for debug mode, or just use println!/eprintln! with verbose flag.

In summary, the architecture leverages Rust’s strengths (speed, safety, strong CLI libraries) to ensure the tool is efficient and reliable. The modular design will make it easy to extend to new languages or adjust detection rules as needed. The use of proven libraries (Clap, Rayon, etc.) reduces the chance of bugs and speeds up development.

CLI User Experience and Interactions

Even as a CLI, user experience is crucial. We want the tool to feel intuitive and even enjoyable to use, with clear feedback and minimal frustration. Below are specific UX considerations:
	•	Color and Formatting: All output should be formatted for quick scanning. Important elements (like directory names, sizes, prompts) will be highlighted. For instance:
	•	Use green text for successful operations or confirmation messages (e.g., “Clean complete!”).
	•	Use yellow or blue for informational messages (e.g., listing items).
	•	Use red for warnings or anything requiring caution (e.g., “Are you sure?” prompt, or errors like “failed to delete X”).
	•	Align columns (project name, size) for readability as shown in sample output above. Possibly use fixed-width formatting or padding.
	•	Provide legends if icons are used: e.g., at the top or bottom of the list, show a mapping of icons to project types (🦀 Rust, 📦 Node, 🐍 Python, etc.), unless it’s obvious from context ￼.
	•	Interactive Controls: In interactive mode, the tool transforms from a static list to a navigable interface:
	•	Navigation: arrow keys (up/down) to move through the list of projects.
	•	Selection: spacebar or maybe the Enter key to select/deselect a project for cleaning. A selected item might be marked with an [*] or a different color.
	•	Expand/Collapse: perhaps if a project entry is highlighted, pressing right arrow or > could toggle showing its artifact files. This would list, indented, maybe the top-level contents of that directory (to quickly inspect if it looks safe). Left arrow or < to collapse. This is a bonus feature and can be implemented if using a TUI framework. If not, a simpler approach: a separate flag --list-files could print all files to be deleted for each project in the dry-run output, if user wants detail.
	•	Confirmation: in interactive mode, after selection, pressing a key (maybe d for delete or just Enter) will ask one more “You have selected N projects to clean, proceed? (y/N)” as final confirmation.
	•	Provide an obvious way to exit without doing anything (e.g., pressing q to quit interactive mode).
	•	The interactive interface should handle window resizing or large lists (perhaps showing a page at a time if too many entries).
	•	Responsive Feedback: As soon as the user runs the command:
	•	If scanning takes more than half a second, show a spinner or a message “Scanning…”. This prevents the user from wondering if the tool is stuck. The spinner (using Indicatif, for example) can rotate or a simple text animation can be used. Once scanning is done, replace it with the results.
	•	If using progress bars (Indicatif), something like: “[####……..] 40% Scanned” showing how many directories have been processed out of an estimated total. Estimating total is tricky unless we count beforehand. A simpler approach: show count of projects found so far and still scanning.
	•	Ensure that the terminal UI does not glitch when printing colored output or when switching to interactive mode (clear screen or use alternate screen if needed in a TUI).
	•	Examples in Help: In the --help text, include usage examples to guide new users:
	•	e.g. “cleaner ~/Projects --dry-run – preview cleaning for all projects under ~/Projects.”
	•	e.g. “cleaner . -y -p node – immediately remove all node_modules in current directory and subdirs, no prompt.”
	•	This reduces the learning curve.
	•	Comparisons for UX: Tools like npkill provide a very user-friendly way to free space for Node projects, with a scrollable list and size display ￼. We aim for similar convenience but across all languages. Another example is the cargo-sweep and similar which are command-driven (less interactive). We combine the benefits: a quick one-shot command with flags for scripting, and an optional interactive mode for manual control. This dual approach covers both audiences.
	•	Output verbosity levels:
	•	Normal mode: concise summary of each project (one line each, plus total).
	•	Verbose mode (-v): might list every single folder/file being deleted under each project. Possibly not default because that can be thousands of lines for a big node_modules. But it can list maybe top-level of each artifact directory, and say “… (1234 more files)” if too many, just to indicate scope.
	•	Extremely quiet mode: if user only cares about the summary, we might offer --quiet to suppress the per-project listing and just show final stats. Not a must, but some might want it for scripting output (or rely on exit code only).
	•	The tool’s exit code: should be 0 on success (even if some files couldn’t be deleted but majority were, maybe still success but with warning), and non-zero if a major failure happened (like couldn’t access the path at all). This matters for integration in scripts.
	•	Console UI Libraries: If implementing a richer interactive UI, consider using high-level libraries:
	•	Cursive or RatATUI (tui-rs) for full control – could be overkill but powerful.
	•	Or minimal approach with dialoguer which can do multi-select and might suffice.
	•	Using these ensures compatibility across Unix terminals and proper handling of input keys. They also allow nice extras like clearing the screen or using an alternate buffer so that after quitting interactive mode, the original terminal content is restored (so the interactive menu doesn’t remain printed).
	•	Accessibility: Although not a GUI, we should ensure the color choices are discernible (also consider colorblind users – maybe use distinct text labels alongside color). Also if someone can’t use interactive mode (no TTY, or just preference), all features must be accessible via non-interactive flags (which we do provide, e.g., filters and --yes flags).
	•	Usage Flow Examples (textual):
Example normal run output (conceptual):

$ build-cleaner ~/dev_projects
Scanning /home/alice/dev_projects ... 🔍
Found 5 projects with cleanable artifacts:
  🦀  backend-service       (Rust)     target/        812 MB
  📦  web-app-client        (Node.js)  node_modules/  1.4 GB
  🐍  data-analysis         (Python)   __pycache__/   30 MB
  🐹  experiment-go         (Go)       vendor/        45 MB
  🦋  flutter_mobile_app    (Flutter)  build/         200 MB
Total space that can be reclaimed: **~2.5 GB**

Proceed to delete these directories? [y/N]

If the user presses y, then:

Deleting...
 ✅ Removed rust project 'backend-service/target' (812 MB)
 ✅ Removed node project 'web-app-client/node_modules' (1.4 GB)
 ✅ Removed python project 'data-analysis/__pycache__' (30 MB)
 ✅ Removed go project 'experiment-go/vendor' (45 MB)
 ✅ Removed flutter project 'flutter_mobile_app/build' (200 MB)
Clean complete! Freed approximately 2.5 GB.

(If any errors occurred, lines with ❌ and an explanation would appear, but assume none here.)
Interactive mode might initially show the same list but with controls to select.

By focusing on clear visuals, confirmations, and ease of navigation, the tool’s console UX will make the cleaning process straightforward. The user should feel in control and well-informed about what the tool is doing at each step.

Comparisons to Similar Tools

Before this tool, developers have used a variety of methods to clean build artifacts. Here we compare our solution to existing tools or practices:
	•	Per-language Clean Commands: Each ecosystem typically provides its own clean mechanism:
	•	Rust: cargo clean removes that one project’s target folder, but doesn’t help if you have many projects. The community tool cargo-sweep enhances this by cleaning recursively across projects and optionally only old artifacts ￼. However, cargo-sweep is Rust-specific. Our tool generalizes this idea to multiple languages in one binary.
	•	Node.js: Developers often delete node_modules manually or use rimraf scripts. Npkill is a dedicated tool that finds all node_modules on a disk and shows an interactive list for deletion ￼. It’s very similar in spirit to our Node functionality, even offering an interactive UI. Npkill is implemented in Node and mainly targets Node projects. Our tool offers the same convenience for Node plus other types, and being in Rust it might run faster and not require a Node runtime.
	•	Python: No ubiquitous tool exists solely for cleaning Python caches; most developers use find . -name "__pycache__" -delete or manually remove build folders. Our tool codifies this in an automated way, looking at config files to be safe. There is pip-cache for pip’s download cache, but that’s different (global, not project-specific).
	•	Flutter: The flutter clean command is the official way to clear Flutter build artifacts ￼. But it must be run per project. If you have many Flutter projects, you’d have to script something. There is a community tool like dart_clean which can recursively run flutter clean in multiple folders ￼. That is quite similar in aim, but again limited to Dart/Flutter. Our tool would achieve the same for Flutter projects while also covering others.
	•	Java/Gradle/Maven: Typically use mvn clean or gradle clean. These remove the build outputs for that project. Again, multiple projects require multiple commands. Also, if someone isn’t sure where space is going, they might not realize old build folders linger; our tool helps by scanning.
	•	.NET: Visual Studio’s Clean Solution only cleans one solution at a time, and doesn’t delete across different solutions. Some PowerShell scripts exist to wipe all bin/obj recursively ￼ or developer-made tools (e.g., a VS extension CleanBinAndObj was mentioned in community ￼). Those are Windows-specific or require VS. Our tool would let a .NET Core developer on Linux/Mac do similar cleanup easily.
	•	General Disk Cleaners: OS-level disk cleanup tools (like CCleaner on Windows, or built-in Disk Utility on macOS) usually don’t target source repositories. They might clear system caches but not your project’s node_modules. Our tool is specialized for developer artifacts, providing more targeted cleaning.
	•	Shell Scripts and Find commands: A lot of developers solve this ad-hoc, e.g., using find:
	•	Example: find ~/Projects -type d -name node_modules -prune -exec rm -rf {} + to wipe all node_modules. Similarly for target or bin/obj. While effective, these one-liners are risky if miswritten, and they don’t give feedback on how much was freed or allow selective control. One wrong rm -rf pattern could delete more than intended. Our tool encapsulates this logic with safeguards (requiring confirmation, checking for config files, etc.), making it much safer and user-friendly.
	•	Shell scripts also typically lack features like a dry-run preview or interactive selection. Our CLI provides those niceties out-of-the-box.
	•	Existing Multi-language Tools: There are a few emerging tools:
	•	The open-source clean-dev-dirs (Rust crate) which we found is very similar to what we propose. It supports Rust, Node, Python, and Go currently ￼. It has features like parallel scanning, interactive mode, filters by size and age, etc. ￼ ￼. Our project is essentially in the same domain but aims to extend to even more languages (Flutter, Java, C#, C++). We should acknowledge that project as inspiration (if this PRD were for an internal product, we’d note it as a reference implementation). We plan to differentiate by broader ecosystem support and possibly an even more polished UI, since we specifically mention spinners and collapsible views (some of which clean-dev-dirs has already implemented with colored output and progress bars ￼ ￼).
	•	Other tools might exist in smaller scope (like the Dart one, npkill, etc., as mentioned). But there isn’t a well-known, single “universal build cleaner” tool dominating the space (as of 2025). This product aims to fill that gap comprehensively.

Comparison Table:

Tool / Method	Languages Supported	Interactive UI	Safeguards (Dry-run, confirm)	Notes
Our Rust Cleaner	Node, Rust, Python, Go, Flutter, Java, C/C++, .NET (C#)	Yes (optional)	Yes (dry-run, prompts, filters, etc.)	Broad coverage, high performance
clean-dev-dirs ￼	Node, Rust, Python, Go	Yes	Yes (dry-run, interactive, filters) ￼ ￼	Rust tool, very similar core features, fewer languages (no Java, etc.)
npkill	Node.js only (node_modules)	Yes (full TUI) ￼	Implicitly yes (you choose interactively)	Node-focused, requires Node runtime
cargo-sweep	Rust only	No	No (but cleans only outdated by default)	Cargo subcommand, good for Rust cache specifically ￼
flutter clean	Flutter only	No (per project)	N/A (built-in command, no preview)	Must run per project ￼
Custom Scripts	Whatever user codes (e.g. find + rm)	Possibly (if coded)	Typically no (unless script adds it)	Error-prone, not user-friendly

In essence, our tool combines the functionality of several single-language tools into one, and adds a cohesive user experience. There’s an efficiency and convenience gain for developers who work in polyglot environments (which is increasingly common).

Potential Limitations and Edge Cases

No product is without limitations. Here we outline what our tool might not handle and how we plan to mitigate or accept these edge cases:
	•	Windows Not Supported: As specified, the tool will not run on Windows (at least initially). This means Windows developers cannot use it in their PowerShell or CMD. Mitigation: focus on Mac/Linux developers (which include many using VS Code, WSL, etc.). Possibly suggest using WSL for Windows users as a workaround if they want to clean projects on Windows drives (though careful with path differences). We explicitly mention no Windows support in documentation to set expectations.
	•	Incomplete Language Coverage: While we cover many major ecosystems, there will always be more (PHP, Ruby, Swift, etc., as noted in future ideas). Those are not handled in v1. So the tool might miss build files for an unsupported language. For example, if someone scans a Ruby project, our tool currently wouldn’t delete the vendor/bundle directory (since we didn’t implement Ruby detection yet). The outcome is simply that it reports “no project” there, and nothing happens. This is safe (no deletion), but not helpful for that project. We consider this acceptable because adding support is planned and the user can always manually clean or contribute to the tool’s development ￼. We will likely mention in documentation which languages are supported and encourage community contributions to add more.
	•	False Negatives (Missed Cleanups): Some edge cases might cause the tool to not delete something that is actually safe to delete:
	•	If a project uses non-standard directory names for outputs, and the user hasn’t whitelisted them, we will skip them. Example: a C++ project that puts build artifacts in a dir called “out/” which we don’t recognize. We’d miss it. The user might notice space still used. Mitigation: user can add --include out or we improve detection if such cases become common requests.
	•	Artifacts that are files rather than directories: e.g., a single binary output in a Go project’s root. We currently don’t plan to delete individual files (except perhaps .pyc inside pycache). So some stray binaries or .apk from Flutter build might remain if they are not inside a folder. For instance, Flutter build outputs go into build/, but the final APK might be in build/app/outputs/apk/..., which will be gone if build/ is gone. So fine. But another scenario: a C project that compiles to a.out in the project root – we wouldn’t delete a.out because we didn’t identify a build directory. This is minor (usually not huge, and maybe user doesn’t want it deleted automatically anyway).
	•	Conclusion: Missing some files is not critical from a safety standpoint (just leaves some space used). We focus on directories where the bulk of size is.
	•	False Positives (Accidental Deletions): The most critical edge case to avoid. Could the tool ever delete something it shouldn’t?
	•	We’ve put heuristics to avoid that (config + known folder). But consider a scenario: a user has a directory structure where node_modules is used for something else, not Node dependencies (rare, but suppose someone named a folder “node_modules” as a joke or for some other data). If there’s no package.json, our tool will not delete it (since config missing). If they did have a package.json but still stored something important in node_modules (which would be an odd usage), our tool would consider it safe to wipe. One hopes no one misuses names that badly. Similarly, if someone put important files inside a build folder manually, and also has a build.gradle, we’d remove it. Generally, by convention, developers know not to put important long-term files in these artifact dirs, but it’s a theoretical risk.
	•	Mitigation: Documentation warning – we will state “do not store irreplaceable files in directories like node_modules or target, because those are meant to be ephemeral; if you do, exclude them explicitly.” And our confirmation prompt/dry-run gives a chance to notice if something looks off.
	•	Also, interactive mode allows one to skip a particular project if unsure.
	•	We might also implement a safety rule: never delete a directory that contains a .git subfolder (meaning maybe it’s a sub-repo) or some obvious user file. For example, if someone bizarrely checked in a node_modules with their own code – then it wouldn’t be in .gitignore. Perhaps we could check: if a directory to be deleted contains a VCS folder or certain markers (like a README), pause. However, this complicates matters and likely isn’t needed if people follow normal project structures.
	•	Permission Issues: If some files are not deletable (root-owned files, etc.), the tool will skip them with error messages. In most dev scenarios, this shouldn’t happen (user builds typically produce files owned by the user). But if they used sudo for something and left root-owned files in a build dir, our tool might not remove those. We will inform the user to manually adjust or re-run with sudo (though running the whole tool with sudo is dangerous; better to change ownership or exclude).
	•	The tool should never try to elevate permissions by itself. Just report and continue.
	•	Large Directory Handling: There may be edge cases with extremely large directories (millions of files). For example, a pathological node_modules or a deeply nested target. Our deletion using remove_dir_all should handle it but might take time. If it’s very slow, the user might think it hung. We mitigate by showing progress/spinner during deletion if possible. Also, ensure our size calculation doesn’t overflow 64-bit if extremely large (unlikely, but who knows with node_modules 😄).
	•	If a directory is too deep (exceeding OS path length limits), remove_dir_all might fail. This can happen on Windows typically; on Linux/Mac the limits are high, but node_modules with deeply nested dependencies on Windows often hit issues. We are not on Windows, so probably fine. On Mac/Linux, we could still hit the 255 char filename limit or 1024 char path. Unlikely but we could recursively handle those (maybe have to shorten path or let OS handle it).
	•	Simultaneous Runs and Race Conditions: If a user (or two different processes) run the tool on overlapping directories, or if the user triggers a build while the cleaner is running, weird things could happen (like deleting as new files appear). This is an edge case; we assume user will not compile stuff at the same time as cleaning it. If it happens, worst case some files might fail to delete (busy) or some new files remain after cleaning (no big deal). We do not attempt any synchronization with external processes, that’s out of scope.
	•	Monorepo complexities: Some monorepos might have nested projects where one project’s artifact directory is another’s source. Unlikely, but if so, we might double-list or conflict. Example: a repo that contains multiple packages in subdirs – we will list each. If one package’s build dir is inside another’s directory, and the user chooses to only delete one, the other’s perspective might break. However, typically build dirs are within the project that created them, not shared.
	•	Post-clean state: After cleaning, a user will need to rebuild to get those artifacts back. This could surface issues if the build process isn’t fully reproducible (e.g., maybe some files were generated once and not tracked anywhere). That’s a user project issue, not our tool’s fault, but it’s worth noting that cleaning should ideally be done when you’re not in the middle of work or you have the ability to rebuild easily. Our documentation can advise: “Only clean when you know you can regenerate, e.g., don’t clean a virtualenv environment if you don’t have requirements to recreate it.”
	•	Trash vs Permanent Delete: Currently we permanently delete. If a user regrets cleaning, there’s no easy way back except re-building/downloading. Perhaps as a limitation, we note that the tool is not a backup, it assumes these artifacts are disposable. We might consider an option to move to trash as mentioned, but not required.
	•	Security Consideration: We should be mindful that deleting files could be dangerous if misused, but that’s true of any such tool. We don’t operate on untrusted input or produce network calls, so security concerns are minimal. Just ensure we don’t allow something like --include "/" to wipe the root (and if someone did that with --yes, well, they essentially invoked an rm -rf / which any tool can’t fully prevent if user insists).

In summary, most limitations result in either not deleting some junk (which is safe but maybe less thorough), or requiring user attention in unusual cases. The highest priority is to avoid the opposite (deleting something non-junk), which our design strives to prevent. We will document known limitations and possibly add improvements in future updates as we gather user feedback on edge cases.
