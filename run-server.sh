URL="${GITPOD_WORKSPACE_URL/https:\/\//"7768-"}"
cargo watch -x "run --no-default-features --features server -- --webrtc-url=https://$URL"