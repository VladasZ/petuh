#!/bin/bash

set -euo pipefail

./build.py

docker login
docker buildx create --use
docker buildx inspect --bootstrap
docker buildx build --platform linux/amd64 -t vladasz/petuh:0.8.5 --push .
