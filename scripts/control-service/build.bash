#!/bin/bash

set -euo pipefail
docker stop control-service || true
docker rm control-service || true
docker rmi control-service || true
docker build --progress=plain -t control-service ./control-service