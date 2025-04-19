#!/usr/bin/env python3

import subprocess
from pathlib import Path
import platform
import shutil

# Config
TARGET = "x86_64-unknown-linux-gnu"
DOCKER_IMAGE = "rust:latest"
FINAL_PATH = Path(f"target/{TARGET}/release/glavpetuh")


def build_locally():
    result = subprocess.run(["cargo", "build", "--release"], check=True)

    # Copy the binary to the cross-targeted location
    native_path = Path("target/release/glavpetuh")
    FINAL_PATH.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(native_path, FINAL_PATH)


def build_with_docker():
    docker_cmd = (
        f'docker run --rm '
        f'--platform linux/amd64 '
        f'-v "{Path.cwd()}:/usr/src/myapp" '
        f'-w /usr/src/myapp '
        f'{DOCKER_IMAGE} '
        f'bash -c "apt update && '
        f'apt install -y gcc libc6-dev pkg-config libssl-dev && '
        f'rustup target add {TARGET} && '
        f'export PKG_CONFIG_ALLOW_CROSS=1 && '
        f'cargo build --release --target={TARGET}"'
    )

    subprocess.run(docker_cmd, shell=True, check=True)


if __name__ == "__main__":
    system = platform.system()
    machine = platform.machine()

    if system == "Linux" and machine == "x86_64":
        build_locally()
    else:
        build_with_docker()
