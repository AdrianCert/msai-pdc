# Use an official Python runtime as a parent image
FROM python:3.11-slim

# Set the working directory in the container to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

RUN pip install .

# Install PDM, a modern Python package manager
# # Use PDM to install the Python dependencies from pyproject.toml
# RUN pdm install --no-editable

# Make port 80 available to the world outside this container
EXPOSE 80

CMD ["python", "-m", "snap4frame_backend.server"]