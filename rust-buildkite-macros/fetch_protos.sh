#!/bin/bash
set -e

# Check if protodep is installed
if ! command -v protodep &> /dev/null; then
    echo "Installing protodep..."
    
    # Determine platform to get the right protodep binary
    PLATFORM=""
    case "$(uname -s)" in
        Darwin)
            PLATFORM="darwin"
            ;;
        Linux)
            PLATFORM="linux"
            ;;
        *)
            echo "Unsupported platform: $(uname -s)"
            exit 1
            ;;
    esac
    
    # Determine architecture
    ARCH=""
    case "$(uname -m)" in
        x86_64)
            ARCH="amd64"
            ;;
        arm64|aarch64)
            ARCH="arm64"
            ;;
        *)
            echo "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
    
    # Download protodep
    PROTODEP_VERSION="v0.1.8"
    DOWNLOAD_URL="https://github.com/stormcat24/protodep/releases/download/${PROTODEP_VERSION}/protodep_${PLATFORM}_${ARCH}.tar.gz"
    
    echo "Downloading protodep from ${DOWNLOAD_URL}..."
    curl -L -o protodep.tar.gz "${DOWNLOAD_URL}"
    tar -xzvf protodep.tar.gz
    chmod +x protodep
    
    # Move to a directory in PATH
    sudo mv protodep /usr/local/bin/
    rm -f protodep.tar.gz
    
    echo "protodep installed successfully!"
fi

# Fetch dependencies using protodep
echo "Fetching proto dependencies..."
# Clean existing proto directory to avoid duplications
if [ -d "./proto" ]; then
    echo "Cleaning existing proto directory..."
    rm -rf ./proto
fi
echo "Running protodep up with HTTPS..."
protodep up -f ./protodep.toml --use-https

# Print information about fetched protos
echo "Proto dependencies fetched successfully!"
echo "Remote APIs commit: 536ec595e1df0064bb37aecc95332a661b8c79b2"
echo "Bazel commit: 61be2d6010650299b361b53686d9935d4f562eed"
echo "Googleapis commit: 1c153adc542b4c915eeab5290bc42581c821cc93"
echo "Google protobuf commit: v21.12"

echo "done"