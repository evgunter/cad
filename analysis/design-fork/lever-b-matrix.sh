#!/bin/bash
# Design fork, lever B: the witness and the warp probe under three levers,
# three ε, Hermite-first and march-forced.
BIN=/home/user/tgt-B/debug/deps/all-39c92c0ced009b1d
OUT=${1:-/home/user/lever-b-matrix.out}
: > "$OUT"
for eps in 1e-6 1e-9 1e-12; do
  for lever in chart normal extent; do
    for path in hermite march; do
      force=""; [ "$path" = march ] && force=1
      for t in m5_pr7_ssi::a_flat_wall_whose_chart_bends_answers_as_the_plane_it_is rev3983_probes::rev3983_warp_mid_arm; do
        echo "=== eps=$eps lever=$lever path=$path test=$t" >> "$OUT"
        if [ -n "$force" ]; then
          CAD_TOLERANCE_EPS=$eps LEVER_PROBE=$lever LEVER_PROBE_FORCE_MARCH=1 "$BIN" "$t" --exact --nocapture --test-threads=1 2>&1 | grep -v "^running\|^$\|^test result" | head -40 >> "$OUT"
        else
          CAD_TOLERANCE_EPS=$eps LEVER_PROBE=$lever "$BIN" "$t" --exact --nocapture --test-threads=1 2>&1 | grep -v "^running\|^$\|^test result" | head -40 >> "$OUT"
        fi
      done
    done
  done
done
echo DONE >> "$OUT"
