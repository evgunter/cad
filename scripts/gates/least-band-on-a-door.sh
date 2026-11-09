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
# `normalize-by-hand.sh` reads them), where "the least band" is
# `Band::least()` (any path before `Band`) or a name the same file binds
# to it (`let exact = Band::least();`, `const`/`static` alike), found
# INSIDE THE CALL'S OWN ARGUMENT LIST, matched paren for paren across
# lines, so `decide(..).or(h(Band::least()))` is not a site and a `;`
# inside an argument (a block) does not end one:
#   * a free call of a door, `decide`, `decide_reported`, `decide_flagged`
#     or `check_unlogged`, bare, path-qualified (`k_stats::decide`), with
#     a turbofish (`decide::<T>(…)`), or under a name a `use … as` in the
#     same file gives it; a crate-local wrapper of the same name
#     (profile's `gate::decide`, `gate::check_unlogged`) included;
#   * a method-form `decide` (`.decide(…)`, or `Type::decide(…)` under a
#     capitalised qualifier: `Period::decide`'s mark walk) at the least
#     band OUTSIDE an `unlogged(…)` call that encloses it lexically.
#     Inside one it is F31's compound form, which `choose`'s doc names.
#
# NOT REFUSED, each for its reason:
#   * a reader that takes a band and decides several margins inside it
#     (`stable_azimuth`), and a `Band::least()` handed to a reader as its
#     band (`exact_band()`, a chart point, `principal_as`): not a
#     decision at the call;
#   * `sign_within(Band::least())`, which is `raw-sign-within.sh`'s.
#
# BLIND SPOTS, each left for its reason:
#   * a least band that reaches the door other than by its spelling or a
#     name bound in the same file: a function that returns it
#     (`exact_band()`), a parameter, a struct field. Following those is
#     data flow, which a lexer does not do;
#   * a name bound to the least band is matched across its FILE, not its
#     scope: shadowing it with another band cries wolf, and the direction
#     is the safe one;
#   * a wrapper of a door under another name, called at the least band:
#     that wrapper is a door the gate does not know. `flagged_census`
#     resolves forwarding wrappers for the unlogged doors; this gate reads
#     names;
#   * a method-form `decide` whose enclosing `unlogged` is in a caller
#     rather than lexically around it: refused, the safe direction, and
#     the call is moved inside, as `Period::near` and `Azimuth::near` do.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-core/src/k_stats.rs
HOME_SUBJECT='the home of the least-band doors, k_stats::prove and k_stats::choose'

