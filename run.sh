URL="${GITPOD_WORKSPACE_URL/https:\/\//"7768-"}"
echo "{\"signaling_url\":\"$URL\"}" > ./assets/settings.json
trunk serve & cargo watch -x 'run -- --no-default-features --features server' && fg