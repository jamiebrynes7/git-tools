#!/bin/bash
set -ex

cd "$(dirname $0)/../"

cargo fmt -- --write-mode=diff > /dev/null
EXIT_CODE=$?

if [ ${EXIT_CODE} -ne 0 ]; then
    echo "Failed linting. Run cargo fmt."
    exit 1
fi