#!/bin/bash
set -e

# 1. Build for ARM64
cargo build --release --target aarch64-apple-darwin

# 2. Change directory to npm package
cd packages/npm/raphook

# 3. Run npm pack
npm pack

echo "Build and pack completed successfully!"
