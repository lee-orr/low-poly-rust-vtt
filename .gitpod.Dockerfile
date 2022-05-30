FROM gitpod/workspace-full

RUN sudo apt-get update; sudo apt-get install --no-install-recommends libasound2-dev libudev-dev && rustup target add wasm32-unknown-unknown && cargo install trunk