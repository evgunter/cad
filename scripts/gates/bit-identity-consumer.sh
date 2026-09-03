#!/usr/bin/env bash
# bit-identity-consumer.sh — the bit-identity consumer tripwire
# (retirement landed, M4 PR 5). ONE home; ci.yml's "bit-identity
# consumer tripwire (retirement landed, M4 PR 5)" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# Bit-identity coincidence checking is RETIRED from production
# (M4 PR 5, NAMING-DESIGN N6; DESIGN.md roadmap; Ev, #53): the
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
# revision). Docs may name the channel; code may not use it
# unlisted, so the scan reads `lib.sh`'s code-only view: comments
# and string literals are gone in BOTH directions, where the
# leading-`//` filter this gate carried stripped neither a trailing
# comment nor a block one.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  gate_require_crate_sources
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E 'bit_identity::|repr_bits|eq_bits' \
    | gate_grep -vE '^crates/geom-core/src/bit_identity\.rs:' \
    | gate_grep -vE '^crates/geom-core/src/interval\.rs:' \
    | gate_grep -vE '^crates/topo/src/source\.rs:' \
    | gate_grep -vE '^crates/editor-core/src/eval/memo\.rs:')
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "bit-identity channel use above — the channel is RETIRED from production (M4 PR 5, N6); use GeomSource, or revise DESIGN.md before adding any consumer"
    exit 1
  fi
  gate_ok "no bit-identity consumer outside the four non-consumer rows"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn same(a: f64, b: f64) -> bool { geom_core::bit_identity::eq_bits(a, b) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# A use hidden behind a block comment on one line: the leading-`//`
# filter read the whole line as code and the matcher fired on the prose
# beside it. Both halves of that are wrong and this case pins the fix.
plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn same(a: f64, b: f64) -> bool { /* why */ geom_core::bit_identity::eq_bits(a, b) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# THE NEAR MISSES: prose naming the retired channel is exactly what the
# docs are supposed to do, and a gate that reds on it is the cry-wolf
# half that pushes the next reader toward an allowlist entry.
plant_prose_only() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! The bit channel (bit_identity::eq_bits) is retired.\n'
    printf '/// Do not call repr_bits here.\n'
    printf '/*\n * eq_bits is named in this block comment only.\n */\n'
    printf 'pub const WHY: &str = "bit_identity::eq_bits";\n'
    printf 'pub fn ok(a: f64) -> f64 { a } // and repr_bits in a trailing one\n'
  } > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="bit-identity channel use above"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_passes "prose, doc comments and a string literal naming the channel" plant_prose_only
  printf '%s selftest OK: passes a clean fixture and prose/doc/string mentions of the channel; fires on a use, and on a use hidden behind a block comment; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
