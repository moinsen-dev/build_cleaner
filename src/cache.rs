use std::fmt;
use std::path::PathBuf;

/// Categories for grouping caches in output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheCategory {
    /// AI/ML model caches (Hugging Face, Ollama, PyTorch)
    AiMl,
    /// Package manager caches (npm, yarn, cargo, pip, etc.)
    PackageManager,
    /// Development tool caches (Xcode, Android SDK)
    Development,
    /// IDE caches (JetBrains, VSCode)
    Ide,
    /// Container caches (Docker) - requires special commands
    Container,
}

impl CacheCategory {
    /// Get the icon for this category
    pub fn icon(&self) -> &'static str {
        match self {
            CacheCategory::AiMl => "\u{1F916}",         // Robot
            CacheCategory::PackageManager => "\u{1F4E6}", // Package
            CacheCategory::Development => "\u{1F6E0}",  // Hammer and wrench
            CacheCategory::Ide => "\u{1F4BB}",          // Laptop
            CacheCategory::Container => "\u{1F40B}",    // Whale (Docker)
        }
    }

    /// Get the label for this category
    pub fn label(&self) -> &'static str {
        match self {
            CacheCategory::AiMl => "AI/ML",
            CacheCategory::PackageManager => "Package managers",
            CacheCategory::Development => "Development",
            CacheCategory::Ide => "IDE",
            CacheCategory::Container => "Container",
        }
    }
}

impl fmt::Display for CacheCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.icon(), self.label())
    }
}

/// Supported user-level cache types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheType {
    // AI/ML - High Priority (often 10-100GB)
    HuggingFace,
    Ollama,
    PyTorch,
    LmStudio,

    // Package Managers - High Priority
    Npm,
    Yarn,
    Pnpm,
    CargoRegistry,
    CargoGit,
    GradleCache,
    GradleWrapper,
    MavenRepository,
    PubCache,
    Pip,
    CocoaPods,
    RubyGems,

    // Development - High Priority (macOS)
    XcodeDerivedData,
    XcodeDeviceSupport,
    XcodeArchives,

    // IDE - Medium Priority
    JetBrainsCache,
    VSCodeCache,
    VSCodeServer,

    // Container - Requires special command
    Docker,

    // Language Caches
    GoBuild,
    PreCommit,
}

