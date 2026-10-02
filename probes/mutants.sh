#!/usr/bin/env bash
# Reviewer mutant runner (reach-dual3844-r1). Applies one mutant to
# crates/topo/src/boolean/ops.rs at the frozen head, runs the rows the PR
# touches, restores. Line numbers are the frozen head's.
set -u
F=crates/topo/src/boolean/ops.rs
cp $F /tmp/ops.rs.orig
mut() {
  case $1 in
    allow0)   sed -i '1678,1679s/band.escalate()/(0.0 * band.escalate())/' $F ;;
    allow100) sed -i '1678,1679s/band.escalate()/(100.0 * band.escalate())/' $F ;;
    nointerval) sed -i '1852a\            if true { return Err(implausible()); }' $F ;;
    nomaterial) sed -i '1915a\    if true { return Ok(()); }' $F ;;
    nounion)  sed -i '1705s/if ba \&\& bb {/if false \&\& ba \&\& bb {/' $F ;;
  esac
}
for m in "$@"; do
  cp /tmp/ops.rs.orig $F; mut $m
  echo "=== mutant $m: $(git diff --stat -- $F | tail -1)"
  for e in 1e-9 1e-12; do
    CARGO_INCREMENTAL=0 CAD_TOLERANCE_EPS=$e cargo nextest run -p topo --lib --test all \
      -E 'test(volume_backstop) | test(door_backstop) | test(contact9_side_codes) | test(at_rest_policy)' \
      --no-fail-fast 2>&1 | grep -E "^\s*FAIL \[|Summary|error\[" | sort -u | sed "s/^/  [$e topo] /"
    CARGO_INCREMENTAL=0 CAD_TOLERANCE_EPS=$e cargo nextest run -p sweep --test all reach_continuation \
      --no-fail-fast 2>&1 | grep -E "^\s*FAIL \[|Summary|error\[" | sort -u | sed "s/^/  [$e sweep] /"
  done
done
cp /tmp/ops.rs.orig $F
