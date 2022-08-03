URL="${GITPOD_WORKSPACE_URL/https:\/\//"7767-"}"
cargo watch -x "run --no-default-features --features server -- --webrtc-url=https://$URL"