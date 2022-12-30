#! /bin/bash
cd "$(cd "$(dirname "$(readlink -f "$0")")" && cd .. && pwd)"

source /vol0004/apps/oss/llvm-v14.0.1/init.sh

set -eux

cargo build --target 'aarch64-unknown-linux-gnu' --release
