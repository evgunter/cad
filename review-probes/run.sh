#!/bin/bash
# usage: run.sh <tag> [extra env assignments...]  -- runs the topo+sweep census population with the gate asking
cd /home/user/cad-measure
export CARGO_TARGET_DIR=/home/user/measure-target CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0
tag=$1; shift
rm -rf /home/user/cad-measure-probes/log-$tag; mkdir -p /home/user/cad-measure-probes/log-$tag
for pkg in topo sweep; do
  env "$@" CAD_GATE_LOG=/home/user/cad-measure-probes/log-$tag cargo nextest run -j 2 --build-jobs 2 -p $pkg --profile ci --ignore-default-filter --no-fail-fast -E "$(cat /home/user/cad-measure-probes/pop-$pkg.filter)" > /home/user/cad-measure-probes/out-$tag-$pkg.txt 2>&1
done
echo done > /home/user/cad-measure-probes/log-$tag/DONE
