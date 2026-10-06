#!/bin/bash
# usage: run.sh <variant-or-base>
label=$1; v=${1%-d}; [ "$v" = base ] && v=""
cd /home/user/wt-A
export CARGO_TARGET_DIR=/home/user/target-A
for e in 1e-6 1e-9 1e-12; do
  s=$(date +%s)
  LEVER_VARIANT=$v CAD_TOLERANCE_EPS=$e cargo test -p geom-brep --release --no-fail-fast -- --test-threads=4 2>&1 \
    | grep -E "^test .* \.\.\. |panicked at|^---- " > /home/user/scratch-A/$label-$e.txt
  echo "$label $e: $(grep -c ' ok$' /home/user/scratch-A/$label-$e.txt) ok, $(grep -c 'FAILED$' /home/user/scratch-A/$label-$e.txt) failed, $(( $(date +%s)-s ))s"
done
