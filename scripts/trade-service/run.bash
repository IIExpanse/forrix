#!/bin/bash

set -euo pipefail
service_name='trade-service'

read -rsp "Enter telegram bot token: " token
echo ""
docker stop "$service_name" || true
docker rm "$service_name" || true
docker run -e TELOXIDE_TOKEN="$token" -e RUST_LOG=debug --name "$service_name" "$service_name"