#!/bin/bash
set -ex

cd "$(dirname $0)/../"

cargo fmt -- --write-mode=diff