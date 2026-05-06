# K8sForge

K8sForge is a small Rust CLI for bootstrapping container support in an existing project and managing the resulting services with Docker Compose.

It currently detects common project types, generates Docker files for supported apps, and exposes a few convenience commands for starting, stopping, and following container logs.

## Features

- Detects the project type in the current directory.
- Generates a `Dockerfile` and `docker-compose.yml` for Node.js projects.
- Uses the current directory name as the Compose service name.
- Wraps `docker-compose up`, `down`, and `logs` behind a single CLI.
- Lets you choose the exposed port when generating config files.

## Requirements

- Rust toolchain for building the CLI.
- Docker installed and running.
- `docker-compose` available on your `PATH`.

## Installation

Build from source with Cargo:

```bash
cargo build --release
```

Run it directly during development:

```bash
cargo run -- <command>
```

If you install the binary, the package name is `dock-rs`.

## Usage

All commands operate on the current working directory, so run them from the project you want to containerize.

### Generate Docker files

```bash
dock-rs init --port 3000
```

or, during development:

```bash
cargo run -- init --port 3000
```

This command:

- Detects the project type from the current directory.
- Looks for a Node entry point in `package.json`, then falls back to `server.js`, `app.js`, `index.js`, and finally `index.js`.
- Generates a `Dockerfile` from the embedded Node template.
- Generates a `docker-compose.yml` using the current directory name as the service key.

### Start services

```bash
dock-rs up
```

This runs `docker-compose up --build -d`.

### Stop services

```bash
dock-rs down
```

This runs `docker-compose down`.

### Follow logs

```bash
dock-rs logs
```

This runs `docker-compose logs --follow`.

## What Gets Generated

### Dockerfile

For Node.js projects, K8sForge renders `templates/Dockerfile.node.tpl` into a production-style multi-stage image:

- Builds on `node:20-alpine`.
- Installs dependencies with `npm install`.
- Copies the application into the final image.
- Exposes the port you pass to `init`.
- Starts the app with `node <entry_point>`.

### docker-compose.yml

The generated Compose file uses:

- `version: "3.8"`
- One service named after the current directory.
- A bind from `<port>:<port>`.
- A build context of `.`.

## Supported Project Detection

The CLI currently detects these project types:

- Node.js, when `package.json` exists.
- Python, when `requirements.txt` or `pyproject.toml` exists.
- Rust, when `Cargo.toml` exists.
- Unknown, when none of the above are present.

At the moment, Dockerfile generation is implemented only for Node.js projects. If you run `init` in a Python, Rust, or unknown project, generation will fail with an unsupported project type error.

## Example Workflow

```bash
cd my-node-app
dock-rs init --port 8000
dock-rs up
dock-rs logs
dock-rs down
```

## Notes

- The CLI uses the `docker-compose` executable, not `docker compose`.
- The generated `Dockerfile` and `docker-compose.yml` are written to the current directory and will overwrite files with the same names.
- The default port for `init` is `3000`.

## Project Layout

```text
Cargo.toml
Dockerfile
docker-compose.yml
package.json
README.md
src/
	detector.rs
	docker.rs
	generator.rs
	main.rs
templates/
	Dockerfile.node.tpl
```

## Development

Useful commands while iterating on the CLI:

```bash
cargo fmt
cargo check
cargo run -- init --port 3000
```

## Future Work

Likely next steps for the project are support for generating Dockerfiles for Python and Rust projects, plus a migration from `docker-compose` to the modern `docker compose` CLI where available.
