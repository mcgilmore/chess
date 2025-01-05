#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/chess
readonly DESTINATION_PATH=$1

cargo build --release --target=${TARGET_ARCH}
scp ${SOURCE_PATH} ${DESTINATION_PATH}
