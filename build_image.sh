#!/bin/bash

set -euo pipefail

./build.py

IMAGE_NAME="vladasz/petuh:0.11.2"

docker login

ARCH=$(uname -m)
OS=$(uname -s)

if [[ "$OS" == "Linux" && "$ARCH" == "x86_64" ]]; then
    echo "Building directly with docker (native x86_64 Linux)..."
    docker build -t "$IMAGE_NAME" .
    docker push "$IMAGE_NAME"
else
    echo "Cross-building with docker buildx..."
    docker buildx create --use || true
    docker buildx inspect --bootstrap
    docker buildx build --platform linux/amd64 -t "$IMAGE_NAME" --push .
fi
