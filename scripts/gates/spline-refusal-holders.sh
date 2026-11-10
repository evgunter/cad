#!/usr/bin/env bash
# spline-refusal-holders.sh — a holder of a bare spline-layer refusal is
# registered as one whose input its caller built. ONE home; ci.yml's
# `lint` job runs every gate in this directory.
#
# THE RULE. `geom_core::spline::SplineError` and `KnotAlgebraError`,
# and the fitting stack's `geom::FitError` and `geom_core::linalg::lsq::
# LsqError`, end on a recourse addressed to whoever supplied the input
# or asked for the operation. A holder whose input the kernel built (or adopted from
# a file) holds `geom_core::spline::Built<E>` instead, set at the
# `map_err` of the kernel's own build: its `Display` is the refused fact
# alone, so the caller-facing recourse cannot reach a user who never
# wrote that input, and the holder ends it at its own door. A holder
# that keeps the bare type says with a row in the register below why
# that recourse cannot reach a user who never wrote the input: its
# caller built the input, or the holder renders its own sentence and
# never the payload's text.
#
# WHAT THE MATCHER SEES, in production code with test modules skipped,
# each type bare or through its module path (`spline::`,
# `geom_core::spline::`, `geom::curves::fit::`, `geom_core::linalg::lsq::`):
#  - a field on one line, `name: SplineError,`;
#  - an element of a one-line tuple arm or tuple struct, alone or among
#    others: `Name(SplineError),`, `Name(SplineError, usize),`,
#    `pub struct C(pub SplineError);`.
# WHAT IT CANNOT SEE: a holder through a type alias, a generic parameter
# or a container (`Box<SplineError>`, `Option<SplineError>`, `Vec<…>`), a
# wrapper enum of its own, a field or tuple declared over several lines,
# or one that stringifies the refusal (`String`). Nor does the register
# say WHICH arm of a file is the registered one: a second bare holder in
# a registered file raises its count and fails, but a swap within one
# file does not, and is held in review.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

REGISTER=scripts/gates/spline-refusal-holders.counts
TYPE_RE='((pncad::)?(geom_core|geom)::)?((spline|curves|fit|linalg|lsq)::)*(SplineError|KnotAlgebraError|FitError|LsqError)'
VIS_RE='(pub(\([a-z]+\))?[[:space:]]+)?'
FIELD_RE="^[^:]*:[0-9]+:[[:space:]]*${VIS_RE}[a-z_][a-z0-9_]*:[[:space:]]*${TYPE_RE},\$"
TUPLE_RE="[A-Z][A-Za-z0-9]*\\(([^()]*,[[:space:]]*)?${VIS_RE}${TYPE_RE}[[:space:]]*(,[^()]*)?\\)"
HOLDER_RE="${FIELD_RE}|${TUPLE_RE}"

gate() {
  gate_require_crate_sources
  gate_require_file "$REGISTER" "the bare spline-refusal holder register"
  gate_production_sources
  local view
  view=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}")
  local live want
  live=$(printf '%s\n' "$view" | gate_grep -E "$HOLDER_RE" | gate_record_file | sort | uniq -c \
    | awk '{ print $2, $1 }')
  want=$(awk '!/^[[:space:]]*(#|$)/ { print $1, $2 }' "$REGISTER" | sort)
  local diff
  diff=$(diff <(printf '%s\n' "$live" | sed '/^$/d' | sort) <(printf '%s\n' "$want" | sed '/^$/d') || true)
  if [ -n "$diff" ]; then
    printf '%s\n' "$diff"
    gate_error "bare spline-refusal holders differ from $REGISTER ('<' is the tree, '>' the register): hold geom_core::spline::Built where the kernel built the input, or register the holder with why the payload's caller recourse cannot reach the user"
    exit 1
  fi
  gate_ok "every bare spline-layer or fitting-stack refusal holder is registered with why its payload's caller recourse cannot reach the user"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# none\n' > "$1/$REGISTER"
}

plant_unregistered() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub enum E {\n    Structure(geom_core::spline::SplineError),\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_unregistered_field() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub enum E {\n    Seat {\n        source: KnotAlgebraError,\n    },\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_multi_field_arm() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub enum E {\n    Y(SplineError, usize),\n}\n' > "$1/crates/planted/src/lib.rs"
}

plant_tuple_struct() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub struct C(pub geom_core::spline::SplineError);\n' > "$1/crates/planted/src/lib.rs"
}

plant_built_in_a_tuple() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub enum E {\n    Y(Built<SplineError>, usize),\n}\npub fn f() -> Result<(), SplineError> { Err(SplineError::X) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_stale_entry() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$REGISTER"
}

plant_registered() {
  plant_unregistered "$1"
  printf 'crates/planted/src/lib.rs 1 the caller supplied it\n' >> "$1/$REGISTER"
}

plant_built() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub enum E {\n    Structure(Built<SplineError>),\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_in_a_test_module() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod rows {\n    pub enum E {\n        Structure(SplineError),\n    }\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="bare spline-refusal holders differ"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_unregistered
  gate_selftest_case "$want" plant_unregistered_field
  gate_selftest_case "$want" plant_multi_field_arm
  gate_selftest_case "$want" plant_tuple_struct
  gate_selftest_case "$want" plant_stale_entry
  gate_selftest_passes "a registered holder" plant_registered
  gate_selftest_passes "a Built holder" plant_built
  gate_selftest_passes "a Built element of a tuple, and a bare return type" plant_built_in_a_tuple
  gate_selftest_passes "a holder inside a #[cfg(test)] module" plant_in_a_test_module
  printf '%s selftest OK: passes a clean tree, a registered holder, a Built one and one in a test module; fires on an unregistered tuple arm, multi-field tuple arm, tuple struct and field, and on a register entry the tree no longer carries, and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
