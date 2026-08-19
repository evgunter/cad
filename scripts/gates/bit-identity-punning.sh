#!/usr/bin/env bash
# bit-identity-punning.sh — the type-punning tripwire. ONE home;
# ci.yml's "bit-identity punning tripwire" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# Complementary punning tripwire (M3 PR 4; Evan #53/#57/#58): the
# type-punning plumbing that lets generic code reach a scalar's
# bits (Any downcasts / TypeId dispatch) is sanctioned in exactly
# ONE seam — geom_core::bit_identity. Any other occurrence is a
# second bit channel and fails here. Callers (including the
# allowlisted consumers above) go through the door's public fns
# (repr_bits/eq_bits), which need no punning at the call site.
# Same M4 retirement schedule as above.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  if grep -rnE 'downcast_ref|downcast_mut|TypeId|core::any|std::any' crates/*/src \
    | grep -vE '^crates/geom-core/src/bit_identity\.rs:' \
    | grep -vE ':[0-9]+:\s*//'; then
    gate_error "bit-identity punning outside the sanctioned seam (geom-core/src/bit_identity.rs)"
    exit 1
  fi
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn as_f64(v: &dyn core::any::Any) -> Option<&f64> { v.downcast_ref::<f64>() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_parse_args "$@"
gate_main "bit-identity punning outside the sanctioned seam" plant
