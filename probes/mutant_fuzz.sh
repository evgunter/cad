#!/usr/bin/env bash
# Runs each mutant of PR 3847's slack through the fuzz corpus and the
# independent oracle (review lane reach-dual3847-r2).
set -u
cd "$(git rev-parse --show-toplevel)"
IN=$1; S=$(dirname "$IN")
R=crates/topo/src/boolean/circle_roots.rs; C=crates/topo/src/boolean/circle_sphere.rs
mut() {
  name=$1; file=$2; from=$3; to=$4
  if [ -n "${ONLY:-}" ] && ! [[ "$ONLY" == *"$name"* ]]; then return; fi
  cp "$file" /tmp/mutf.bak
  python3 - "$file" "$from" "$to" <<'PY'
import sys; p,a,b=sys.argv[1:]; s=open(p).read(); assert s.count(a)==1,(a,s.count(a)); open(p,'w').write(s.replace(a,b))
PY
  CARGO_INCREMENTAL=0 CS_PROBE_IN=$IN CS_PROBE_OUT=$S/mut.txt cargo test -q -p topo --lib cs_probe_run >/dev/null 2>&1
  cp /tmp/mutf.bak "$file"
  echo "== $name"; python3 probes/cs_oracle.py $IN $S/mut.txt > $S/mut_or.txt; grep -v "^ROOT-OFF" $S/mut_or.txt; echo "ROOT-OFF rows: $(grep -c "^ROOT-OFF" $S/mut_or.txt)"; grep "^ROOT-OFF" $S/mut_or.txt | sort -t" " -k8 -g | tail -2
}
mut "bound x0" $C "            lo_noise: h.lo_error,
            hi_noise: h.hi_error," "            lo_noise: h.lo_error * T::zero(),
            hi_noise: h.hi_error * T::zero(),"
mut "phase dropped" $R "    let slack = radius * (at_root / slope + phase + rounding_charge(T::tau()));" "    let slack = radius * (at_root / slope + rounding_charge(T::tau())); let _ = phase;"
mut "angle dropped" $R "    let slack = radius * (at_root / slope + phase + rounding_charge(T::tau()));" "    let slack = radius * (at_root / slope + phase);"
mut "near/far inverted" $R "select_le_zero(past(hi_value), T::pi() - past(lo_value))" "select_le_zero(T::pi() - past(lo_value), past(hi_value))"
mut "old acos half-chord" $R "select_le_zero(past(hi_value), T::pi() - past(lo_value))" "select_le_zero((T::zero() - c0 / a1).max(T::zero() - T::one()).min(T::one()).acos(), (T::zero() - c0 / a1).max(T::zero() - T::one()).min(T::one()).acos()); let _ = past;"
