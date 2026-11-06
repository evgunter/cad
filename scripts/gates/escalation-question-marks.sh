#!/usr/bin/env bash
# escalation-question-marks.sh — every production `?` on a band
# decision inside a loop body is registered, per file, at a pinned count.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE INVARIANT. A loop that reads candidates until one decides, and
# returns the first reading the band does not decide by `?`, refuses
# where a reading it has not read yet would answer definitely:
# `splitting::pieced_arc::PiecedArc::contact` refused an end in band
# where a piece placed the point on the arc. The rule is stated once, in
# `geom_brep::ladder::climb`, which keeps what it passes over and ranks
# it. `escalation-discards.sh` sees the loop that drops a refusal; this
# gate sees the loop that returns one early. A new site reds until the
# register names why its `?` is not a search's kept reading: most are,
# rightly, a reading every later one needs (a premise), or a loop with
# no answer to find (every element is needed, and the first refusal is
# as good as any).
#
# THE REGISTER, `escalation-question-marks.counts`: `<file> <count>` per
# line, `#` comments carrying each file's disposition. A file whose count
# is above its entry (or that has none) reds; one below reds too, asking
# for the entry to be lowered.
#
# WHAT IS READ: production sources (`crates/*/src`, less test-only mounts
# and `#[cfg(test)]` items), in the code view (comments and literals
# blanked), for a line holding a call whose name starts `decide` and a
# `)?` after it, inside the braces of a `for`, `while` or `loop` whose
# header ends its line with `{`.
#
# WHAT IT CANNOT SEE, each a pass where an early return exists:
#   * a `?` on a call that decides inside its callee
#     (`CarrierLoop::place_off`'s `edge.contact(…)?` was one). That is not
#     line-greppable; the callee's error type stating its standing
#     (`geom_brep::ladder`'s module docs) is what makes the call a
#     visible kept reading;
#   * a decision whose call and `?` sit on different lines;
#   * a loop written as an iterator chain (`try_for_each`, `map(…)
#     .collect::<Result<…>>()?`), or a header split over lines;
#   * a `?` ahead of a later read outside any loop (two reads in
#     sequence).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

REGISTER=scripts/gates/escalation-question-marks.counts

# Records on stdin, `file:LINE:text`; prints the file of each line that
# holds a `decide*(…)?` inside a loop body. Brace depth is tracked per
# file over the code view, so a brace in a comment or literal is not
# counted.
in_loop_question_marks() {
  gate_record_awk '
    {
      if (!gate_record_split($0)) { next }
      if (GR_FILE != file) { file = GR_FILE; depth = 0; nloops = 0 }
      t = GR_TEXT
      if (nloops > 0 && t ~ /decide[a-z_]*[(]/ && t ~ /[)][?]/) print GR_FILE
      if (t ~ /^[[:space:]]*([a-z_]+:[[:space:]]*)?(for[[:space:]]|while[[:space:]]|loop([[:space:]]|[{]))/ && t ~ /[{][[:space:]]*$/) {
        loops[++nloops] = depth
      }
      opens = gsub(/[{]/, "{", t); closes = gsub(/[}]/, "}", t)
      depth += opens - closes
      while (nloops > 0 && depth <= loops[nloops]) nloops--
    }'
}

gate() {
  gate_require_crate_sources
  gate_require_file "$REGISTER" "the escalation-question-mark register"
  gate_production_sources
  local view
  view=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}")
  local live want
  live=$(printf '%s\n' "$view" | in_loop_question_marks | sort | uniq -c | awk '{ print $2, $1 }')
  want=$(awk '!/^[[:space:]]*(#|$)/ { print $1, $2 }' "$REGISTER" | sort)
  local diff
  diff=$(diff <(printf '%s\n' "$live" | sed '/^$/d' | sort) <(printf '%s\n' "$want" | sed '/^$/d') || true)
  if [ -n "$diff" ]; then
    printf '%s\n' "$diff"
    gate_error "early returns on a band decision in a loop differ from $REGISTER ('<' is the tree, '>' the register): port the loop to geom_brep::ladder::climb, or register the site with why its ? is not a search's kept reading"
    exit 1
  fi
  gate_ok "every production ? on a band decision inside a loop is registered"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# none\n' > "$1/$REGISTER"
}

plant_unregistered() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(v: &[f64]) -> Result<bool, ()> {\n    for x in v {\n        if decide(*x)? {\n            return Ok(true);\n        }\n    }\n    Ok(false)\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_outside_a_loop() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(x: f64) -> Result<bool, ()> {\n    let a = decide(x)?;\n    for _ in 0..1 {\n    }\n    Ok(a && decide(x)?)\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_in_a_while_loop() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(mut x: f64) -> Result<(), ()> {\n    while x > 0.0 {\n        x -= decide_step(x)?;\n    }\n    Ok(())\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_stale_entry() {
  printf 'crates/clean/src/lib.rs 1\n' >> "$1/$REGISTER"
}

plant_registered() {
  plant_unregistered "$1"
  printf 'crates/planted/src/lib.rs 1\n' >> "$1/$REGISTER"
}

plant_in_a_test_module() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod rows {\n    fn f(v: &[f64]) -> Result<(), ()> {\n        for x in v {\n            decide(*x)?;\n        }\n        Ok(())\n    }\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="early returns on a band decision in a loop differ"
  gate_selftest_clean
  gate_selftest_case "$want" plant_unregistered
  gate_selftest_case "$want" plant_in_a_while_loop
  gate_selftest_case "$want" plant_stale_entry
  gate_selftest_passes "a registered site" plant_registered
  gate_selftest_passes "a ? outside any loop" plant_outside_a_loop
  gate_selftest_passes "a site inside a #[cfg(test)] module" plant_in_a_test_module
  printf '%s selftest OK: passes a clean tree, a registered site, one outside any loop and one in a test module; fires on an unregistered site in a for or a while body, and on a register entry the tree no longer carries\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