# Records on stdin, `FILE:LINE:TEXT`; prints the record of each door call
# whose own argument list reaches the least band (header). A file is
# buffered whole and scanned only if its code calls a `least(` at all.
LEAST_ON_A_DOOR='
function is_ident(c) { return c ~ /[A-Za-z0-9_]/ }
function least_in(args,   rest, k, nm) {
  if (args ~ /Band[[:space:]]*::[[:space:]]*least[[:space:]]*[(][[:space:]]*[)]/) return 1
  for (nm in bound) {
    rest = args
    while ((k = index(rest, nm)) > 0) {
      if ((k == 1 || !is_ident(substr(rest, k - 1, 1))) && !is_ident(substr(rest, k + length(nm), 1))) return 1
      rest = substr(rest, k + length(nm))
    }
  }
  return 0
}
function enclosed_by_unlogged(code, at,   d, j, c, w) {
  d = 0
  for (j = at - 1; j >= 1; j--) {
    c = substr(code, j, 1)
    if (c == ")") d++
    else if (c == "(") {
      if (d > 0) { d--; continue }
      w = substr(code, 1, j - 1)
      sub(/[[:space:]]*$/, "", w)
      if (w ~ /(^|[^A-Za-z0-9_])unlogged$/) return 1
    }
  }
  return 0
}
function line_of(code, at,   pre) { pre = substr(code, 1, at); return gsub(/\n/, "", pre) + 1 }
function flush(   code, i, n, rest, nm, at, base, open, d, j, c, before, q, method, args, ln, m) {
  if (cur == "" || !mentions) { reset(); return }
  code = ""
  for (i = 1; i <= last; i++) code = code (i in text ? text[i] : "") "\n"
  split("", doors); split("", bound)
  doors["decide"]; doors["decide_reported"]; doors["decide_flagged"]; doors["check_unlogged"]
  rest = code
  while (match(rest, /(decide|decide_reported|decide_flagged|check_unlogged)[[:space:]]+as[[:space:]]+[A-Za-z_][A-Za-z0-9_]*/)) {
    m = substr(rest, RSTART, RLENGTH); sub(/.*[[:space:]]/, "", m); doors[m]
    rest = substr(rest, RSTART + RLENGTH)
  }
  rest = code
  while (match(rest, /(let|const|static)[[:space:]]+(mut[[:space:]]+)?[A-Za-z_][A-Za-z0-9_]*[[:space:]]*(:[^=;]*)?=[[:space:]]*([A-Za-z_][A-Za-z0-9_]*[[:space:]]*::[[:space:]]*)*Band[[:space:]]*::[[:space:]]*least[[:space:]]*[(][[:space:]]*[)]/)) {
    m = substr(rest, RSTART, RLENGTH)
    sub(/^(let|const|static)[[:space:]]+(mut[[:space:]]+)?/, "", m); sub(/[^A-Za-z0-9_].*/, "", m)
    bound[m]
    rest = substr(rest, RSTART + RLENGTH)
  }
  for (nm in doors) {
    base = 0; rest = code
    while ((i = index(rest, nm)) > 0) {
      at = base + i; base = at + length(nm) - 1; rest = substr(rest, i + length(nm))
      if (at > 1 && is_ident(substr(code, at - 1, 1))) continue
      j = at + length(nm)
      if (is_ident(substr(code, j, 1))) continue
      # the turbofish, then the opening paren
      while (substr(code, j, 1) ~ /[[:space:]]/) j++
      if (substr(code, j, 3) == "::<") {
        d = 0
        for (j += 2; j <= length(code); j++) {
          c = substr(code, j, 1)
          if (c == "<") d++
          else if (c == ">") { d--; if (d == 0) { j++; break } }
        }
        while (substr(code, j, 1) ~ /[[:space:]]/) j++
      }
      if (substr(code, j, 1) != "(") continue
      open = j
      # the argument list, paren for paren
      d = 0
      for (; j <= length(code); j++) {
        c = substr(code, j, 1)
        if (c == "(") d++
        else if (c == ")") { d--; if (d == 0) break }
      }
      args = substr(code, open + 1, j - open - 1)
      if (!least_in(args)) continue
      # what stands before the name: a method dot, a path, or nothing
      before = substr(code, 1, at - 1); sub(/[[:space:]]*$/, "", before)
      method = 0
      if (before ~ /\.$/) method = 1
      else if (before ~ /::$/) {
        q = substr(before, 1, length(before) - 2); sub(/[[:space:]]*$/, "", q)
        if (match(q, /[A-Za-z_][A-Za-z0-9_]*$/) && substr(q, RSTART, 1) ~ /[A-Z]/) method = 1
      }
      if (method && (nm != "decide" || enclosed_by_unlogged(code, at))) continue
      ln = line_of(code, at)
      print cur ":" ln ":" text[ln]
    }
  }
  reset()
}
function reset() { split("", text); last = 0; mentions = 0 }
{
  if (!gate_record_split($0)) { next }
  if (GR_FILE != cur) { flush(); cur = GR_FILE }
  text[GR_LINE + 0] = GR_TEXT; if (GR_LINE + 0 > last) last = GR_LINE + 0
  if (index(GR_TEXT, "least") && GR_TEXT ~ /(^|[^A-Za-z0-9_])least[[:space:]]*[(]/) mentions = 1
}
END { flush() }
'

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
  hits=$(gate_rust_code --skip-cfg-test "${files[@]}" \
    | gate_record_awk "$LEAST_ON_A_DOOR" \
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

plant_check_unlogged() {
  printf 'pub fn s(m: f64) -> R { check_unlogged("site", m, Band::least(), "F1") }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_flagged() {
  printf 'pub fn s(m: M) -> R { k_stats::decide_flagged("site", m, Band::least(), "F1") }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_turbofish() {
  printf 'pub fn s(m: M) -> R { decide::<f64>("site", m, Band::least()) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_renamed() {
  printf 'use geom_core::k_stats::{decide as read, prove};\npub fn s(m: M) -> R { read("site", m, Band::least()) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_bound() {
  printf 'pub fn s(m: M) -> R {\n    let exact = geom_core::Band::least();\n    decide("site", m, exact)\n}\n' >> "$1/crates/clean/src/lib.rs"
}

plant_block_argument() {
  printf 'pub fn s(m: M) -> R { decide("site", m, { let b = Band::least(); b }) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_method_outside_unlogged() {
  printf 'pub fn c(p: P, t: T) -> R { p.decide("mark", t, Margin::of, Band::least()) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_type_method_outside_unlogged() {
  printf 'pub fn c(p: P, t: T) -> R { Period::decide(p, "mark", t, Margin::of, Band::least()) }\n' >> "$1/crates/clean/src/lib.rs"
}

# The doors at their call sites, a method walk inside `unlogged`, a band
# handed to a reader, a decision on the caller's band with the least band
# read after its argument list closes, prose, a test module and fixture
# files are not the shape.
plant_not_sites() {
  {
    printf 'pub fn a(m: M) -> R { prove("site", m) }\n'
    printf 'pub fn b(m: M) -> P { choose("site", m, "F32") }\n'
    printf 'pub fn c(p: P, t: T) -> R {\n    unlogged("F31", || {\n        match p.decide("mark", t, Margin::of, Band::least()) { k => k }\n    })\n}\n'
    printf 'pub fn d(a: A) -> T { a.principal_as("row", Band::least()) }\n'
    printf 'pub fn e(m: M, band: Band) -> R { decide("site", m, band) }\n'
    printf 'pub fn f(m: M, band: Band) -> R { decide("site", m, band).or(h(Band::least())) }\n'
    printf 'pub fn g(m: M, least: Band) -> R { undecide("site", m, Band::least()); decide_more("site", Band::least()) }\n'
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
  gate_selftest_without_tool awk "the shared Rust reader exited"
  gate_selftest_case "$want" plant_logged_least
  gate_selftest_case "$want" plant_qualified
  gate_selftest_case "$want" plant_reported
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_case "$want" plant_check_unlogged
  gate_selftest_case "$want" plant_flagged
  gate_selftest_case "$want" plant_turbofish
  gate_selftest_case "$want" plant_renamed
  gate_selftest_case "$want" plant_bound
  gate_selftest_case "$want" plant_block_argument
  gate_selftest_case "$want" plant_method_outside_unlogged
  gate_selftest_case "$want" plant_type_method_outside_unlogged
  gate_selftest_passes "the doors, a method walk inside unlogged, a band handed to a reader, the caller's band with the least band past its argument list, other verbs, prose, a test module, fixture files and the home" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "$HOME_FILE"
  printf '%s selftest OK: passes a clean fixture carrying the home, and prove, choose, a method walk inside unlogged, a band handed to a reader, the caller'"'"'s band with the least band past its argument list, other verbs, prose, a test module and fixture files; fires on a door at the least band bare, qualified, kept reported, flagged, unlogged by check_unlogged, with a turbofish, renamed on import, bound to a name first, inside a block argument and wrapped across lines, and on a method-form decide outside unlogged; and stays RED when grep or awk cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
