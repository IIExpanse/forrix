#!/bin/bash

set -euo pipefail
service_name='control-service'

docker stop "$service_name" || true
docker rm "$service_name" || true
docker rmi "$service_name" || true
docker build --progress=plain -t "$service_name" f ./docker/"$service_name"/Dockerfile .