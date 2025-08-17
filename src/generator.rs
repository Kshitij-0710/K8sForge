// src/generator.rs
use tera::{Context, Tera};
use std::fs;
use crate::detector::ProjectType;

pub fn generate_dockerfile(project_type: &ProjectType) -> anyhow::Result<()> {
    let tera = Tera::new("templates/**/*")?;

    let mut context = Context::new();
    context.insert("port", &3000);

    let template_name = match project_type {
        ProjectType::Node => "Dockerfile.node.tpl",
        _ => anyhow::bail!("Unsupported project type for Dockerfile generation"),
    };

    let dockerfile_content = tera.render(template_name, &context)?;

    fs::write("Dockerfile", dockerfile_content)?;

    println!("✅ Successfully created Dockerfile!");
    Ok(())
}