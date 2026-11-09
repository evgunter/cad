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
# `normalize-by-hand.sh` reads them), whitespace and rustfmt's wrap
# across lines allowed throughout:
#   * a closure that destructures a run and drops its multiplicity, passed
#     to any adapter of `runs(…)` / `knot_runs(…)`, method or UFCS
#     (`Knots::runs(k)`): `|(v, _)|`, `|(v, _m)|`, `|(v, ..)|`, `&`-borrowed,
#     `move`, typed (`|(v, _): (f64, usize)|`), whatever its body (`v`, a
#     call on `v`, a block), under `map`, `filter_map`, `for_each`, `any`,
#     `all`, `find`, `filter`, `flat_map` or `inspect`;
#   * `for (v, _) in …runs(…)` (the same patterns);
#   * `let (v, _) = …runs(…)….unzip()`;
#   * `knots().to_vec()` followed, within the next four statements of the
#     same file, by a `dedup()`.
#
# BLIND SPOTS. A `.map(|r| r.0)` that indexes the pair rather than
# destructuring it; the runs collected first and projected later, or a
# `to_vec` and its `dedup` more than four statements apart; a helper fn
# taking the runs as an argument; `interior_knot_runs()` (interior values
# only, a different set, and `InteriorKnot`-typed, so never this door's).
# The second pass for the first is
# `grep -rnE 'runs\([^()]*\)[^;]*\.map\(\|\w+\|[^;]*\.0\b' crates/*/src`.
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
  local runs='(^|[^A-Za-z0-9_])(knot_)?runs[[:space:]]*\([^()]*\)'
  local dropped='&?[[:space:]]*\([[:space:]]*&?[[:space:]]*[A-Za-z_][A-Za-z0-9_]*[[:space:]]*,[[:space:]]*(_[A-Za-z0-9_]*|\.\.)[[:space:]]*\)'
  local adapter='(map|filter_map|for_each|any|all|find|filter|flat_map|inspect)'
  local closure="$runs[^;]*\.$adapter[[:space:]]*\([[:space:]]*(move[[:space:]]+)?\|[[:space:]]*$dropped"
  local for_loop="(^|[^A-Za-z0-9_])for[[:space:]]+$dropped[[:space:]]+in[[:space:]][^;{]*(knot_)?runs[[:space:]]*\("
  local unzipped="(^|[^A-Za-z0-9_])let[[:space:]]+$dropped[^=;]*=[^;]*(knot_)?runs[[:space:]]*\([^;]*\.unzip[[:space:]]*\("
  local stream hits
  stream=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")|$(gate_record_anchor "$TWIN_FILE")" || true)
  hits=$(printf '%s\n' "$stream" | gate_grep -E "$closure|$for_loop|$unzipped" || true)
  hits+=$(printf '%s\n' "$stream" | awk -F: '
    /knots[[:space:]]*\([[:space:]]*\)[[:space:]]*\.[[:space:]]*to_vec[[:space:]]*\(/ { held = $0; file = $1; left = 4; next }
    held != "" && $1 == file && left > 0 && /\.[[:space:]]*dedup[[:space:]]*\(/ { print held; held = ""; next }
    held != "" { left--; if (left <= 0 || $1 != file) held = "" }
  ')
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

plant_named_ignore() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, _m)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_rest() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, ..)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_typed() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(v, _): (f64, usize)| v).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_called() {
  printf 'pub fn b(k: &K) -> Vec<P> { k.runs().map(|(t, _)| eval(t)).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_block() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().map(|(t, _)| { t }).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_filter_map() {
  printf 'pub fn b(k: &K) -> Vec<f64> { k.runs().filter_map(|(t, _)| f(t)).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_ufcs() {
  printf 'pub fn b(k: &K) -> Vec<f64> { Knots::runs(k).map(|(t, _)| t).collect() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_for_loop() {
  printf 'pub fn b(k: &K) { for (t, _) in k.knot_runs() { use_it(t); } }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_unzip() {
  printf 'pub fn b(k: &K) -> Vec<f64> { let (t, _): (Vec<f64>, Vec<usize>) = k.runs().unzip(); t }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_to_vec_dedup() {
  printf 'pub fn b(kv: &KV) -> Vec<f64> {\n    let mut b: Vec<f64> = kv.knots().to_vec();\n    b.sort_by(f64::total_cmp);\n    b.dedup();\n    b\n}\n' >> "$1/crates/clean/src/lib.rs"
}

# A multiplicity read, the other element, prose, a test module, a
# fixture file and the homes themselves are not the shape.
plant_not_sites() {
  {
    printf 'pub fn a(k: &K) -> Vec<usize> { k.runs().map(|(_, m)| m).collect() }\n'
    printf 'pub fn e(k: &K) -> usize { k.runs().map(|(v, m)| m).sum() }\n'
    printf 'pub fn f(kv: &KV) -> Vec<f64> { kv.knots().to_vec() }\n'
    printf 'pub fn c(k: &K) -> Vec<f64> { k.runs().map(|(v, m)| v * m).collect() }\n'
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
  gate_selftest_case "$want" plant_named_ignore
  gate_selftest_case "$want" plant_rest
  gate_selftest_case "$want" plant_typed
  gate_selftest_case "$want" plant_called
  gate_selftest_case "$want" plant_block
  gate_selftest_case "$want" plant_filter_map
  gate_selftest_case "$want" plant_ufcs
  gate_selftest_case "$want" plant_for_loop
  gate_selftest_case "$want" plant_unzip
  gate_selftest_case "$want" plant_to_vec_dedup
  gate_selftest_passes "a multiplicity read, other elements, prose, a test module, fixture files and the homes" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "$HOME_FILE" "$TWIN_FILE"
  printf '%s selftest OK: passes a clean fixture carrying both homes, and a multiplicity read, other elements, prose, a test module and fixture files; fires on runs() and knot_runs() mapped to their breakpoint, borrowed, through value_channel and wrapped across lines, on a named or rest ignore, a typed closure, a call or block body, filter_map, UFCS, a for loop, an unzip, and knots().to_vec() then dedup; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
