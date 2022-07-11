# You can find the new timestamped tags here: https://hub.docker.com/r/gitpod/workspace-full/tags
FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN sudo apt-get update
RUN apt-get install --no-install-recommends libasound2-dev libudev-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
