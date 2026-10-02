#!/bin/bash
# usage: mut.sh name sed-expr
set -u
F=topo/src/splitting/classify.rs
cp $F /tmp/classify.orig
sed -i "$2" $F
if cmp -s $F /tmp/classify.orig; then echo "MUTANT $1: sed did nothing"; exit 1; fi
echo "== MUTANT $1"
CARGO_INCREMENTAL=0 timeout 3000 cargo test --release --no-fail-fast -p sweep -p topo --test all -- reach_split_gate_per_face split_gate_per_face probe_r2 --nocapture 2>&1 | grep -E "^error|FAILED|test result|TALLY|panicked at crates/(sweep|topo)/tests" | sort | uniq -c | cut -c1-250 | head -30
cp /tmp/classify.orig $F
# Invocations used (line numbers against classify.rs at e284e7f1):
# M1 always-pass:  mutants.sh always-pass '44s/let face_clears = |face| box_clears(gate_face_reach(body, face, band), plane, band);/let face_clears = |_face: FaceKey| true;/'
# M2 whole-ball:   mutants.sh whole-ball '112s/let ball = crate::census::face_reach(body, face, band)?;/let ball = crate::census::face_reach(body, face, band)?; if true { return Some(ball); }/'
# M3 pad-zero:     mutants.sh pad-zero '180s/let pad = T::from_f64(crate::boolean::boxes::sweep_pad(band));/let pad = T::from_f64(0.0 * crate::boolean::boxes::sweep_pad(band));/'
# M4 edge face-clear off: mutants.sh edge-face-clear-off '82s/|| bounding.into_iter().flatten().any(face_clears);/|| (bounding.into_iter().flatten().any(face_clears) \&\& false);/'
