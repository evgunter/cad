#!/bin/bash
# usage: mut.sh <name> <line> <sed-expr>
cd /home/user/cad
F=crates/sweep/tests/spline_spiric_operands.rs
export CARGO_TARGET_DIR=$S/target
cp $S/orig.rs $F
sed -i "$2s/$3/" $F
echo "== $1: $(sed -n "$2p" $F | sed 's/^ *//')"
for e in 1e-6 1e-9; do
  out=$(CAD_TOLERANCE_EPS=$e timeout 400 cargo nextest run -p sweep --profile ci --ignore-default-filter -E 'test(=spline_spiric_operands::no_spline_or_spiric_operand_builds_wrong)' --no-capture 2>&1)
  if echo "$out" | grep -q "1 passed"; then echo "  ε $e: SURVIVES"; else echo "  ε $e: KILLED: $(echo "$out" | grep -A1 "panicked" | tail -1 | cut -c1-220)"; fi
done
cp $S/orig.rs $F
