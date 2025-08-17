mod detector;
mod generator;

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
    /// Initialize Docker files in the current project
    Init,
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
        Commands::Init => {
            // Use '?' for cleaner error handling
            let current_dir = env::current_dir()?;
            let project_type = detector::detect_project_type(&current_dir);

            println!("Project detected: {:?}", project_type);
            generator::generate_dockerfile(&project_type)?;
            generator::generate_compose_file("my-node-app", 3000)?;
        }
        Commands::Up => {
            println!("Running the 'up' command...");
        }
        Commands::Down => {
            println!("Running the 'down' command...");
        }
        Commands::Logs => {
            println!("Running the 'logs' command...");
        }
    }

    // Signal that everything finished successfully
    Ok(())
}