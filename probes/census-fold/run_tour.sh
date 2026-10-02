#!/usr/bin/env bash
# Runs the tour (tour-hook.patch + kernel-hook.patch applied) at each eps row.
for e in 1e-9 1e-6 1e-12; do
  d=/home/user/cf-out/tour-$e; mkdir -p $d/data
  CENSUS_FOLD_DIR=$d/data CAD_TOLERANCE_EPS=$e /home/user/cf-tour-target/release/demo-tour $d/render >$d/log.txt 2>&1
  echo "exit $?" >>$d/log.txt
done
