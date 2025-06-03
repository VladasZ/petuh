#!/usr/bin/env python3

import subprocess
import platform
import sys


def run(cmd, check=True):
    print(f"Running: {cmd}")
    subprocess.run(cmd, check=check, shell=True)


def main():
    try:
        run("./build.py")

        local_registry = "192.168.0.201:30500"
        image_name = f"{local_registry}/petuh:0.11.5"

        arch = platform.machine()
        os_name = platform.system()

        if os_name == "Linux" and arch == "x86_64":
            print("Building directly with docker (native x86_64 Linux)...")
            run(f"docker build -t {image_name} .")
            run(f"docker push {image_name}")
        else:
            print("Cross-building with docker buildx...")
            builder_name = "petuh_builder"

            run(f"docker buildx create --name {builder_name} --use")
            run("docker buildx inspect --bootstrap")
            run(f"docker buildx build --platform linux/amd64 -t {image_name} --push .")
            run(f"docker buildx rm {builder_name}")

    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
