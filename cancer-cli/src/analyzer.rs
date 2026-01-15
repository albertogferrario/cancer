//! Project structure analyzer for convention detection.
//!
//! Scans project directories to detect existing patterns and conventions,
//! enabling smart defaults for CLI commands like make:scaffold.

// Allow unused code warnings as this module is used by make_scaffold in subsequent tasks
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

/// Pattern for test file organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestPattern {
    /// No test files detected
    None,
    /// Tests organized per controller (e.g., user_controller_test.rs)
    PerController,
    /// Tests in a unified test file
    Unified,
}

/// Pattern for factory file organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactoryPattern {
    /// No factory files detected
    None,
    /// Factories organized per model (e.g., user_factory.rs)
    PerModel,
    /// Factories in a unified factory file
    Unified,
}

/// Detected project conventions and structure.
#[derive(Debug, Clone)]
pub struct ProjectConventions {
    /// Whether src/tests/ directory exists
    pub has_tests_dir: bool,
    /// Whether src/factories/ directory exists
    pub has_factories_dir: bool,
    /// Whether frontend/src/pages/ directory exists with content
    pub has_inertia_pages: bool,
    /// List of existing model names (from src/models/)
    pub existing_models: Vec<String>,
    /// Detected test file organization pattern
    pub test_pattern: TestPattern,
    /// Detected factory file organization pattern
    pub factory_pattern: FactoryPattern,
    /// Number of existing test files
    pub test_file_count: usize,
    /// Number of existing factory files
    pub factory_file_count: usize,
}

impl Default for ProjectConventions {
    fn default() -> Self {
        Self {
            has_tests_dir: false,
            has_factories_dir: false,
            has_inertia_pages: false,
            existing_models: Vec::new(),
            test_pattern: TestPattern::None,
            factory_pattern: FactoryPattern::None,
            test_file_count: 0,
            factory_file_count: 0,
        }
    }
}

/// Analyzer for detecting project conventions and patterns.
pub struct ProjectAnalyzer {
    root: PathBuf,
}

impl ProjectAnalyzer {
    /// Create a new analyzer for the given project root.
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Create an analyzer for the current working directory.
    pub fn current_dir() -> Self {
        Self::new(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    /// Analyze the project structure and return detected conventions.
    pub fn analyze(&self) -> ProjectConventions {
        let mut conventions = ProjectConventions::default();

        // Detect tests directory and pattern
        self.detect_tests(&mut conventions);

        // Detect factories directory and pattern
        self.detect_factories(&mut conventions);

        // Detect Inertia pages
        self.detect_inertia_pages(&mut conventions);

        // Detect existing models
        self.detect_models(&mut conventions);

        conventions
    }

    /// Detect test directory presence and pattern.
    fn detect_tests(&self, conventions: &mut ProjectConventions) {
        let tests_dir = self.root.join("src/tests");

        if !tests_dir.exists() || !tests_dir.is_dir() {
            return;
        }

        conventions.has_tests_dir = true;

        // Count test files and detect pattern
        let test_files = self.count_files_matching(&tests_dir, "_controller_test.rs");
        let unified_test = self.file_exists(&tests_dir, "tests.rs");

        conventions.test_file_count = test_files;

        if test_files > 0 {
            conventions.test_pattern = TestPattern::PerController;
        } else if unified_test {
            conventions.test_pattern = TestPattern::Unified;
        }
    }

    /// Detect factories directory presence and pattern.
    fn detect_factories(&self, conventions: &mut ProjectConventions) {
        let factories_dir = self.root.join("src/factories");

        if !factories_dir.exists() || !factories_dir.is_dir() {
            return;
        }

        conventions.has_factories_dir = true;

        // Count factory files and detect pattern
        let factory_files = self.count_files_matching(&factories_dir, "_factory.rs");
        let unified_factory = self.file_exists(&factories_dir, "factory.rs");

        conventions.factory_file_count = factory_files;

        if factory_files > 0 {
            conventions.factory_pattern = FactoryPattern::PerModel;
        } else if unified_factory {
            conventions.factory_pattern = FactoryPattern::Unified;
        }
    }

    /// Detect Inertia pages directory presence and content.
    fn detect_inertia_pages(&self, conventions: &mut ProjectConventions) {
        let pages_dir = self.root.join("frontend/src/pages");

        if !pages_dir.exists() || !pages_dir.is_dir() {
            return;
        }

        // Check if there are any .tsx files (indicating Inertia pages)
        if let Ok(entries) = fs::read_dir(&pages_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Check for either .tsx files or subdirectories with .tsx files
                if path.is_dir() {
                    if self.has_tsx_files(&path) {
                        conventions.has_inertia_pages = true;
                        return;
                    }
                } else if path.extension().is_some_and(|ext| ext == "tsx") {
                    conventions.has_inertia_pages = true;
                    return;
                }
            }
        }
    }

    /// Detect existing models from src/models/ directory.
    fn detect_models(&self, conventions: &mut ProjectConventions) {
        let models_dir = self.root.join("src/models");

        if !models_dir.exists() || !models_dir.is_dir() {
            return;
        }

        if let Ok(entries) = fs::read_dir(&models_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_stem() {
                        let name_str = name.to_string_lossy().to_string();
                        // Skip mod.rs
                        if name_str != "mod" {
                            conventions.existing_models.push(name_str);
                        }
                    }
                }
            }
        }

