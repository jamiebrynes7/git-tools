#!/bin/bash
set -ex

cd "$(dirname $0)/../"

cross="cross/cross"

if [ -f ${cross} ]; then
    echo "Cross not found. Run ci/install.sh to fetch this."
fi

$cross test --target $TARGET
$cross test --target $TARGET --release