#!/bin/bash
set -e

# 1. Build for ARM64
cargo clean
cargo build --target aarch64-apple-darwin

# 2. Copy the binary to the npm package
cp target/aarch64-apple-darwin/debug/raphook packages/npm/raphook/bin/
cp target/aarch64-apple-darwin/debug/raphook packages/npm/raphook-darwin-arm64/bin/

# 3. Change directory to npm package and run npm pack
cd packages/npm/raphook
npm pack

# 4. Also pack the platform-specific package
cd ../raphook-darwin-arm64
npm pack

echo "Build and pack completed successfully!"
