// src/docker.rs
use std::process::{Command, Stdio};

fn run_compose_command(args: &[&str]) -> anyhow::Result<()> {
    println!("Running: docker-compose {}", args.join(" "));

    let mut cmd = Command::new("docker-compose")
        .args(args)
        .stdout(Stdio::inherit()) // Show command output to the user in real-time
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = cmd.wait()?;
    if !status.success() {
        // The command failed, return an error
        anyhow::bail!("docker-compose command failed!");
    }
    Ok(())
}

pub fn up() -> anyhow::Result<()> {
    run_compose_command(&["up", "--build", "-d"])
}
pub fn down() -> anyhow::Result<()> {
    run_compose_command(&["down"])
}

pub fn logs() -> anyhow::Result<()> {
    run_compose_command(&["logs", "--follow"])
}