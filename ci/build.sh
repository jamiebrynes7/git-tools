#!/bin/bash
set -ex

cd "$(dirname $0)/../"

cross="cross/cross"

if [ ! -f ${cross} ]; then
    echo "Cross not found. Run ci/install.sh to fetch this."
    exit 1
fi

$cross build --target $TARGET
$cross build --target $TARGET --release