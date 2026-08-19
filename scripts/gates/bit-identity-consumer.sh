#!/usr/bin/env bash
# bit-identity-consumer.sh — the bit-identity consumer tripwire
# (retirement landed, M4 PR 5). ONE home; ci.yml's "bit-identity
# consumer tripwire (retirement landed, M4 PR 5)" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# Bit-identity coincidence checking is RETIRED from production
# (M4 PR 5, NAMING-DESIGN N6; DESIGN.md roadmap; Evan, #53): the
# declared rung is GeomSource lookup, and the production-consumer
# allowlist is EMPTY. The remaining rows are NON-consumers:
#  - bit_identity.rs — the sanctioned seam itself;
#  - interval.rs — Interval::repr_bits over its OWN storage:
#    scalar plumbing (exact-representation access), never a
#    coincidence comparison;
#  - eval/memo.rs — HASHES exact bits into content keys (spec D4
#    "values AS BITS"): scalar plumbing, never a comparison;
#  - topo/source.rs — the cfg(debug_assertions)-gated
#    "records agree with bits" assertion (N6's
#    debug_assert!(same_source => eq_bits)); not in production
#    builds.
# The tripwire stays ARMED: any NEW file using the channel fails
# here — a new production consumer is a design regression against
# the ratified retirement (add nothing without a DESIGN.md
# revision). Comment lines are excluded (docs may name the
# channel; code may not use it unlisted).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  if grep -rnE 'bit_identity::|repr_bits|eq_bits' crates/*/src \
    | grep -vE '^crates/geom-core/src/bit_identity\.rs:' \
    | grep -vE '^crates/geom-core/src/interval\.rs:' \
    | grep -vE '^crates/topo/src/source\.rs:' \
    | grep -vE '^crates/editor-core/src/eval/memo\.rs:' \
    | grep -vE ':[0-9]+:\s*//'; then
    gate_error "bit-identity channel use above — the channel is RETIRED from production (M4 PR 5, N6); use GeomSource, or revise DESIGN.md before adding any consumer"
    exit 1
  fi
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn same(a: f64, b: f64) -> bool { geom_core::bit_identity::eq_bits(a, b) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_parse_args "$@"
gate_main "bit-identity channel use above" plant
