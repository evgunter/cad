#!/usr/bin/env bash
# rung-holds-one-home.sh — a reading of whether one rung holds becomes
# its rung in ONE place, `geom_core::ladder`'s `Rung::holds`, and a
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
# `crates/geom-core/src/ladder.rs`, in the statement view (comments and
# literals blanked, one record per statement, so a call rustfmt splits
# over lines is read whole), for:
#   * an arm `Ok(true) => Rung::Answers`, the hand-rolled `Rung::holds`
#     (the path written `ladder::Rung`, `geom_core::ladder::Rung` or the
#     transitional `geom_brep::ladder::Rung`, the
#     body a `return`), in either order against its `Ok(false)` arm, since
#     a match is one record;
#   * `rung_or_abort(X.refusal(), X, Err)` (or `X.1.refusal()`; the abort
#     written `Result::Err`, `|e| Err(e)` or `|e: E| Err(e)`), the
#     hand-rolled `Rung::refused`: the standing read off the payload, or
#     a field of it, that the ladder then keeps;
#   * the same with the standing read by path, `E::refusal(&X)`;
#   * a refusal type's standing spelled as a match at the site, an arm
#     `Err(X::Escalated { .. }) => Kept::of(..)` keeping the escalation
#     by hand (read on the line view, since the statement view cuts at the
#     variant's braces): the type states its standing once, by its own
#     `refusal()`, and the site reads `Rung::refused(e, X::refusal)`;
#   * `rung_or_abort` spelled out as a match, an arm
#     `Err(Aborts) => Rung::Aborts(Err(x))` or `Err(_) => …`, with `x` a
#     bare binding: as in the call form, an abort that maps its payload
#     (`Rung::Aborts(Err(wrap(e)))`) is not the home's and passes.
# There is no register: each shape has a home that takes it.
#
# WHAT IT CANNOT SEE, each a pass where a copy exists:
#   * one arm on a binding, `Ok(b) => if b { .. } else { .. }`: that shape
#     is every `if` on a read `bool`, so a grep cannot tell a rung from
#     any other branch;
#   * a braced arm (`Ok(true) => { Rung::Answers(..) }`), a braced closure
#     abort (`|e| { Err(e) }`), or the standing read by
#     `if let Err(Aborts) = e.refusal() { .. }`: the statement view cuts a
#     record at every `{` and `}`, so the arm and its body are two records;
#   * the same table spelled through the ladder's one-step keep,
#     `Err(X::Escalated { diag, .. }) => Rung::kept(diag).map_kept(..)`:
#     the arm's braces split it on the statement view and its keep may
#     sit on the next line. Two sites spell it today, `boolean::slit`'s
#     holding (≈1163) and `chord_join::ring_side` (≈5484), both moved
#     onto their type's `refusal()` by PR 6576;
#   * a lone abort arm, `Err(e) => Rung::Aborts(Err(e))`: it is also how
#     a type with no standing is read (every refusal of it stops the
#     ladder), and the gate cannot tell the two apart. Seven sites spell it
#     today (`boolean::join` ×2, `offset_axial` ×2, `side_reader` ×2,
#     `stands`); each is a type with no `refusal()` or a reading every
#     refusal of which is the query's;
#   * a payload whose refusal is read through another binding
#     (`rung_or_abort(standing, e, Err)` with `standing` bound above);
#   * a `Rung::Answers` or `rung_or_abort` behind an alias or a `use … as`.
# A reviewer's, those.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-core/src/ladder.rs
# A path to the ladder's `Rung`: bare, `ladder::`, `geom_core::ladder::`
# or the transitional `geom_brep::ladder::`.
LADDER='((geom_core::|geom_brep::)?ladder::)?'
HOLDS_RE='Ok\(true\)[[:space:]]*=>[[:space:]]*(return[[:space:]]+)?'"$LADDER"'Rung::Answers'
# The abort the ladder hands back as the answer: `Err` (or `Result::Err`),
# or a closure `|e| Err(e)`, its binding typed or not.
ABORT='((Result::)?Err|\|[a-z_][a-z0-9_]*([[:space:]]*:[^|]*)?\|[[:space:]]*(Result::)?Err\([a-z_][a-z0-9_]*\))[[:space:]]*,?[[:space:]]*\)'
REFUSED_RE='rung_or_abort\([[:space:]]*([A-Za-z_][A-Za-z0-9_]*)[A-Za-z0-9_.]*\.refusal\(\)[[:space:]]*,[[:space:]]*\1[[:space:]]*,[[:space:]]*'"$ABORT"
REFUSED_PATH_RE='rung_or_abort\([[:space:]]*[A-Za-z_][A-Za-z0-9_:]*::refusal\([[:space:]]*&[[:space:]]*([A-Za-z_][A-Za-z0-9_]*)[A-Za-z0-9_.]*[[:space:]]*\)[[:space:]]*,[[:space:]]*\1[[:space:]]*,[[:space:]]*'"$ABORT"
REFUSED_MATCH_RE='Err\(('"$LADDER"'Aborts|_)\)[[:space:]]*=>[[:space:]]*(return[[:space:]]+)?'"$LADDER"'Rung::Aborts\([[:space:]]*(Result::)?Err\([[:space:]]*[a-z_][a-z0-9_]*[[:space:]]*\)[[:space:]]*\)'
# A refusal type's escalation kept by hand at the site.
TABLE_RE='Escalated[[:space:]]*\{[^}]*\}[[:space:]]*\)[[:space:]]*=>[[:space:]]*(return[[:space:]]+)?'"$LADDER"'Kept::of\('

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
  local lines
  lines=$(gate_rust_code --skip-cfg-test "${files[@]}")
  # One scan per shape, not one alternation: a back reference counts its
  # groups from the start of the whole pattern.
  hits=$(
    local re
    for re in "$HOLDS_RE" "$REFUSED_RE" "$REFUSED_PATH_RE" "$REFUSED_MATCH_RE"; do
      printf '%s\n' "$view" | gate_grep -E "$re"
    done
    printf '%s\n' "$lines" | gate_grep -E "$TABLE_RE"
  )
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a bool reading or a refusal is turned into a rung by hand: use geom_core::ladder's Rung::holds(reading, yes, standing) or Rung::refused(e, standing)"
    exit 1
  fi
  gate_ok "no bool reading or refusal is turned into a rung outside geom_core::ladder"
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

