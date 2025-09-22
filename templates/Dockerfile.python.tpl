# templates/Dockerfile.python.tpl
#
# Stage 1: Base image with dependencies
FROM python:3.11-slim AS base
WORKDIR /app

# Install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the application
COPY . .

# Expose the port the app runs on
EXPOSE {{ port }}

# Command to run the application
CMD ["python", "{{ entry_point }}"]