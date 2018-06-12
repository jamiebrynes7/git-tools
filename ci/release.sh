#!/bin/bash
set -ex

MAJOR_VERSION=0
MINOR_VERSION=2
PATCH_VERSION=0

VERSION="v${MAJOR_VERSION}.${MINOR_VERSION}.${PATCH_VERSION}"

echo "Creating releases for version ${VERSION} and ${TARGET}"
mkdir -p build

# Remove build artifacts and zip
rm target/${TARGET}/release/*.d target/${TARGET}/release/*.rlib
zip -j build/git-tools-${VERSION}-${TARGET}.zip target/${TARGET}/release/*
