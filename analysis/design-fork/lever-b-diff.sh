#!/bin/bash
# Lever B: which suite rows move between levers, per ε.
D=${1:-/home/user/lever-b-suites}
for eps in 1e-6 1e-9 1e-12; do
  for lever in normal extent; do
    echo "== $eps: chart vs $lever"
    diff <(sed 's/ \.\.\. / /' "$D/$eps-chart.verdicts") <(sed 's/ \.\.\. / /' "$D/$eps-$lever.verdicts") | grep '^[<>]' || echo "  (no row moves)"
  done
  echo "== $eps summaries"; cat "$D/$eps-"*.summary
done
