use crate::detector::ProjectType;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use tera::{Context, Tera};

#[derive(Serialize)]
struct Compose {
    version: String,
    services: BTreeMap<String, Service>,
}

#[derive(Serialize)]
struct Service {
    build: String,
    ports: Vec<String>,
}

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

pub fn generate_compose_file(service_name: &str, port: u16) -> anyhow::Result<()> {
    let mut services = BTreeMap::new();
    services.insert(
        service_name.to_string(),
        Service {
            build: ".".to_string(),
            ports: vec![format!("{}:{}", port, port)],
        },
    );

    let compose_config = Compose {
        version: "3.8".to_string(),
        services,
    };

    let yaml_content = serde_yaml::to_string(&compose_config)?;
    fs::write("docker-compose.yml", yaml_content)?;

    println!("✅ Successfully created docker-compose.yml!");
    Ok(())
}