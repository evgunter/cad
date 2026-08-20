#!/usr/bin/env bash
# signed-zero-one-home.sh — the negative-zero flush has exactly ONE
# home. ONE home for the gate too; ci.yml's "signed-zero one home" step
# and local-scripts/ci-local.sh's discipline row both call this file.
#
# `crates/step-import/src/signed_zero.rs` asserts, in its own module
# doc, that it is the importer's only flush of `-0.0` to `+0.0`. That
# claim went unenforced through FOUR copies of the same three lines in
# one crate (S18's roll-up row), each landing green: the reader's
# `as_real`, two private `flush_zero`/`flush_zero_point` pairs in the
# recognition modules, and the home itself. A claim nothing checks is
# how copy five lands too, so this is the check.
#
# WHAT FIRES IT: a flush spelled anywhere under `crates/step-import/src`
# except the home — either form the four copies actually used, `x + 0.0`
# (add-to-flush) or `if x == 0.0 { 0.0 }` (branch-and-substitute).
#
# WHAT IT CANNOT SEE: a flush written without a literal `0.0` on either
# side (an `f64::from_bits` sign-bit mask, `x - x.min(x)`), one behind a
# generic or a trait method, and any copy outside this crate. The claim
# guarded is the one the module makes — one flush in THIS importer — not
# a workspace-wide uniqueness that is false by design
# (`pncad-py/src/py/doc.rs` folds `-0.0` for `__hash__`/`__eq__`
# consistency, a different rule with a different reason).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/step-import/src/signed_zero.rs
GATE_SCAN_NOUN='step-import source file'

gate() {
  # The home must exist (a renamed subject would make this green
  # forever), and the crate it guards must actually have sources.
  gate_require_file "$HOME_FILE"
  GATE_SCAN_FILES=$(find crates/step-import/src -type f -name '*.rs' | wc -l)
  if grep -rnE '\+[[:space:]]*0\.0|==[[:space:]]*0\.0[[:space:]]*\{[[:space:]]*0\.0' \
    crates/step-import/src \
    | grep -vE "^$HOME_FILE:" \
    | grep -vE ':[0-9]+:[[:space:]]*(//|/\*|\*)'; then
    gate_error "a negative-zero flush outside the sanctioned home ($HOME_FILE) — call crate::signed_zero instead of re-deriving it"
    exit 1
  fi
  gate_ok "the negative-zero flush lives only in $HOME_FILE"
}

# This gate's subject is one crate's sources, not `crates/*/src`.
gate_plant_clean() {
  mkdir -p "$1/crates/step-import/src"
  printf 'pub fn plus_zero_scalar(x: f64) -> f64 { if x == 0.0 { 0.0 } else { x } }\n' \
    > "$1/$HOME_FILE"
  printf 'pub fn mint(x: f64) -> f64 { crate::signed_zero::plus_zero_scalar(-x) }\n' \
    > "$1/crates/step-import/src/recognize.rs"
}

plant_add() {
  printf 'pub fn flush(x: f64) -> f64 { x + 0.0 }\n' \
    > "$1/crates/step-import/src/recognize.rs"
}

plant_branch() {
  printf 'pub fn flush(x: f64) -> f64 { if x == 0.0 { 0.0 } else { x } }\n' \
    > "$1/crates/step-import/src/recognize_curve.rs"
}

# Two known spellings, two cases: a gate that caught only the form the
# home uses would have missed the two copies this row actually had.
gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "outside the sanctioned home" plant_add
  gate_selftest_case "outside the sanctioned home" plant_branch
  printf '%s selftest OK: passes a clean fixture, fires on both planted spellings\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main "outside the sanctioned home" plant_add
