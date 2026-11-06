#!/usr/bin/env bash
# breakpoints-by-hand.sh — a spline's distinct breakpoints are read
# through `breakpoints()`, never spelled `runs().map(|(v, _)| v)` by hand.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (`work/onecopy/a-knot-vectors-breakpoints-are-collected-four-times-with-no-door.md`).
# `geom_core::spline::Knots::breakpoints` (`crates/geom-core/src/spline/knot_values.rs`)
# and `KnotVector::breakpoints` (`crates/geom-core/src/spline/knots.rs`)
# are the one statement of where a spline's Bézier pieces begin and end:
# each breakpoint once, ascending, clamps included. A site that drops a
# run's multiplicity by hand is a second statement of it, and the shape a
# fifth site copies from; one that reaches for `values()` (every knot,
# repeated) instead doubles a piece boundary at a multiple knot.
# Production code takes the doors; the two home files are exempt.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (comments and
# string literals out, `#[cfg(test)]` modules and test-only mounts out,
# and files named `test_support*`, `*_rows.rs` or `*_fuzz.rs` out, as
# `normalize-by-hand.sh` reads them):
#   * `runs().map(|(X, _)| X …` and `knot_runs().map(|(X, _)| X …`, the
#     pattern optionally `&`-borrowed, whitespace and rustfmt's wrap
#     across lines allowed.
#
# BLIND SPOTS. A run destructured in a `for` loop (`for (v, _) in
# knots.runs()`), a `.map(|r| r.0)`, or the runs collected first and
# projected later; `interior_knot_runs()` (interior values only, a
# different set, and `InteriorKnot`-typed). The second pass for the
# first two is `grep -rnE 'for \(\w+, _\) in .*runs\(\)|runs\(\)\.map\(\|\w+\| \w+\.0\)' crates/*/src`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-core/src/spline/knot_values.rs
TWIN_FILE=crates/geom-core/src/spline/knots.rs
HOME_SUBJECT='the home of a spline'"'"'s breakpoints, Knots::breakpoints'

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$HOME_SUBJECT" "$HOME_FILE" "$TWIN_FILE"
  local files=() f
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    case "${f##*/}" in
      test_support*|*_rows.rs|*_fuzz.rs) ;;
      *) files+=("$f") ;;
    esac
  done
  GATE_SCAN_FILES=${#files[@]}
  local hits
  hits=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -E "(^|[^A-Za-z0-9_])(knot_)?runs\\(\\)[[:space:]]*\\.map\\([[:space:]]*\\|[[:space:]]*&?\\([[:space:]]*([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*,[[:space:]]*_[[:space:]]*\\)[[:space:]]*\\|[[:space:]]*\\3([^A-Za-z0-9_]|\$)" \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")|$(gate_record_anchor "$TWIN_FILE")")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a spline's breakpoints collected by hand (above): read them as knots.breakpoints() (Knots::breakpoints / KnotVector::breakpoints), the one home"
    exit 1
  fi
  gate_ok "no spline breakpoints collected by hand outside Knots::breakpoints ($GATE_SCAN_FILES files)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}"
  printf 'pub fn breakpoints(k: &K) -> impl Iterator<Item = f64> + %s_ { k.runs().map(|(v, _)| v) }\n' "'" > "$1/$HOME_FILE"
  printf 'pub fn breakpoints(k: &K) -> impl Iterator<Item = f64> + %s_ { k.knot_runs().map(|(k, _)| k) }\n' "'" > "$1/$TWIN_FILE"
}

plant_runs() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_knot_runs() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.knot_runs().map(|(x, _)| x).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_borrowed() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|&(v, _)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_channel() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v.value_channel()).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_wrapped() {
  printf 'pub fn b(k: &K) -> Vec<f64> {\n    k.runs()\n        .map(|(value, _)| value)\n        .collect()\n}\n' >> "$1/crates/clean/src/lib.rs"
}

# A multiplicity read, the other element, prose, a test module, a
# fixture file and the homes themselves are not the shape.
plant_not_sites() {
  {
    printf 'pub fn a(k: &K) -> Vec<usize> { k.runs().map(|(_, m)| m).collect() }\n'
    printf 'pub fn c(k: &K) -> Vec<f64> { k.runs().map(|(v, m)| v * m).collect() }\n'
    printf 'pub fn d(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| w).collect() }\n'
    printf '// k.runs().map(|(v, _)| v) by hand\npub const S: &str = "k.runs().map(|(v, _)| v)";\n'
    printf '#[cfg(test)]\nmod tests {\n    fn u(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v).collect() }\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn u(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v).collect() }\n' > "$1/crates/clean/src/test_support.rs"
  printf 'pub fn u(k: &K) -> Vec<f64> { k.runs().map(|(v, _)| v).collect() }\n' > "$1/crates/clean/src/walk_fuzz.rs"
}

gate_selftest() {
  local want="a spline's breakpoints collected by hand"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_runs
  gate_selftest_case "$want" plant_knot_runs
  gate_selftest_case "$want" plant_borrowed
  gate_selftest_case "$want" plant_channel
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_passes "a multiplicity read, other elements, prose, a test module, fixture files and the homes" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "$HOME_FILE" "$TWIN_FILE"
  printf '%s selftest OK: passes a clean fixture carrying both homes, and a multiplicity read, other elements, prose, a test module and fixture files; fires on runs() and knot_runs() mapped to their breakpoint, borrowed, through value_channel and wrapped across lines; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
