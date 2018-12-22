#!/bin/bash
set -ex

MAJOR_VERSION=0
MINOR_VERSION=3
PATCH_VERSION=1

VERSION="v${MAJOR_VERSION}.${MINOR_VERSION}.${PATCH_VERSION}"

echo "Creating releases for version ${VERSION} and ${TARGET}"
mkdir -p build

# Remove build artifacts and zip
rm target/release/*.d target/release/*.rlib
zip -j build/git-tools-${VERSION}-${TARGET}.zip target/release/*
