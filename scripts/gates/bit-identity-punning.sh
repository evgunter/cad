#!/usr/bin/env bash
# bit-identity-punning.sh — the type-punning tripwire. ONE home;
# ci.yml's "bit-identity punning tripwire" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# Complementary punning tripwire (M3 PR 4; Ev #53/#57/#58): the
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

# THE SANCTIONED SEAM'S PATH, held once: the filter's exemption is built
# from it and the clean fixture plants it.#
# ONE LIST, AND ONE DIRECTION PROVED. The fixture->filter direction reds
# the clean fixture the moment a planted home stops being exempt; the
# filter->fixture direction is convention and not a check — a filter
# naming a home no fixture plants stays green here, and catching that is
# the subject-half residue's job
# (`work/gates/whole-file-skips-do-not-check-their-subject.md`).
HOME_FILE=crates/geom-core/src/bit_identity.rs

gate() {
  gate_require_crate_sources
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E 'downcast_ref|downcast_mut|TypeId|core::any|std::any' \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "bit-identity punning outside the sanctioned seam (geom-core/src/bit_identity.rs)"
    exit 1
  fi
  gate_ok "no type punning outside geom-core/src/bit_identity.rs"
}

# THE SEAM ITSELF IS IN THE CLEAN FIXTURE, which is `lib.sh`'s
# exact-skip contract read for a whole-file skip: a skip no fixture
# exercises is dead in every case, and an anchor that over-narrows is
# then noticed by nobody. The home carries the punning it is the home
# OF, so the clean case reds the moment the exemption stops covering it.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}"
  printf 'pub fn as_f64(v: &dyn core::any::Any) -> Option<&f64> { v.downcast_ref::<f64>() }\n' \
    > "$1/$HOME_FILE"
}

# The home followed by a colon that is not a line number — one of the
# three shapes `gate_record_anchor`'s header enumerates, and the one a
# skip that ends at `:` exempts.
plant_colon_after_the_home_that_is_not_a_line_number() {
  printf 'pub fn as_f64(v: &dyn core::any::Any) -> Option<&f64> { v.downcast_ref::<f64>() }\n' \
    > "$1/$HOME_FILE:x.rs"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn as_f64(v: &dyn core::any::Any) -> Option<&f64> { v.downcast_ref::<f64>() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn as_f64(v: &dyn core::any::Any) -> Option<&f64> { /* why */ v.downcast_ref::<f64>() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_prose_only() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! Punning (downcast_ref, TypeId) lives in one seam.\n'
    printf '/*\n * std::any is named in this block comment only.\n */\n'
    printf 'pub const WHY: &str = "core::any::TypeId";\n'
    printf 'pub fn ok(a: f64) -> f64 { a } // and downcast_mut in a trailing one\n'
  } > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="bit-identity punning outside the sanctioned seam"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "prose, doc comments and a string literal naming the plumbing" plant_prose_only
  printf '%s selftest OK: passes a clean fixture carrying the sanctioned seam itself, and prose/doc/string mentions of the punning plumbing; fires on a downcast, on one hidden behind a block comment, and at the colon-carrying path a home skip that ends at `:` exempts; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
