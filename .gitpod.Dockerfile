FROM gitpod/workspace-full

RUN sudo apt-get update
RUN sudo apt-get install --no-install-recommends libasound2-dev libudev-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
