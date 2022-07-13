URL="match.gschup.dev/bevy"
# ${GITPOD_WORKSPACE_URL/https:\/\//""}"    
echo "{\"signaling_url\":\"$URL\"}" > ./assets/settings.json
matchbox_server & trunk serve --proxy-backend http://localhost:3538 --proxy-rewrite /server/ --proxy-ws && fg