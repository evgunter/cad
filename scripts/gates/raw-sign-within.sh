#!/usr/bin/env bash
# raw-sign-within.sh — a production `.sign_within(` outside the scalar
# impls and the `k_stats` doors is registered, per file.
#
# WHY. `k_stats::decide` and its siblings note the predicate's name for
# the recorder and log the verdict; a raw `sign_within` does neither, so
# at `Probe` its samples reach the K sink as `<unnamed>`, and an
# undecided read leaves no record in the verdict log. Only the probe sweep's
# corpus dump could see it, nightly: `geom_core::running::Rounded::divisor`
# read one raw for four days before it did. A new raw site now has to be
# registered here, beside why it is not a decision, or go through a door.
#
# EXEMPT: geom-core's `predicate.rs` (the trait and its doors),
# `k_stats.rs` (the funnel), and the scalar impls that delegate the trait
# (`interval.rs`, `dual.rs`, `sym.rs`).
#
# WHAT IT CANNOT SEE: a sign read through any other spelling — a
# helper that wraps `sign_within` under another name, `Decide::sign_*`
# methods other than this one, or a comparison of a `Real` against zero.
# It counts per file, so a site moved within a registered file, or one
# added beside one removed, passes.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

REGISTER=scripts/gates/raw-sign-within.counts
CALL_RE='\.sign_within[[:space:]]*\('
EXEMPT_RE='^crates/geom-core/src/(predicate|k_stats|interval|dual|sym)\.rs:'

gate() {
  gate_require_crate_sources
  gate_require_file "$REGISTER" "the raw sign_within register"
  gate_production_sources
  local view live want diff
  view=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}")
  live=$(printf '%s\n' "$view" | gate_grep -E "$CALL_RE" | gate_grep -vE "$EXEMPT_RE" \
    | gate_record_file | sort | uniq -c | awk '{ print $2, $1 }')
  want=$(awk '!/^[[:space:]]*(#|$)/ { print $1, $2 }' "$REGISTER" | sort)
  diff=$(diff <(printf '%s\n' "$live" | sed '/^$/d' | sort) <(printf '%s\n' "$want" | sed '/^$/d') || true)
  if [ -n "$diff" ]; then
    printf '%s\n' "$diff"
    gate_error "raw sign_within sites differ from $REGISTER ('<' is the tree, '>' the register): read the sign through a named k_stats door (decide, check_unlogged with its audit row), or register the file with why its read is no decision"
    exit 1
  fi
  gate_ok "every production sign_within outside the doors is registered"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# none\n' > "$1/$REGISTER"
}

plant_unregistered() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(x: f64) -> bool { x.sign_within(Band::least()).is_ok() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_stale_entry() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$REGISTER"
}

plant_registered() {
  plant_unregistered "$1"
  printf 'crates/planted/src/lib.rs 1\n' >> "$1/$REGISTER"
}

plant_in_a_door() {
  mkdir -p "$1/crates/geom-core/src"
  printf 'pub fn classify(x: f64) -> bool { x.sign_within(Band::least()).is_ok() }\n' \
    > "$1/crates/geom-core/src/k_stats.rs"
}

plant_in_a_test_module() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod rows { fn f(x: f64) -> bool { x.sign_within(b()).is_ok() } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_in_a_comment() {
  mkdir -p "$1/crates/planted/src"
  printf '// read it raw: x.sign_within(band)\npub fn f() {}\n' > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="raw sign_within sites differ"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_unregistered
  gate_selftest_case "$want" plant_stale_entry
  gate_selftest_passes "a registered raw site" plant_registered
  gate_selftest_passes "a read inside the k_stats funnel" plant_in_a_door
  gate_selftest_passes "a read inside a #[cfg(test)] module" plant_in_a_test_module
  gate_selftest_passes "a read named only in a comment" plant_in_a_comment
  printf '%s selftest OK: passes a clean tree, a registered site, the funnel, a test module and a comment; fires on an unregistered site and a stale entry, and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
