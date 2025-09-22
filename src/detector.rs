// src/detector.rs
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum ProjectType {
    Node { entry_point: String },
    Python { entry_point: String }, // <-- MODIFIED
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

fn detect_python_entry_point(path: &Path) -> String {
    for filename in ["main.py", "app.py"] {
        if path.join(filename).exists() {
            println!("Found entry point by filename: {}", filename);
            return filename.to_string();
        }
    }
    "main.py".to_string()
}

pub fn detect_project_type(path: &Path, entry_point_override: Option<&str>) -> anyhow::Result<ProjectType> {
    
    let check_override = |override_file: &str| -> anyhow::Result<String> {
        if path.join(override_file).exists() {
            println!("Using user-specified entry point: {}", override_file);
            Ok(override_file.to_string())
        } else {
            anyhow::bail!("Specified entry point '{}' does not exist.", override_file)
        }
    };

    if path.join("package.json").exists() {
        let entry_point = match entry_point_override {
            Some(file) => check_override(file)?,
            None => detect_node_entry_point(path),
        };
        Ok(ProjectType::Node { entry_point })

    } else if path.join("requirements.txt").exists() || path.join("pyproject.toml").exists() {
        let entry_point = match entry_point_override {
            Some(file) => check_override(file)?,
            None => detect_python_entry_point(path),
        };
        Ok(ProjectType::Python { entry_point }) // <-- MODIFIED

    } else if path.join("Cargo.toml").exists() {
        if entry_point_override.is_some() {
            println!("Warning: --entry-point is ignored for Rust projects.");
        }
        Ok(ProjectType::Rust)

    } else {
        if entry_point_override.is_some() {
             anyhow::bail!("Cannot determine project type. --entry-point was provided, but no package.json, requirements.txt, or Cargo.toml was found.");
        }
        Ok(ProjectType::Unknown)
    }
}