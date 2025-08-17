use crate::detector::ProjectType;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use tera::{Context, Tera};
use include_dir::{include_dir, Dir};

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
static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn generate_dockerfile(project_type: &ProjectType, port: u16) -> anyhow::Result<()> {
    let mut tera = Tera::default();

    // Load the templates from the embedded directory
    for file in TEMPLATES_DIR.files() {
        let path_str = file.path().to_str().ok_or_else(|| anyhow::anyhow!("Invalid template path"))?;
        tera.add_raw_template(path_str, file.contents_utf8().unwrap())?;
    }

    let mut context = Context::new();
    context.insert("port", &port);

    // This is the line we are fixing
    let template_name = match project_type {
        ProjectType::Node { entry_point } => {
            context.insert("entry_point", entry_point);
            "Dockerfile.node.tpl" // REMOVED "templates/" PREFIX
        }
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