plant_holds_by_path_in_another_order() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(r: Result<bool, E>) -> R { match r { Ok(false) => ladder::Rung::Passes, Ok(true) => return geom_brep::ladder::Rung::Answers(Ok(())), Err(e) => e.rung() } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_refused_typed_closure() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(e: E) -> R { rung_or_abort(e.refusal(), e, |e: E| Result::Err(e)) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_refused_match_wildcard() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(e: E) -> R { match e.refusal() { Ok(r) => r.rung(e), Err(_) => return ladder::Rung::Aborts(Err(e)), } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_mapped_match() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(e: E) -> R { match e.refusal() { Ok(r) => r.rung(wrap(e)), Err(Aborts) => Rung::Aborts(Err(wrap(e))), } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_standing_table() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(r: Result<S, E>) -> R { match r { Ok(s) => Rung::Answers(Ok(s)),\n Err(E::Escalated { cause, check }) => Kept::of(cause).map(|cause| E::Escalated { cause, check }).into(),\n Err(e) => Rung::Aborts(Err(e)), } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_in_the_home() {
  mkdir -p "$(dirname "$1/$HOME_FILE")"
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
  gate_selftest_case "$want" plant_holds_by_path_in_another_order
  gate_selftest_case "$want" plant_refused_typed_closure
  gate_selftest_case "$want" plant_refused_match_wildcard
  gate_selftest_case "$want" plant_standing_table
  gate_selftest_passes "the home's own spelling" plant_in_the_home
  gate_selftest_passes "a refusal whose payload is mapped" plant_mapped_payload
  gate_selftest_passes "a match whose abort maps its payload" plant_mapped_match
  gate_selftest_passes "a spelling inside a #[cfg(test)] module" plant_in_a_test_module
  printf '%s selftest OK: passes a clean tree, the home, a mapped payload (in the call and the match form) and a test module; fires on a hand-rolled holds arm (in either order, by path, as a return) and on a hand-rolled refused (on one line, split over lines, by path, with a closure abort typed or not, and as a match on Aborts or _), and on a standing table kept by hand, and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
