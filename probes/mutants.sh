#!/usr/bin/env bash
# Mutants for PR 3847's rows (review lane reach-dual3847-r2). Each mutant
# is applied, the circle x sphere / first-harmonic rows run, then reverted.
set -u
cd "$(git rev-parse --show-toplevel)"
I=crates/geom-brep/src/implicit.rs; R=crates/topo/src/boolean/circle_roots.rs
run() {
  name=$1; file=$2; from=$3; to=$4
  if [ -n "${ONLY:-}" ] && [[ "$name" != *"$ONLY"* ]]; then return; fi
  cp "$file" /tmp/mut.bak
  python3 - "$file" "$from" "$to" <<'PY'
import sys; p,a,b=sys.argv[1:]; s=open(p).read(); assert s.count(a)==1,(a,s.count(a)); open(p,'w').write(s.replace(a,b))
PY
  out=$(CARGO_INCREMENTAL=0 cargo test -q -p topo --lib boolean::circle 2>&1 | grep -E "^    boolean|test result")
  echo "== $name"; echo "$out" | head -8
  cp /tmp/mut.bak "$file"
}
run "bound x0" $I "        lo_error: lo.error,
        hi_error: hi.error," "        lo_error: lo.error * T::zero(),
        hi_error: hi.error * T::zero(),"
run "phase dropped" $R "    let slack = radius * (at_root / slope + phase + rounding_charge(T::tau()));" "    let slack = radius * (at_root / slope + rounding_charge(T::tau())); let _ = phase;"
run "angle dropped" $R "    let slack = radius * (at_root / slope + phase + rounding_charge(T::tau()));" "    let slack = radius * (at_root / slope + phase);"
run "near/far inverted" $R "select_le_zero(past(hi_value), T::pi() - past(lo_value))" "select_le_zero(T::pi() - past(lo_value), past(hi_value))"
run "old acos half-chord" $R "select_le_zero(past(hi_value), T::pi() - past(lo_value))" "select_le_zero((T::zero() - c0 / a1).max(T::zero() - T::one()).min(T::one()).acos(), (T::zero() - c0 / a1).max(T::zero() - T::one()).min(T::one()).acos()); let _ = past;"
run "root-slack Err arm passes" $R "    match decide(rows.root_slack, Margin::of(slack), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => return Ok(CircleRoots::Uncertain)," "    match decide(rows.root_slack, Margin::of(slack), band) {
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
        Ok(Sign::Positive) => return Ok(CircleRoots::Uncertain),"
run "root-slack meter removed" $R "    match decide(rows.root_slack, Margin::of(slack), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => return Ok(CircleRoots::Uncertain)," "    match decide(rows.root_slack, Margin::of(slack), band) {
        _ => {}"
