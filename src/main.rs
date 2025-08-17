mod detector;

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


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let project_type = detector::detect_project_type(&current_dir);

            println!("Project detected: {:?}", project_type);
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
}