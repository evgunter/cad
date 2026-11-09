#!/usr/bin/env bash
# least-band-on-a-door.sh — a decision on the least band is read through
# its door: `k_stats::prove` for a proof's inequality (logged), or
# `k_stats::choose` for a heuristic's choice (unlogged, under its ledger
# row), never `decide(.., Band::least())` by hand. ci.yml's `lint` job
# runs every gate in this directory.
#
# THE RULE (`work/onecopy/a-heuristics-choice-has-no-door-of-its-own.md`).
# A margin read on the sign alone is one of two things. Either a reading
# rests on it, a proof's inequality, and it is logged (`prove`). Or no
# reading does, a choice between sound candidates, and it is taken off the
# log under a ledger row that argues so (`choose`, checked by
# `geom-core`'s `flagged_census`). The logged `decide` with the least band
# spelled at the call is neither by name. That is the default a choice
# fell into three times (F31, F32, F33), each escalating at `Interval`
# with a cue no box answers until a lane argued it out. With the band off
# the logged door's callers, the next one lands at a door by construction.
# The doors' home, `crates/geom-core/src/k_stats.rs`, is exempt.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (comments and
# string literals out, `#[cfg(test)]` modules and test-only mounts out,
# and files named `test_support*`, `*_rows.rs` or `*_fuzz.rs` out, as
# `normalize-by-hand.sh` reads them):
#   * a free call `decide(…)` or `decide_reported(…)`, a crate-local
#     wrapper of the same name included, whose argument list reaches
#     `Band::least()`, whitespace and rustfmt's wrap across lines allowed.
#
# NOT REFUSED, each for its reason:
#   * a method `.decide(…)` (`Period::decide`'s mark walk), and a reader
#     that takes a band and decides several margins inside it
#     (`stable_azimuth`): F31's compound form, run inside
#     `k_stats::unlogged("F31", ..)` at the least band, which `choose`'s
#     doc names;
#   * a `Band::least()` handed to a reader as its band (`exact_band()`,
#     a chart point, `principal_as`): not a decision at the call;
#   * `sign_within(Band::least())`, which is `raw-sign-within.sh`'s.
#
# BLIND SPOTS. The least band bound to a name first (`let exact =
# Band::least(); … decide(.., exact)`), and a wrapper under another name.
# The second pass for the first is
# `grep -rnE 'let \w+ = (geom_core::)?Band::least\(\)' crates/*/src`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-core/src/k_stats.rs
HOME_SUBJECT='the home of the least-band doors, k_stats::prove and k_stats::choose'

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$HOME_SUBJECT" "$HOME_FILE"
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
    | gate_grep -E "(^|[^A-Za-z0-9_.])decide(_reported)?[[:space:]]*\\([^;]*Band::least[[:space:]]*\\([[:space:]]*\\)" \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a decision on the least band taken through the logged decide (above): a proof's inequality is k_stats::prove(name, margin), a heuristic's choice k_stats::choose(name, margin, ledger_row)"
    exit 1
  fi
  gate_ok "no decision on the least band outside k_stats::prove and k_stats::choose ($GATE_SCAN_FILES files)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}"
  printf 'pub fn prove(n: &str, m: M) -> R { decide(n, m, Band::least()) }\n' > "$1/$HOME_FILE"
}

plant_logged_least() {
  printf 'pub fn s(m: M) -> R { decide("site", m, Band::least()) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_qualified() {
  printf 'pub fn s(m: M) -> R { geom_core::k_stats::decide("site", m, geom_core::Band::least()) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_reported() {
  printf 'pub fn s(m: M) -> R { decide_reported("site", m, Band::least()) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_wrapped() {
  printf 'pub fn s(m: M) -> R {\n    decide(\n        "site",\n        Margin::of(m),\n        Band::least(),\n    )\n}\n' >> "$1/crates/clean/src/lib.rs"
}

# The doors at their call sites, a method walk, a band handed to a reader,
# a decision on the caller's band, prose, a test module and fixture files
# are not the shape.
plant_not_sites() {
  {
    printf 'pub fn a(m: M) -> R { prove("site", m) }\n'
    printf 'pub fn b(m: M) -> P { choose("site", m, "F32") }\n'
    printf 'pub fn c(p: P, t: T) -> R { p.decide("mark", t, Margin::of, Band::least()) }\n'
    printf 'pub fn d(a: A) -> T { a.principal_as("row", Band::least()) }\n'
    printf 'pub fn e(m: M, band: Band) -> R { decide("site", m, band) }\n'
    printf '// decide("site", m, Band::least()) by hand\npub const S: &str = "decide(n, m, Band::least())";\n'
    printf '#[cfg(test)]\nmod tests {\n    fn u(m: M) -> R { decide("site", m, Band::least()) }\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn u(m: M) -> R { decide("site", m, Band::least()) }\n' > "$1/crates/clean/src/test_support.rs"
  printf 'pub fn u(m: M) -> R { decide("site", m, Band::least()) }\n' > "$1/crates/clean/src/walk_fuzz.rs"
}

gate_selftest() {
  local want="a decision on the least band taken through the logged decide"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_logged_least
  gate_selftest_case "$want" plant_qualified
  gate_selftest_case "$want" plant_reported
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_passes "the doors, a method walk, a band handed to a reader, the caller's band, prose, a test module, fixture files and the home" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "$HOME_FILE"
  printf '%s selftest OK: passes a clean fixture carrying the home, and prove, choose, a method walk, a band handed to a reader, the caller'"'"'s band, prose, a test module and fixture files; fires on a logged decide at the least band bare, qualified, kept reported and wrapped across lines; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
