#!/usr/bin/env bash
# The mutants that survive geom-brep's lib suite, against geom-brep's and topo's whole fast sets
# (lib + integration). Usage: survivors.sh <worktree with mutants.py applied>; CARGO_TARGET_DIR set.
cd "$1" || exit 1
cargo nextest run --no-run --profile ci -p geom-brep -p topo > /home/user/logs/survivors-build.log 2>&1 || { echo BUILD FAILED; exit 1; }
for m in r4-ruled-turn-dropped r5-ruled-shift-sign r6-ruled-hinge-at-foot r11-chart-arm-ignores-p r12-chart-arm-doubles-reach r13-chart-arm-refusal-is-reach r15-holds-always-straddles r16-region-span-never-outside r17-cone-boundary-arm-session-box; do
  REVIEW_MUTANT=$m cargo nextest run --profile ci -p geom-brep -p topo --no-fail-fast --status-level fail --final-status-level none > /home/user/logs/surv-$m.log 2>&1
  echo "$m: $(grep -c '^ *FAIL' /home/user/logs/surv-$m.log) red; $(grep Summary /home/user/logs/surv-$m.log | tail -1)"
  grep '^ *FAIL' /home/user/logs/surv-$m.log | sed 's/^ *FAIL *\[[^]]*\] *([^)]*)/    /' | sort -u | head -8
done
echo SURVIVORS DONE
