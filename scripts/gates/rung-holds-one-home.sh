#!/usr/bin/env bash
# rung-holds-one-home.sh — a reading of whether one rung holds becomes
# its rung in ONE place, `geom_brep::ladder`'s `Rung::holds`, and a
# refusal stands by its own standing in one place, `Rung::refused`. ONE
# home; ci.yml's `lint` job runs every gate in this directory.
#
# THE INVARIANT. A ladder over candidates whose reader answers a `bool`
# over a refusal with a standing (`face_holds`, `corners_meet`,
# `strictly_inside`, a meeting) turns each reading into a rung the same
# way: holding answers, not holding passes, and a refusal is kept by its
# standing or aborts the ladder. PR 6574's review found that rung spelled
# four times beside the ladder, and the refusal arm once more on its own
# (`rung_or_abort(e.refusal(), e, Err)`). This gate reds a new spelling
# of either, and points at the home.
#
# WHAT IS READ: production sources (`crates/*/src`, less test-only mounts
# and `#[cfg(test)]` items) other than the home,
# `crates/geom-brep/src/ladder.rs`, in the statement view (comments and
# literals blanked, one record per statement, so a call rustfmt splits
# over lines is read whole), for:
#   * an arm `Ok(true) => Rung::Answers`, the hand-rolled `Rung::holds`;
#   * `rung_or_abort(X.refusal(), X, Err)` (or `X.1.refusal()`, or the
#     abort written `|e| Err(e)`), the hand-rolled `Rung::refused`: the
#     standing read off the payload, or a field of it, that the ladder
#     then keeps;
#   * the same with the standing read by path, `E::refusal(&X)`;
#   * `rung_or_abort` spelled out as a match, an arm
#     `Err(Aborts) => Rung::Aborts(Err(..))`.
# There is no register: each shape has a home that takes it.
#
# WHAT IT CANNOT SEE, each a pass where a copy exists:
#   * the two `Ok` arms in another order (`Ok(false) => Rung::Passes`
#     first, then `Ok(true)`), or one arm on a binding,
#     `Ok(b) => if b { .. } else { .. }`: that shape is every `if` on a
#     read `bool`, and the statement view cuts it at its `{`, so a line
#     grep cannot tell a rung from any other branch;
#   * a payload whose refusal is read through another binding
#     (`rung_or_abort(standing, e, Err)` with `standing` bound above);
#   * a `Rung::Answers` or `rung_or_abort` behind an alias or a `use … as`.
# A reviewer's, those.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-brep/src/ladder.rs
HOLDS_RE='Ok\(true\)[[:space:]]*=>[[:space:]]*(geom_brep::ladder::)?Rung::Answers'
# The abort the ladder hands back as the answer: `Err`, or `|e| Err(e)`.
ABORT='(Err|\|[a-z_][a-z0-9_]*\|[[:space:]]*Err\([a-z_][a-z0-9_]*\))[[:space:]]*,?[[:space:]]*\)'
REFUSED_RE='rung_or_abort\([[:space:]]*([A-Za-z_][A-Za-z0-9_]*)[A-Za-z0-9_.]*\.refusal\(\)[[:space:]]*,[[:space:]]*\1[[:space:]]*,[[:space:]]*'"$ABORT"
REFUSED_PATH_RE='rung_or_abort\([[:space:]]*[A-Za-z_][A-Za-z0-9_:]*::refusal\([[:space:]]*&[[:space:]]*([A-Za-z_][A-Za-z0-9_]*)[A-Za-z0-9_.]*[[:space:]]*\)[[:space:]]*,[[:space:]]*\1[[:space:]]*,[[:space:]]*'"$ABORT"
REFUSED_MATCH_RE='Err\((geom_brep::ladder::)?Aborts\)[[:space:]]*=>[[:space:]]*(geom_brep::ladder::)?Rung::Aborts\([[:space:]]*Err\('

gate() {
  gate_require_crate_sources
  gate_production_sources
  local -a files=()
  local f
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    [ "$f" = "$HOME_FILE" ] || files+=("$f")
  done
  local view hits
  view=$(gate_rust_code --skip-cfg-test --statements "${files[@]}")
  # One scan per shape, not one alternation: a back reference counts its
  # groups from the start of the whole pattern.
  hits=$(
    local re
    for re in "$HOLDS_RE" "$REFUSED_RE" "$REFUSED_PATH_RE" "$REFUSED_MATCH_RE"; do
      printf '%s\n' "$view" | gate_grep -E "$re"
    done
  )
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a bool reading or a refusal is turned into a rung by hand: use geom_brep::ladder's Rung::holds(reading, yes, standing) or Rung::refused(e, standing)"
    exit 1
  fi
  gate_ok "no bool reading or refusal is turned into a rung outside geom_brep::ladder"
}

plant_holds() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(r: Result<bool, E>) -> Rung<Result<(), E>, E> { match r { Ok(true) => Rung::Answers(Ok(())),\n Ok(false) => Rung::Passes,\n Err(e) => e.rung() } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_refused() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(e: (usize, E)) -> Rung<Result<(), (usize, E)>, (usize, E)> { rung_or_abort(e.1.refusal(), e, Err) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_refused_split() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(e: E) -> R {\n    rung_or_abort(\n        e.refusal(),\n        e,\n        Err,\n    )\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_refused_by_path() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(e: E) -> R { rung_or_abort(E::refusal(&e), e, |e| Err(e)) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_refused_as_a_match() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(e: E) -> R {\n    match e.refusal() {\n        Ok(r) => r.rung(e),\n        Err(Aborts) => Rung::Aborts(Err(e)),\n    }\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_in_the_home() {
  mkdir -p "$1/crates/geom-brep/src"
  printf 'pub fn holds(r: Result<bool, E>) -> Rung<Result<(), E>, E> { match r { Ok(true) => Rung::Answers(Ok(())),\n _ => Rung::Passes } }\n' \
    > "$1/$HOME_FILE"
}

plant_mapped_payload() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(e: E) -> Rung<Result<(), F>, F> { rung_or_abort(e.refusal(), wrap(e), Err) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_in_a_test_module() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod rows { fn f(r: Result<bool, E>) -> Rung<Result<(), E>, E> { match r { Ok(true) => Rung::Answers(Ok(())),\n _ => Rung::Passes } } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="turned into a rung by hand"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_holds
  gate_selftest_case "$want" plant_refused
  gate_selftest_case "$want" plant_refused_split
  gate_selftest_case "$want" plant_refused_by_path
  gate_selftest_case "$want" plant_refused_as_a_match
  gate_selftest_passes "the home's own spelling" plant_in_the_home
  gate_selftest_passes "a refusal whose payload is mapped" plant_mapped_payload
  gate_selftest_passes "a spelling inside a #[cfg(test)] module" plant_in_a_test_module
  printf '%s selftest OK: passes a clean tree, the home, a mapped payload and a test module; fires on a hand-rolled holds arm and on a hand-rolled refused (on one line, split over lines, by path with a closure abort, and as a match), and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
