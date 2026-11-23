#!/usr/bin/env bash
# Second delta: run-time mutants on the frozen head bb12af18 (head-with-mutants.diff), each over its
# crate's whole fast set. CARGO_TARGET_DIR set; run from the patched worktree.
for m in e1-below-takes-apex-reach-from-above e6-below-without-apex-reach e5-above-without-apex-reach e4-coaxial-reads-below e3-run-below-for-both e9-run-above-for-both e2-spline-section-reads-above; do
  out=$(REVIEW_MUTANT=$m cargo nextest run --profile ci -p step-import --no-fail-fast --status-level fail --final-status-level none 2>&1)
  echo "$m: $(echo "$out" | grep -c '^ *FAIL') red; $(echo "$out" | grep Summary | tail -1)"; echo "$out" | grep '^ *FAIL' | sed -E 's/^ *FAIL *\[[^]]*\] *\([^)]*\)/    /' | sort -u | head -6
done
for m in e7-point-given-radius e8-cone-points-always-held; do
  out=$(REVIEW_MUTANT=$m cargo nextest run --profile ci -p geom-brep --no-fail-fast --status-level fail --final-status-level none 2>&1)
  echo "$m: $(echo "$out" | grep -c '^ *FAIL') red; $(echo "$out" | grep Summary | tail -1)"; echo "$out" | grep '^ *FAIL' | sed -E 's/^ *FAIL *\[[^]]*\] *\([^)]*\)/    /' | sort -u | head -6
done
echo MUTANTS DONE
