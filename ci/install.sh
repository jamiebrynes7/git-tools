#!/bin/bash

# Adapted from: https://github.com/japaric/trust/blob/master/ci/install.sh

set -ex

cd "$(dirname $0)/../"


rustfmt_check=$(rustup component list | grep "rustfmt" | grep "installed")

if [ -z ${rustfmt_check} ]; then
    echo "Fetching rust linter"
    rustup component add rustfmt-preview
fi


# Fetch cross compilation tool

dest="cross/"

if [ -f "${dest}/cross" ]; then
    echo "Already have cross."
    exit 0
fi

target=
if [ -z "${TRAVIS_OS_NAME}" ]; then
    # We are on local machine.
    target=x86_64-unknown-linux-gnu
    sort=sort
elif [ $TRAVIS_OS_NAME = linux ]; then
    target=x86_64-unknown-linux-musl
    sort=sort
else
    target=x86_64-apple-darwin
    sort=gsort  # for `sort --sort-version`, from brew's coreutils.
fi


cross_repository="https://github.com/japaric/cross"

tag=$(git ls-remote --tags --refs --exit-code ${cross_repository} \
                | cut -d/ -f3 \
                | grep -E '^v[0.1.0-9.]+$' \
                | $sort --version-sort \
                | tail -n1)

download_url="${cross_repository}/releases/download/${tag}/cross-${tag}-${target}.tar.gz"


td=$(mktemp -d || mktemp -d -t tmp)
curl -sL $download_url | tar -C $td -xz

for f in $(ls $td); do
    test -x $td/$f || continue
    mkdir -p $dest
    install -m 755 $td/$f $dest
done

rm -rf $td
