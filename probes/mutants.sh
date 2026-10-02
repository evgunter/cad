#!/bin/bash
# usage: mutants.sh  (runs from repo root)
set -u
F=crates/topo/src/boolean/ellipse_roots.rs
run() { # name file old new
  name=$1; f=$2; old=$3; new=$4
  cp "$f" /tmp/mut_backup
  python3 - "$f" "$old" "$new" <<'PY'
import sys
p,o,n=sys.argv[1:4]
s=open(p).read()
assert s.count(o)==1, (o, s.count(o))
open(p,'w').write(s.replace(o,n))
PY
  for e in 1e-9 1e-6; do
    r=$(CAD_TOLERANCE_EPS=$e CARGO_INCREMENTAL=0 timeout 900 cargo nextest run -p topo --no-fail-fast -E 'test(/ellipse_roots::tests|probe_r2_ellipse_roots_fuzz|probe_r2_near_tangent/)' 2>&1 | grep -E "Summary|^\s+FAIL \[" | sort -u | tr '\n' ' ')
    echo "$name @ $e: $r"
  done
  cp /tmp/mut_backup "$f"
}
run lever_2a $F "lever: two * conic.minor," "lever: two * conic.major,"
run speed_lo_major $F "speed_lo: conic.minor," "speed_lo: conic.major,"
run speed_hi_minor $F "speed_hi: conic.major," "speed_hi: conic.minor,"
run first_speed_minor $F "first_harmonic_roots(&first, conic.major," "first_harmonic_roots(&first, conic.minor,"
run drop_second_noise $F "noise: noise + second," "noise: noise,"
