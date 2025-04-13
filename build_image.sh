#!/bin/bash

set -euo pipefail

docker login
docker buildx create --use
docker buildx inspect --bootstrap
docker buildx build --platform linux/amd64 -t vladasz/petuh:0.7.6 --push .
