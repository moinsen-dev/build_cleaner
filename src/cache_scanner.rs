use crate::cache::{Cache, CacheCategory, CacheType};
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Scanner for user-level caches
pub struct CacheScanner {
    home_dir: PathBuf,
    categories: Option<Vec<CacheCategory>>,
    min_size: Option<u64>,
}

impl CacheScanner {
    /// Create a new cache scanner
    pub fn new() -> Option<Self> {
        dirs::home_dir().map(|home_dir| Self {
            home_dir,
            categories: None,
            min_size: None,
        })
    }

    /// Filter by cache categories
    pub fn with_categories(mut self, categories: Vec<CacheCategory>) -> Self {
        if !categories.is_empty() {
            self.categories = Some(categories);
        }
        self
    }

    /// Filter by minimum size
    pub fn with_min_size(mut self, min_size: Option<u64>) -> Self {
        self.min_size = min_size;
        self
    }

    /// Scan for user caches
    pub fn scan(&self) -> Vec<Cache> {
        let is_macos = cfg!(target_os = "macos");

        // Get all cache types, filtered by category if specified
        let cache_types: Vec<CacheType> = CacheType::all()
            .iter()
            .filter(|ct| {
                self.categories
                    .as_ref()
                    .map(|cats| cats.contains(&ct.category()))
                    .unwrap_or(true)
            })
            .copied()
            .collect();

        // Scan in parallel
        let caches: Vec<Cache> = cache_types
            .par_iter()
            .flat_map(|cache_type| {
                cache_type
                    .paths()
                    .iter()
                    .filter_map(|(rel_path, macos_only)| {
                        // Skip macOS-only paths on other platforms
                        if *macos_only && !is_macos {
                            return None;
                        }

                        let path = self.home_dir.join(rel_path);
                        if path.exists() {
                            let size = calculate_dir_size(&path);

                            // Apply minimum size filter
                            if let Some(min) = self.min_size {
                                if size < min {
                                    return None;
                                }
                            }

                            Some(Cache::new(*cache_type, path, size))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // Sort by size (largest first)
        let mut sorted = caches;
        sorted.sort_by(|a, b| b.size.cmp(&a.size));
        sorted
    }
}

impl Default for CacheScanner {
    fn default() -> Self {
        Self::new().expect("Could not determine home directory")
    }
}

/// Calculate the total size of a directory
fn calculate_dir_size(path: &PathBuf) -> u64 {
    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::CacheType;

    #[test]
    fn test_cache_scanner_creation() {
        let scanner = CacheScanner::new();
        assert!(scanner.is_some());
    }

    #[test]
    fn test_category_filter_ai_ml() {
        let scanner = CacheScanner::new()
            .unwrap()
            .with_categories(vec![CacheCategory::AiMl]);
        // Just verify it doesn't panic and returns a vec
        let result = scanner.scan();
        assert!(result.iter().all(|c| c.cache_type.category() == CacheCategory::AiMl));
    }

    #[test]
    fn test_category_filter_package_manager() {
        let scanner = CacheScanner::new()
            .unwrap()
            .with_categories(vec![CacheCategory::PackageManager]);
        let result = scanner.scan();
        assert!(result.iter().all(|c| c.cache_type.category() == CacheCategory::PackageManager));
    }

    #[test]
    fn test_empty_categories_returns_all() {
        // with_categories on an empty vec leaves categories as None → scan all
        let scanner_all = CacheScanner::new().unwrap();
        let scanner_empty = CacheScanner::new()
            .unwrap()
            .with_categories(vec![]);
        // Both should produce the same result since empty list means no filter
        let all = scanner_all.scan();
        let empty_filtered = scanner_empty.scan();
        assert_eq!(all.len(), empty_filtered.len());
    }

    #[test]
    fn test_min_size_filter_excludes_small_caches() {
        // With an absurdly large minimum, nothing should pass
        let scanner = CacheScanner::new()
            .unwrap()
            .with_min_size(Some(u64::MAX));
        let result = scanner.scan();
        assert!(result.is_empty(), "no cache should exceed u64::MAX bytes");
    }

    #[test]
    fn test_min_size_none_includes_all_existing() {
        let scanner_no_filter = CacheScanner::new().unwrap().with_min_size(None);
        let scanner_zero = CacheScanner::new().unwrap().with_min_size(Some(0));
        // Both should return the same caches (0-byte minimum = same as no filter)
        let no_filter = scanner_no_filter.scan();
        let zero_min = scanner_zero.scan();
        assert_eq!(no_filter.len(), zero_min.len());
    }

    #[test]
    fn test_results_sorted_largest_first() {
        let scanner = CacheScanner::new().unwrap();
        let result = scanner.scan();
        for window in result.windows(2) {
            assert!(
                window[0].size >= window[1].size,
                "results should be sorted largest first: {} >= {}",
                window[0].size,
                window[1].size
            );
        }
    }

    #[test]
    fn test_calculate_dir_size_empty_dir() {
        let tmp = tempfile::TempDir::new().unwrap();
        let size = calculate_dir_size(&tmp.path().to_path_buf());
        assert_eq!(size, 0);
    }

    #[test]
    fn test_calculate_dir_size_with_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        // Write two files of known size
        std::fs::write(tmp.path().join("a.txt"), b"hello").unwrap(); // 5 bytes
        std::fs::write(tmp.path().join("b.txt"), b"world!").unwrap(); // 6 bytes
        let size = calculate_dir_size(&tmp.path().to_path_buf());
        assert_eq!(size, 11);
    }

    #[test]
    fn test_calculate_dir_size_nested() {
        let tmp = tempfile::TempDir::new().unwrap();
        let sub = tmp.path().join("sub");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(sub.join("file.bin"), vec![0u8; 100]).unwrap();
        let size = calculate_dir_size(&tmp.path().to_path_buf());
        assert_eq!(size, 100);
    }

    #[test]
    fn test_cache_type_paths_are_relative() {
        // All non-macOS paths should not start with '/'
        for ct in CacheType::all() {
            for (path, _macos_only) in ct.paths() {
                assert!(
                    !path.starts_with('/'),
                    "cache path should be relative to home: {} for {:?}",
                    path,
                    ct
                );
            }
        }
    }

    #[test]
    fn test_all_cache_types_have_at_least_one_path() {
        for ct in CacheType::all() {
            assert!(
                !ct.paths().is_empty(),
                "cache type {:?} has no paths",
                ct
            );
        }
    }

    #[test]
    fn test_cache_type_category_is_consistent() {
        use crate::cache::CacheCategory;
        // Spot-check a few well-known mappings
        assert_eq!(CacheType::HuggingFace.category(), CacheCategory::AiMl);
        assert_eq!(CacheType::Npm.category(), CacheCategory::PackageManager);
        assert_eq!(CacheType::XcodeDerivedData.category(), CacheCategory::Development);
        assert_eq!(CacheType::JetBrainsCache.category(), CacheCategory::Ide);
        assert_eq!(CacheType::Docker.category(), CacheCategory::Container);
    }
}
