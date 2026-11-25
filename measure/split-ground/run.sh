#!/bin/bash
# embedwalk-splitground: build + run the population with the split-ground log on.
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/embedwalk-splitground-target CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1
export RUSTC_WORKSPACE_WRAPPER=$PWD/scripts/rustc-test-crates-opt0.sh
S=${S:-~/.local/share/cad-work/embedwalk-splitground-scratch}
L=$S/log; mkdir -p $L; export CAD_SPLIT_GROUND=$L
for c in "$@"; do
  echo "== $c $(date +%T)"
  local-scripts/with-build-slot.sh -- cargo nextest run -p $c --profile ci --ignore-default-filter --no-fail-fast --test-threads 4 -E "$(cat $(dirname $(readlink -f $0))/filter-$c.txt)" > $S/nextest-$c.out 2>&1
  echo "== $c done $? $(date +%T)"
done
