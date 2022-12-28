#! /bin/bash
cd "$(cd "$(dirname "$(readlink -f "$0")")" && cd .. && pwd)"

source /vol0004/apps/oss/llvm-v14.0.1/init.sh

set -eux

./target/aarch64-unknown-linux-gnu/release/tiny-ray \
  --width 1600 \
  --height 1600 \
  --num-rays 8192 \
  --num-reflections 1024 \
  --output "cornell-box.png" \
  cornell-box
