# Docker Compose: Build, Run, and Attach to a Container

Follow these steps to build, run, and interact with a Docker container using Docker Compose.

## Steps

### 1. Build and Start Containers
To build the services defined in your `docker-compose.yml` file and start them:

```bash
docker-compose up --build
```

This will:
- Build the Docker images if they don't already exist or if they need updates.
- Start the services defined in the `docker-compose.yml` file.

### 2. Attach to a Running Container
Once the containers are running, you can attach to a specific container's terminal:

1. Find the name of the running container:
   ```bash
   docker ps
   ```

   This will list all running containers. Look for the `NAMES` column to find the container you want to attach to.

2. Attach to the container:
   ```bash
   docker attach <container_name>
   ```
   "Now the program is running, but if they don't show any text in the terminal. Just press Enter once."

   Replace `<container_name>` with the name of the container.

   This connects you to the container's main process and allows you to interact with it.

### 3. Detach Safely
To safely detach from the container without stopping it, use the following key combination:

```plaintext
Ctrl+P, then Ctrl+Q
```

This will return you to your terminal while keeping the container running.