impl CacheType {
    /// Get all cache types
    pub fn all() -> &'static [CacheType] {
        &[
            // AI/ML
            CacheType::HuggingFace,
            CacheType::Ollama,
            CacheType::PyTorch,
            CacheType::LmStudio,
            // Package Managers
            CacheType::Npm,
            CacheType::Yarn,
            CacheType::Pnpm,
            CacheType::CargoRegistry,
            CacheType::CargoGit,
            CacheType::GradleCache,
            CacheType::GradleWrapper,
            CacheType::MavenRepository,
            CacheType::PubCache,
            CacheType::Pip,
            CacheType::CocoaPods,
            CacheType::RubyGems,
            // Development
            CacheType::XcodeDerivedData,
            CacheType::XcodeDeviceSupport,
            CacheType::XcodeArchives,
            // IDE
            CacheType::JetBrainsCache,
            CacheType::VSCodeCache,
            CacheType::VSCodeServer,
            // Container
            CacheType::Docker,
            // Language
            CacheType::GoBuild,
            CacheType::PreCommit,
        ]
    }

    /// Get the path(s) to this cache relative to home directory
    /// Returns (path, is_macos_only)
    pub fn paths(&self) -> Vec<(&'static str, bool)> {
        match self {
            // AI/ML
            CacheType::HuggingFace => vec![(".cache/huggingface", false)],
            CacheType::Ollama => vec![(".ollama/models", false)],
            CacheType::PyTorch => vec![(".cache/torch", false)],
            CacheType::LmStudio => vec![(".cache/lm-studio", false)],

            // Package Managers
            CacheType::Npm => vec![(".npm", false)],
            CacheType::Yarn => vec![(".yarn/cache", false)],
            CacheType::Pnpm => vec![(".pnpm-store", false)],
            CacheType::CargoRegistry => vec![(".cargo/registry", false)],
            CacheType::CargoGit => vec![(".cargo/git", false)],
            CacheType::GradleCache => vec![(".gradle/caches", false)],
            CacheType::GradleWrapper => vec![(".gradle/wrapper", false)],
            CacheType::MavenRepository => vec![(".m2/repository", false)],
            CacheType::PubCache => vec![(".pub-cache", false)],
            CacheType::Pip => vec![(".cache/pip", false)],
            CacheType::CocoaPods => vec![(".cocoapods", false)],
            CacheType::RubyGems => vec![(".gem", false)],

            // Development (macOS only)
            CacheType::XcodeDerivedData => {
                vec![("Library/Developer/Xcode/DerivedData", true)]
            }
            CacheType::XcodeDeviceSupport => vec![
                ("Library/Developer/Xcode/iOS DeviceSupport", true),
                ("Library/Developer/Xcode/watchOS DeviceSupport", true),
                ("Library/Developer/Xcode/tvOS DeviceSupport", true),
            ],
            CacheType::XcodeArchives => vec![("Library/Developer/Xcode/Archives", true)],

            // IDE
            CacheType::JetBrainsCache => vec![
                ("Library/Caches/JetBrains", true),
                (".cache/JetBrains", false),
            ],
            CacheType::VSCodeCache => vec![
                ("Library/Caches/com.microsoft.VSCode", true),
                (".cache/vscode-cpptools", false),
            ],
            CacheType::VSCodeServer => vec![(".vscode-server", false)],

            // Container
            CacheType::Docker => vec![(".docker", false)],

            // Language Caches
            CacheType::GoBuild => vec![(".cache/go-build", false)],
            CacheType::PreCommit => vec![(".cache/pre-commit", false)],
        }
    }

    /// Get the icon for this cache type
    pub fn icon(&self) -> &'static str {
        match self {
            CacheType::HuggingFace => "\u{1F917}",  // Hugging face
            CacheType::Ollama => "\u{1F999}",       // Llama
            CacheType::PyTorch => "\u{1F525}",      // Fire
            CacheType::LmStudio => "\u{1F4AC}",     // Speech bubble

            CacheType::Npm => "\u{1F4E6}",          // Package
            CacheType::Yarn => "\u{1F9F6}",         // Ball of yarn
            CacheType::Pnpm => "\u{1F4E6}",         // Package
            CacheType::CargoRegistry => "\u{1F980}", // Crab
            CacheType::CargoGit => "\u{1F980}",     // Crab
            CacheType::GradleCache => "\u{1F418}",  // Elephant
            CacheType::GradleWrapper => "\u{1F418}", // Elephant
            CacheType::MavenRepository => "\u{2615}", // Coffee
            CacheType::PubCache => "\u{1F3AF}",     // Dart
            CacheType::Pip => "\u{1F40D}",          // Snake
            CacheType::CocoaPods => "\u{1F36B}",    // Chocolate (cocoa)
            CacheType::RubyGems => "\u{1F48E}",     // Gem

            CacheType::XcodeDerivedData => "\u{1F528}", // Hammer
            CacheType::XcodeDeviceSupport => "\u{1F4F1}", // Mobile phone
            CacheType::XcodeArchives => "\u{1F4E6}",    // Package

            CacheType::JetBrainsCache => "\u{1F9E0}", // Brain
            CacheType::VSCodeCache => "\u{1F4DD}",    // Memo
            CacheType::VSCodeServer => "\u{1F5A5}",   // Computer

            CacheType::Docker => "\u{1F40B}",       // Whale

            CacheType::GoBuild => "\u{1F439}",      // Hamster (gopher)
            CacheType::PreCommit => "\u{1F6A6}",    // Traffic light
        }
    }

    /// Get the display label for this cache type
    pub fn label(&self) -> &'static str {
        match self {
            CacheType::HuggingFace => "Hugging Face",
            CacheType::Ollama => "Ollama models",
            CacheType::PyTorch => "PyTorch hub",
            CacheType::LmStudio => "LM Studio",

            CacheType::Npm => "npm cache",
            CacheType::Yarn => "Yarn cache",
            CacheType::Pnpm => "pnpm store",
            CacheType::CargoRegistry => "Cargo registry",
            CacheType::CargoGit => "Cargo git deps",
            CacheType::GradleCache => "Gradle cache",
            CacheType::GradleWrapper => "Gradle wrapper",
            CacheType::MavenRepository => "Maven repository",
            CacheType::PubCache => "Pub cache",
            CacheType::Pip => "pip cache",
            CacheType::CocoaPods => "CocoaPods",
            CacheType::RubyGems => "RubyGems",

            CacheType::XcodeDerivedData => "Xcode DerivedData",
            CacheType::XcodeDeviceSupport => "Xcode Device Support",
            CacheType::XcodeArchives => "Xcode Archives",

            CacheType::JetBrainsCache => "JetBrains cache",
            CacheType::VSCodeCache => "VSCode cache",
            CacheType::VSCodeServer => "VSCode Server",

            CacheType::Docker => "Docker",

            CacheType::GoBuild => "Go build cache",
            CacheType::PreCommit => "pre-commit cache",
        }
    }

    /// Get a description of what this cache contains
    pub fn description(&self) -> &'static str {
        match self {
            CacheType::HuggingFace => "Downloaded transformer models and datasets",
            CacheType::Ollama => "Downloaded LLM model files",
            CacheType::PyTorch => "PyTorch hub model cache",
            CacheType::LmStudio => "LM Studio model files",

            CacheType::Npm => "npm package cache (re-downloads on install)",
            CacheType::Yarn => "Yarn package cache",
            CacheType::Pnpm => "pnpm content-addressable store",
            CacheType::CargoRegistry => "Crate registry cache (re-downloads on build)",
            CacheType::CargoGit => "Git dependency cache for Cargo",
            CacheType::GradleCache => "Gradle build and dependency cache",
            CacheType::GradleWrapper => "Gradle wrapper distributions",
            CacheType::MavenRepository => "Maven dependency cache",
            CacheType::PubCache => "Dart/Flutter package cache",
            CacheType::Pip => "pip package download cache",
            CacheType::CocoaPods => "CocoaPods spec and pod cache",
            CacheType::RubyGems => "Ruby gem installations",

            CacheType::XcodeDerivedData => "Xcode build intermediates and indexes",
            CacheType::XcodeDeviceSupport => "iOS/watchOS/tvOS device symbol files",
            CacheType::XcodeArchives => "Archived app builds",

            CacheType::JetBrainsCache => "IDE cache and indexes",
            CacheType::VSCodeCache => "VSCode extension and runtime cache",
            CacheType::VSCodeServer => "Remote SSH/WSL server files",

            CacheType::Docker => "Docker images, containers, volumes",

            CacheType::GoBuild => "Go compilation cache",
            CacheType::PreCommit => "pre-commit hook environments",
        }
    }

    /// Get the category for this cache type
    pub fn category(&self) -> CacheCategory {
        match self {
            CacheType::HuggingFace
            | CacheType::Ollama
            | CacheType::PyTorch
            | CacheType::LmStudio => CacheCategory::AiMl,

            CacheType::Npm
            | CacheType::Yarn
            | CacheType::Pnpm
            | CacheType::CargoRegistry
            | CacheType::CargoGit
            | CacheType::GradleCache
            | CacheType::GradleWrapper
            | CacheType::MavenRepository
            | CacheType::PubCache
            | CacheType::Pip
            | CacheType::CocoaPods
            | CacheType::RubyGems => CacheCategory::PackageManager,

            CacheType::XcodeDerivedData
            | CacheType::XcodeDeviceSupport
            | CacheType::XcodeArchives => CacheCategory::Development,

            CacheType::JetBrainsCache | CacheType::VSCodeCache | CacheType::VSCodeServer => {
                CacheCategory::Ide
            }

            CacheType::Docker => CacheCategory::Container,

            CacheType::GoBuild | CacheType::PreCommit => CacheCategory::PackageManager,
        }
    }

    /// Get the official cleanup command if special handling is needed
    /// Returns (command, description)
    pub fn cleanup_command(&self) -> Option<(&'static str, &'static str)> {
        match self {
            CacheType::Docker => Some((
                "docker system prune -a",
                "Remove all unused images, containers, and volumes",
            )),
            CacheType::Npm => Some(("npm cache clean --force", "Official npm cache cleanup")),
            CacheType::Yarn => Some(("yarn cache clean", "Official Yarn cache cleanup")),
            CacheType::Pnpm => Some(("pnpm store prune", "Remove unreferenced packages")),
            CacheType::Pip => Some(("pip cache purge", "Official pip cache cleanup")),
            CacheType::GradleCache => Some(("gradle --stop", "Stop Gradle daemons first")),
            CacheType::GoBuild => Some(("go clean -cache", "Official Go cache cleanup")),
            CacheType::CocoaPods => {
                Some(("pod cache clean --all", "Official CocoaPods cache cleanup"))
            }
            CacheType::RubyGems => Some(("gem cleanup", "Remove old gem versions")),
            _ => None,
        }
    }

    /// Check if this cache requires the app to be closed first
    pub fn requires_app_closed(&self) -> Option<&'static str> {
        match self {
            CacheType::JetBrainsCache => Some("Close all JetBrains IDEs first"),
            CacheType::VSCodeCache => Some("Close VSCode first"),
            CacheType::Docker => Some("Ensure no containers are running"),
            _ => None,
        }
    }
}

impl fmt::Display for CacheType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.icon(), self.label())
    }
}

/// Represents a discovered user cache
#[derive(Debug, Clone)]
pub struct Cache {
    /// Type of cache
    pub cache_type: CacheType,
    /// Full path to the cache directory
    pub path: PathBuf,
    /// Size in bytes
    pub size: u64,
}

impl Cache {
    /// Create a new cache entry
    pub fn new(cache_type: CacheType, path: PathBuf, size: u64) -> Self {
        Self {
            cache_type,
            path,
            size,
        }
    }

    /// Get human-readable size
    pub fn size_display(&self) -> String {
        bytesize::ByteSize(self.size).to_string()
    }

}
