#!/bin/bash

set -ex

cd "$(dirname $0)/../"

rustup component add rustfmt-preview
