#!/bin/bash
set -ex

MAJOR_VERSION=0
MINOR_VERSION=1
PATCH_VERSION=0

VERSION="v${MAJOR_VERSION}.${MINOR_VERSION}.${PATCH_VERSION}"

echo "Creating releases for version ${VERSION}"

mkdir -p build

zip -j build/git-tools-$(VERSION)-x86_64-linux-gnu.zip target/x86_64-unknown-linux-gnu/release/branch_selector target/x86_64-unknown-linux-gnu/release/clean_branches

zip -j build/git-tools-$(VERSION)-x86_64_windows.zip target/x86_64-pc-windows-gnu/release/branch_selector.exe target/x86_64-pc-windows-gnu/release/clean_branches.exe