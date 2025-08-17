mod detector;
mod generator;
mod docker;
use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Init {
        /// The port your application exposes
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Build and start the services in the background
    Up,
    /// Stop and remove the services
    Down,
    /// View logs from the services
    Logs,
}

// The return type is changed here
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { port } => {
            let current_dir = env::current_dir()?;
            let project_type = detector::detect_project_type(&current_dir);

            let service_name = current_dir
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("my-app");

            println!("Project detected: {:?}", project_type);

            generator::generate_dockerfile(&project_type, *port)?;
            generator::generate_compose_file(service_name, *port)?;
        }
         Commands::Up => {
            docker::up()?;
        }
        Commands::Down => {
            docker::down()?;
        }
        Commands::Logs => {
            docker::logs()?;
        }
    }

    // Signal that everything finished successfully
    Ok(())
}