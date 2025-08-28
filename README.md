How to use it:
Navigate into your Node.js project directory.

Run the init command:

Bash

# It will auto-detect your project and generate the Docker files
docker-rust-cli init --port 8000 
Start your containers:

Bash

docker-rust-cli up
Check the logs or stop the services:

Bash

docker-rust-cli logs
docker-rust-cli down
In my next post, I'll share a live demo of the CLI in action. Thanks for following along!
