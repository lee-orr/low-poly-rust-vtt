URL="${GITPOD_WORKSPACE_URL/https:\/\//"https://7768-"}"
echo "{\"signaling_url\":\"$URL\"}" > ./assets/settings.json
trunk serve