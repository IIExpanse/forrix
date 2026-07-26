#!/bin/bash

set -euo pipefail
read -rsp "Enter telegram bot token: " token
echo ""
docker stop control-service || true
docker rm control-service || true
docker run -e TELOXIDE_TOKEN="$token" -e RUST_LOG=debug --name control-service control-service