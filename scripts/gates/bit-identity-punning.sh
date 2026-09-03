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

gate() {
  gate_require_crate_sources
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E 'downcast_ref|downcast_mut|TypeId|core::any|std::any' \
    | gate_grep -vE '^crates/geom-core/src/bit_identity\.rs:')
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "bit-identity punning outside the sanctioned seam (geom-core/src/bit_identity.rs)"
    exit 1
  fi
  gate_ok "no type punning outside geom-core/src/bit_identity.rs"
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
  gate_selftest_passes "prose, doc comments and a string literal naming the plumbing" plant_prose_only
  printf '%s selftest OK: passes a clean fixture and prose/doc/string mentions of the punning plumbing; fires on a downcast, and on one hidden behind a block comment; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
