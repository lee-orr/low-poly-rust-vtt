# You can find the new timestamped tags here: https://hub.docker.com/r/gitpod/workspace-full/tags
FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install matchbox_server
