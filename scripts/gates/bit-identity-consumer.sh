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

# THE NON-CONSUMER ROWS, as paths and held once: the filter's
# exemption is built from this list and the clean fixture plants every
# entry of it, so a row cannot be exempt in one and absent from the
# other.#
# ONE LIST, AND ONE DIRECTION PROVED. The fixture->filter direction reds
# the clean fixture the moment a planted home stops being exempt; the
# filter->fixture direction is convention and not a check — a filter
# naming a home no fixture plants stays green here, and catching that is
# the subject-half residue's job
# (`work/gates/whole-file-skips-do-not-check-their-subject.md`).
NON_CONSUMER_HOMES=(
  crates/geom-core/src/bit_identity.rs
  crates/geom-core/src/interval.rs
  crates/topo/src/source.rs
  crates/editor-core/src/eval/memo.rs
)

gate() {
  gate_require_crate_sources
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E 'bit_identity::|repr_bits|eq_bits' \
    | gate_grep -vE "$(gate_record_anchor_any "${NON_CONSUMER_HOMES[@]}")")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "bit-identity channel use above — the channel is RETIRED from production (M4 PR 5, N6); use GeomSource, or revise DESIGN.md before adding any consumer"
    exit 1
  fi
  gate_ok "no bit-identity consumer outside the ${#NON_CONSUMER_HOMES[@]} non-consumer rows"
}

# EVERY ROW IS IN THE CLEAN FIXTURE, which is `lib.sh`'s
# exact-skip contract read for a whole-file skip: a skip no fixture
# exercises is dead in every case, and an anchor that over-narrows is
# then noticed by nobody. Each home carries the channel use it is
# exempted FOR, so the clean case reds the moment one of them stops
# being covered.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "${NON_CONSUMER_HOMES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub fn same(a: f64, b: f64) -> bool { geom_core::bit_identity::eq_bits(a, b) }\n' \
      > "$1/$home"
  done
}

# The home followed by a colon that is not a line number — one of the
# three shapes `gate_record_anchor`'s header enumerates, and the one a
# skip that ends at `:` exempts.
plant_colon_after_the_home_that_is_not_a_line_number() {
  local home
  for home in "${NON_CONSUMER_HOMES[@]}"; do
    printf 'pub fn same(a: f64, b: f64) -> bool { geom_core::bit_identity::eq_bits(a, b) }\n' \
      > "$1/$home:x.rs"
  done
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
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "prose, doc comments and a string literal naming the channel" plant_prose_only
  printf '%s selftest OK: passes a clean fixture carrying every non-consumer row, and prose/doc/string mentions of the channel; fires on a use, on a use hidden behind a block comment, and at the colon-carrying path a home skip that ends at `:` exempts; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
