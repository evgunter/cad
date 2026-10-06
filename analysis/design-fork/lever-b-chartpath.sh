#!/bin/bash
# Lever B: the bent-chart-path witness under three levers, three ε, both paths.
BIN=/home/user/tgt-B/debug/deps/all-39c92c0ced009b1d
OUT=${1:-/home/user/lever-b-chartpath.out}
: > "$OUT"
for eps in 1e-9 1e-12 1e-6; do
  for lever in chart normal extent; do
    for path in march hermite; do
      echo "=== eps=$eps lever=$lever path=$path" >> "$OUT"
      if [ "$path" = march ]; then
        CAD_TOLERANCE_EPS=$eps LEVER_PROBE=$lever LEVER_PROBE_FORCE_MARCH=1 LEVER_PROBE_TRACE=1 "$BIN" rev3983_probes::leverb_warp_chart_path --exact --nocapture --test-threads=1 2>&1 | awk '/LEVERTRACE/{n++; split($0,a,"chosen="); c=a[2]+0; if(c<min||n==1){min=c; minline=$0}; next} /LEVERB/{print "  states_decided=" n "  min_chosen_arm=" min; print "  min_line: " minline; print; n=0; min=0} /panicked|FAILED|^test /{print}' >> "$OUT"
      else
        CAD_TOLERANCE_EPS=$eps LEVER_PROBE=$lever LEVER_PROBE_TRACE=1 "$BIN" rev3983_probes::leverb_warp_chart_path --exact --nocapture --test-threads=1 2>&1 | awk '/LEVERTRACE/{n++; split($0,a,"chosen="); c=a[2]+0; if(c<min||n==1){min=c; minline=$0}; next} /LEVERB/{print "  states_decided=" n "  min_chosen_arm=" min; print "  min_line: " minline; print; n=0; min=0} /panicked|FAILED|^test /{print}' >> "$OUT"
      fi
    done
  done
done
echo DONE >> "$OUT"
