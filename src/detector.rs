use serde::Deserialize;
use std::fs;

use std::path::Path;

#[derive(Debug)]
pub enum ProjectType {
    Node { entry_point: String },
    Python,
    Rust,
    Unknown,
}

#[derive(Deserialize)]
struct PackageJson {
    main: Option<String>,
}

fn detect_node_entry_point(path: &Path) -> String {
    if let Ok(content) = fs::read_to_string(path.join("package.json")) {
        if let Ok(pkg) = serde_json::from_str::<PackageJson>(&content) {
            if let Some(main_file) = pkg.main {
                println!("Found entry point in package.json: {}", &main_file);
                return main_file;
            }
        }
    }

    for filename in ["server.js", "app.js", "index.js"] {
        if path.join(filename).exists() {
            println!("Found entry point by filename: {}", filename);
            return filename.to_string();
        }
    }

    "index.js".to_string()
}

pub fn detect_project_type(path: &Path) -> ProjectType {
    if path.join("package.json").exists() {
        let entry_point = detect_node_entry_point(path);
        ProjectType::Node { entry_point }
    } else if path.join("requirements.txt").exists() || path.join("pyproject.toml").exists() {
        ProjectType::Python
    } else if path.join("Cargo.toml").exists() {
        ProjectType::Rust
    } else {
        ProjectType::Unknown
    }
}