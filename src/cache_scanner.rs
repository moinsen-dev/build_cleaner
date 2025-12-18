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

    #[test]
    fn test_cache_scanner_creation() {
        let scanner = CacheScanner::new();
        assert!(scanner.is_some());
    }

    #[test]
    fn test_category_filter() {
        let scanner = CacheScanner::new()
            .unwrap()
            .with_categories(vec![CacheCategory::AiMl]);

        // Just verify it doesn't panic
        let _ = scanner.scan();
    }
}
