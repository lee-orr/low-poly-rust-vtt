URL="${GITPOD_WORKSPACE_URL/https:\/\//"7768-"}"
echo "{\"signaling_url\":\"$URL\"}" > ./assets/settings.json
cargo watch -x "run --no-default-features --features server -- --webrtc-url=https://$URL"