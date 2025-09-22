mod config {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct DockerConfig<'a> {
        pub name: &'a str,  // LIFETIMES: 'a ensures this reference lives long enough
        pub image: String,  // Owned data - this struct owns the String
        pub ports: Vec<u16>,
    }

    #[derive(Debug)]
    pub enum Command {
        Start,    // Simple variants
        Stop,
        Restart,
    }

    // OWNERSHIP & BORROWING: Memory safety without garbage collection
    impl<'a> DockerConfig<'a> {
        pub fn new(name: &'a str, image: String) -> Self {
            Self {
                name,           // BORROWING: Just referencing, not owning
                image,          // OWNERSHIP: This struct now owns this String
                ports: Vec::new(),
            }
        }

        // Mutable borrow of self to modify it
        pub fn add_port(&mut self, port: u16) {
            self.ports.push(port);
        }
    }

    // RESULT ENUM: Instead of throwing exceptions, return Ok() or Err()
    pub fn execute_command(config: &DockerConfig, cmd: Command) -> Result<String, String> {
        // PATTERN MATCHING: Handle every possible case exhaustively
        match cmd {
            Command::Start => Ok(format!("Starting {} with image {}", config.name, config.image)),
            Command::Stop => Ok(format!("Stopping {}", config.name)),
            Command::Restart => {
                // Return error as value, not exception
                if config.ports.is_empty() {
                    Err("Cannot restart: no ports configured".to_string())
                } else {
                    Ok(format!("Restarting {} on ports {:?}", config.name, config.ports))
                }
            }
        }
    }
}

fn main() {
    use config::*; // Import everything from our config module

    // OWNERSHIP: Creating owned data and borrowing references
    let mut docker_config = DockerConfig::new("my-app", "nginx:latest".to_string());
    //                                        ^borrowed   ^owned String
    
    // BORROWING: Mutably borrow to modify the struct
    docker_config.add_port(80);
    docker_config.add_port(443);

    // ENUMS: Create different command variants
    let commands = vec![Command::Start, Command::Restart, Command::Stop];
    
    for cmd in commands {
        // PATTERN MATCHING: Handle Result enum safely - no exceptions!
        match execute_command(&docker_config, cmd) { // Immutable borrow
            Ok(msg) => println!("{}", msg),   // Success case
            Err(err) => println!("Error: {}", err), // Error as value
        }
    }
    
    // OWNERSHIP: docker_config is automatically dropped here (no manual memory mgmt)
}