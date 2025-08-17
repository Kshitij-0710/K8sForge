// src/detector.rs
use std::path::Path;

#[derive(Debug)] // Add this to allow printing the enum
pub enum ProjectType {
    Node,
    Python,
    Rust,
    Unknown,
}

pub fn detect_project_type(path: &Path) -> ProjectType {
    if path.join("package.json").exists() {
        ProjectType::Node
    } else if path.join("requirements.txt").exists() || path.join("pyproject.toml").exists() {
        ProjectType::Python
    } else if path.join("Cargo.toml").exists() {
        ProjectType::Rust
    } else {
        ProjectType::Unknown
    }
}