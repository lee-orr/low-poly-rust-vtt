if ! command -v trunk &> /dev/null
then
    echo "trunk doesn't exist"
    sudo apt-get update
    sudo apt-get install --no-install-recommends libasound2-dev libudev-dev
    rustup target add wasm32-unknown-unknown
    cargo install trunk
fi

trunk serve