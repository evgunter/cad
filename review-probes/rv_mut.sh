#!/bin/bash
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/review-target
for m in "$@"; do
  echo "== $m"
  RV_MUT=$m cargo nextest run -p topo -p sweep -j 3 --no-fail-fast --ignore-default-filter \
    -E '(package(topo) & kind(lib) & test(/^injective::/)) | (package(sweep) & test(/self_intersection_proves_injectivity::/))' 2>&1 \
    | grep -E '^\s+(FAIL|TIMEOUT|SIGABRT|SIGSEGV)|Summary' | sed 's/\[.*\] *([^)]*)//' | sort -u | cut -c1-200
done
