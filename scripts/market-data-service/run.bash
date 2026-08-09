#!/bin/bash

set -euo pipefail
service_name='market-data-service'

read -rsp "Enter api url: " api_url
echo ""
read -rsp "Enter api token: " api_token
echo ""
docker stop "$service_name" || true
docker rm "$service_name" || true
docker run -e API_URL="$api_url" -e API_TOKEN="$api_token" -e RUST_BACKTRACE=full -e RUST_LOG=debug --name "$service_name" "$service_name"