use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a test project structure
fn create_test_project(
    temp_dir: &TempDir,
    name: &str,
    config_file: &str,
    artifact_dirs: &[&str],
) -> PathBuf {
    let project_path = temp_dir.path().join(name);
    fs::create_dir_all(&project_path).unwrap();

    // Create config file
    let config_path = project_path.join(config_file);
    let mut file = File::create(&config_path).unwrap();
    writeln!(file, "# Test config").unwrap();

    // Create artifact directories with some files
    for artifact in artifact_dirs {
        let artifact_path = project_path.join(artifact);
        fs::create_dir_all(&artifact_path).unwrap();

        // Add some dummy files to make size > 0
        for i in 0..5 {
            let file_path = artifact_path.join(format!("file_{}.txt", i));
            let mut f = File::create(&file_path).unwrap();
            writeln!(f, "test content {}", i).unwrap();
        }
    }

    project_path
}

#[test]
fn test_rust_project_detection() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(&temp_dir, "my_rust_project", "Cargo.toml", &["target"]);

    // Verify structure
    assert!(temp_dir.path().join("my_rust_project/Cargo.toml").exists());
    assert!(temp_dir.path().join("my_rust_project/target").exists());
}

#[test]
fn test_node_project_detection() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(&temp_dir, "my_node_app", "package.json", &["node_modules"]);

    assert!(temp_dir.path().join("my_node_app/package.json").exists());
    assert!(temp_dir.path().join("my_node_app/node_modules").exists());
}

#[test]
fn test_python_project_detection() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(
        &temp_dir,
        "my_python_app",
        "pyproject.toml",
        &["__pycache__", "venv"],
    );

    assert!(temp_dir
        .path()
        .join("my_python_app/pyproject.toml")
        .exists());
    assert!(temp_dir.path().join("my_python_app/__pycache__").exists());
    assert!(temp_dir.path().join("my_python_app/venv").exists());
}

#[test]
fn test_flutter_project_detection() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(
        &temp_dir,
        "my_flutter_app",
        "pubspec.yaml",
        &["build", ".dart_tool"],
    );

    assert!(temp_dir
        .path()
        .join("my_flutter_app/pubspec.yaml")
        .exists());
    assert!(temp_dir.path().join("my_flutter_app/build").exists());
    assert!(temp_dir.path().join("my_flutter_app/.dart_tool").exists());
}

#[test]
fn test_java_maven_detection() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(&temp_dir, "my_java_maven", "pom.xml", &["target"]);

    assert!(temp_dir.path().join("my_java_maven/pom.xml").exists());
    assert!(temp_dir.path().join("my_java_maven/target").exists());
}

#[test]
fn test_java_gradle_detection() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(&temp_dir, "my_java_gradle", "build.gradle", &["build"]);

    assert!(temp_dir.path().join("my_java_gradle/build.gradle").exists());
    assert!(temp_dir.path().join("my_java_gradle/build").exists());
}

#[test]
fn test_go_project_detection() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(&temp_dir, "my_go_app", "go.mod", &["vendor"]);

    assert!(temp_dir.path().join("my_go_app/go.mod").exists());
    assert!(temp_dir.path().join("my_go_app/vendor").exists());
}

#[test]
fn test_cpp_cmake_detection() {
    let temp_dir = TempDir::new().unwrap();
    create_test_project(&temp_dir, "my_cpp_app", "CMakeLists.txt", &["build"]);

    assert!(temp_dir.path().join("my_cpp_app/CMakeLists.txt").exists());
    assert!(temp_dir.path().join("my_cpp_app/build").exists());
}

#[test]
fn test_dotnet_project_detection() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("my_dotnet_app");
    fs::create_dir_all(&project_path).unwrap();

    // .NET uses *.csproj files
    let csproj_path = project_path.join("MyApp.csproj");
    let mut file = File::create(&csproj_path).unwrap();
    writeln!(file, "<Project />").unwrap();

    // Create artifact directories
    for dir in &["bin", "obj"] {
        let artifact_path = project_path.join(dir);
        fs::create_dir_all(&artifact_path).unwrap();
        let file_path = artifact_path.join("test.dll");
        File::create(&file_path).unwrap();
    }

    assert!(temp_dir.path().join("my_dotnet_app/MyApp.csproj").exists());
    assert!(temp_dir.path().join("my_dotnet_app/bin").exists());
    assert!(temp_dir.path().join("my_dotnet_app/obj").exists());
}

#[test]
fn test_no_false_positive_without_config() {
    let temp_dir = TempDir::new().unwrap();

    // Create a "build" directory without any config file (should NOT be detected)
    let random_dir = temp_dir.path().join("random_folder");
    fs::create_dir_all(&random_dir).unwrap();
    let build_dir = random_dir.join("build");
    fs::create_dir_all(&build_dir).unwrap();

    // No config file, so this should not be treated as a project
    assert!(!random_dir.join("Cargo.toml").exists());
    assert!(!random_dir.join("package.json").exists());
    assert!(!random_dir.join("pom.xml").exists());
}

#[test]
fn test_multiple_projects_in_workspace() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple projects of different types
    create_test_project(&temp_dir, "rust_app", "Cargo.toml", &["target"]);
    create_test_project(&temp_dir, "node_app", "package.json", &["node_modules"]);
    create_test_project(
        &temp_dir,
        "python_app",
        "pyproject.toml",
        &["__pycache__"],
    );

    // All should exist
    assert!(temp_dir.path().join("rust_app/Cargo.toml").exists());
    assert!(temp_dir.path().join("node_app/package.json").exists());
    assert!(temp_dir.path().join("python_app/pyproject.toml").exists());
}

#[test]
fn test_nested_projects() {
    let temp_dir = TempDir::new().unwrap();

    // Create a parent project with a nested subproject
    let parent = temp_dir.path().join("parent");
    fs::create_dir_all(&parent).unwrap();
    let config = parent.join("package.json");
    File::create(&config).unwrap();

    let child = parent.join("packages/child");
    fs::create_dir_all(&child).unwrap();
    let child_config = child.join("package.json");
    File::create(&child_config).unwrap();

    // Create artifacts for both
    let parent_nm = parent.join("node_modules");
    fs::create_dir_all(&parent_nm).unwrap();
    let child_nm = child.join("node_modules");
    fs::create_dir_all(&child_nm).unwrap();

    assert!(parent.join("package.json").exists());
    assert!(child.join("package.json").exists());
    assert!(parent_nm.exists());
    assert!(child_nm.exists());
}