        conventions.existing_models.sort();
    }

    /// Count files in a directory matching a suffix pattern.
    fn count_files_matching(&self, dir: &Path, suffix: &str) -> usize {
        let Ok(entries) = fs::read_dir(dir) else {
            return 0;
        };

        entries
            .filter_map(Result::ok)
            .filter(|e| {
                e.path().is_file()
                    && e.path()
                        .file_name()
                        .is_some_and(|n| n.to_string_lossy().ends_with(suffix))
            })
            .count()
    }

    /// Check if a specific file exists in a directory.
    fn file_exists(&self, dir: &Path, filename: &str) -> bool {
        dir.join(filename).exists()
    }

    /// Check if a directory contains any .tsx files.
    fn has_tsx_files(&self, dir: &Path) -> bool {
        let Ok(entries) = fs::read_dir(dir) else {
            return false;
        };

        entries.filter_map(Result::ok).any(|e| {
            let path = e.path();
            path.is_file() && path.extension().is_some_and(|ext| ext == "tsx")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_project() -> TempDir {
        TempDir::new().unwrap()
    }

    #[test]
    fn test_analyzer_detects_empty_project() {
        let temp = create_test_project();
        let analyzer = ProjectAnalyzer::new(temp.path().to_path_buf());
        let conventions = analyzer.analyze();

        assert!(!conventions.has_tests_dir);
        assert!(!conventions.has_factories_dir);
        assert!(!conventions.has_inertia_pages);
        assert!(conventions.existing_models.is_empty());
        assert_eq!(conventions.test_pattern, TestPattern::None);
        assert_eq!(conventions.factory_pattern, FactoryPattern::None);
    }

    #[test]
    fn test_analyzer_detects_tests_directory() {
        let temp = create_test_project();
        let tests_dir = temp.path().join("src/tests");
        fs::create_dir_all(&tests_dir).unwrap();
        fs::write(tests_dir.join("user_controller_test.rs"), "").unwrap();
        fs::write(tests_dir.join("post_controller_test.rs"), "").unwrap();

        let analyzer = ProjectAnalyzer::new(temp.path().to_path_buf());
        let conventions = analyzer.analyze();

        assert!(conventions.has_tests_dir);
        assert_eq!(conventions.test_pattern, TestPattern::PerController);
        assert_eq!(conventions.test_file_count, 2);
    }

    #[test]
    fn test_analyzer_detects_factories_directory() {
        let temp = create_test_project();
        let factories_dir = temp.path().join("src/factories");
        fs::create_dir_all(&factories_dir).unwrap();
        fs::write(factories_dir.join("user_factory.rs"), "").unwrap();
        fs::write(factories_dir.join("mod.rs"), "").unwrap();

        let analyzer = ProjectAnalyzer::new(temp.path().to_path_buf());
        let conventions = analyzer.analyze();

        assert!(conventions.has_factories_dir);
        assert_eq!(conventions.factory_pattern, FactoryPattern::PerModel);
        assert_eq!(conventions.factory_file_count, 1);
    }

    #[test]
    fn test_analyzer_detects_inertia_pages() {
        let temp = create_test_project();
        let pages_dir = temp.path().join("frontend/src/pages/users");
        fs::create_dir_all(&pages_dir).unwrap();
        fs::write(pages_dir.join("Index.tsx"), "").unwrap();

        let analyzer = ProjectAnalyzer::new(temp.path().to_path_buf());
        let conventions = analyzer.analyze();

        assert!(conventions.has_inertia_pages);
    }

    #[test]
    fn test_analyzer_detects_models() {
        let temp = create_test_project();
        let models_dir = temp.path().join("src/models");
        fs::create_dir_all(&models_dir).unwrap();
        fs::write(models_dir.join("user.rs"), "").unwrap();
        fs::write(models_dir.join("post.rs"), "").unwrap();
        fs::write(models_dir.join("mod.rs"), "").unwrap();

        let analyzer = ProjectAnalyzer::new(temp.path().to_path_buf());
        let conventions = analyzer.analyze();

        assert_eq!(conventions.existing_models, vec!["post", "user"]);
    }
}
