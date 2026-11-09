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
# THE REGISTERS, `<file> <count>` per line, `#` comments carrying each
# file's disposition. A file whose count is above its entry (or that has
# none) reds; one below reds too, asking for the entry to be lowered.
#   * `escalation-question-marks.counts`: a line holding a call whose
#     name starts `decide` and a `)?` after it, inside any loop body.
#   * `escalation-question-marks-callee.counts`: a line holding a `?` on
#     a call that reads `band` and no `decide*(` call (a callee that
#     decides inside it), inside a loop whose outermost loop has an early
#     answering exit (a `break`, a `return` of a value other than
#     `Ok(())` or an error, or a `return` of a bare binding,
#     `return found;`, which may carry either). A loop with no such exit reads every element,
#     so a `?` in it refuses what every element needs.
#
# WHAT IS READ: production sources (`crates/*/src`, less test-only mounts
# and `#[cfg(test)]` items), in the code view (comments and literals
# blanked). A loop is a `for`, `while` or `loop` whose header starts its
# line (after a label) or follows an `=` (`let x = loop {`), and ends its
# line with `{`.
#
# IT MATCHES NAMES, NOT DECISIONS: a call whose name holds `decide` is
# counted whether or not it is a band decision (`undecided(`,
# `strip_decided(`, `Rim::decided(` are), and the callee shape counts any
# fallible call that is handed the band. So the registers hold sites that
# are no band decision, each dispositioned as such, and miss the rest
# below.
#
# WHAT IT CANNOT SEE, each a pass where an early return exists:
#   * a callee read in band whose line names no `band` (the band reached
#     through a context struct or a captured reader);
#   * a search whose answer leaves the loop by assignment rather than by
#     `return` or `break` (`found = Some(..)` read after the loop): its
#     callee `?`s are not counted. Those read at PR 6574 are listed, each
#     with its disposition, on
#     `work/onecopy/question-mark-ladders-the-gate-registered.md`;
#   * a decision whose call and `?` sit on different lines;
#   * a loop written as an iterator chain (`try_for_each`, `map(…)
#     .collect::<Result<…>>()?`), or a header split over lines;
#   * a `?` ahead of a later read outside any loop (two reads in
#     sequence).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

REGISTER=scripts/gates/escalation-question-marks.counts
CALLEE_REGISTER=scripts/gates/escalation-question-marks-callee.counts

# Records on stdin, `file:LINE:text`; prints the file of each line that
# holds a `decide*(…)?` inside a loop body. Brace depth is tracked per
# file over the code view, so a brace in a comment or literal is not
# counted.
# A loop header: `for`, `while` or `loop` starting its line (after a
# label), or a loop that is an expression (`let x = loop {`), ending its
# line with `{`.
LOOP_HEADER='(^[[:space:]]*([a-z_]+:[[:space:]]*)?(for[[:space:]]|while[[:space:]]|loop([[:space:]]|[{]))|=[[:space:]]*([a-z_]+:[[:space:]]*)?(for[[:space:]]|while[[:space:]]|loop([[:space:]]|[{])))'

in_loop_question_marks() {
  gate_record_awk -v HEADER="$LOOP_HEADER" '
    {
      if (!gate_record_split($0)) { next }
      if (GR_FILE != file) { file = GR_FILE; depth = 0; nloops = 0 }
      t = GR_TEXT
      if (nloops > 0 && t ~ /decide[a-z_]*[(]/ && t ~ /[)][?]/) print GR_FILE
      if (t ~ HEADER && t ~ /[{][[:space:]]*$/) {
        loops[++nloops] = depth
      }
      opens = gsub(/[{]/, "{", t); closes = gsub(/[}]/, "}", t)
      depth += opens - closes
      while (nloops > 0 && depth <= loops[nloops]) nloops--
    }'
}

# Records on stdin; prints the file of each line, inside a loop body,
# that holds a `?` on a call reading `band` and no `decide*(` call (a
# callee that decides inside it), where the outermost loop around it
# has an early answering exit: a `return` of a value other than `Ok(())`
# or `Err(…)`, or a `break`. A loop with no such exit reads every
# element, so a `?` in it refuses what every element needs.
callee_question_marks() {
  gate_record_awk -v HEADER="$LOOP_HEADER" '
    function flush(   i, d, n, top, st, en, j, exits) {
      if (nl == 0) return
      d = 0; n = 0
      for (i = 1; i <= nl; i++) { outer[i] = 0 }
      for (i = 1; i <= nl; i++) {
        t = line[i]
        if (t ~ HEADER && t ~ /[{][[:space:]]*$/) { sd[++n] = d; sl[n] = i }
        d += gsub(/[{]/, "{", t) - gsub(/[}]/, "}", t)
        while (n > 0 && d <= sd[n]) {
          if (n == 1) {
            st = sl[1]; exits = 0
            for (j = st; j <= i; j++) {
              u = line[j]
              if (u ~ /(^|[^a-z_])break([^a-z_]|$)/) exits = 1
              if (u ~ /return[[:space:]]+(Some[(]|true|false|[A-Z][A-Za-z]*::|Ok[(])/ && u !~ /return[[:space:]]+Ok[(][(][)][)]/) exits = 1
              if (u ~ /return[[:space:]]+[a-z_][a-z0-9_]*[[:space:]]*;/) exits = 1
            }
            for (j = st; j <= i; j++) { outer[j] = exits ? 2 : 1 }
          }
          n--
        }
      }
      for (i = 1; i <= nl; i++) {
        t = line[i]
        if (outer[i] == 2 && t ~ /band/ && t ~ /[)][?]/ && t !~ /decide[a-z_]*[(]/) print cur
      }
      nl = 0
    }
    {
      if (!gate_record_split($0)) { next }
      if (GR_FILE != cur) { flush(); cur = GR_FILE }
      line[++nl] = GR_TEXT
    }
    END { flush() }'
}

gate() {
  gate_require_crate_sources
  gate_require_file "$REGISTER" "the escalation-question-mark register"
  gate_require_file "$CALLEE_REGISTER" "the callee question-mark register"
  gate_production_sources
  local view
  view=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}")
  local reg live want diff
  for reg in "$REGISTER:in_loop_question_marks" "$CALLEE_REGISTER:callee_question_marks"; do
    live=$(printf '%s\n' "$view" | "${reg#*:}" | sort | uniq -c | awk '{ print $2, $1 }')
    want=$(awk '!/^[[:space:]]*(#|$)/ { print $1, $2 }' "${reg%%:*}" | sort)
    diff=$(diff <(printf '%s\n' "$live" | sed '/^$/d' | sort) <(printf '%s\n' "$want" | sed '/^$/d') || true)
    if [ -n "$diff" ]; then
      printf '%s\n' "$diff"
      gate_error "early returns on a band reading in a loop differ from ${reg%%:*} ('<' is the tree, '>' the register): port the loop to geom_brep::ladder::climb, or register the site with why its ? is not a search's kept reading"
      exit 1
    fi
  done
  gate_ok "every production ? on a band reading inside a search loop is registered"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# none\n' > "$1/$REGISTER"
  printf '# none\n' > "$1/$CALLEE_REGISTER"
}

plant_callee_in_a_search() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(v: &[f64], band: f64) -> Result<bool, ()> {\n    for x in v {\n        if holds(*x, band)? {\n            return Ok(true);\n        }\n    }\n    Ok(false)\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_callee_returning_a_binding() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(v: &[f64], band: f64) -> Result<Option<f64>, ()> {\n    for x in v {\n        let found = pick(*x, band)?;\n        if found.is_some() {\n            return found;\n        }\n    }\n    Ok(None)\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_callee_in_a_fold() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(v: &[f64], band: f64) -> Result<f64, ()> {\n    let mut s = 0.0;\n    for x in v {\n        s += read(*x, band)?;\n    }\n    Ok(s)\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_in_an_expression_loop() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(mut x: f64) -> Result<f64, ()> {\n    let y = loop {\n        if decide(x)? {\n            break x;\n        }\n        x += 1.0;\n    };\n    Ok(y)\n}\n' \
    > "$1/crates/planted/src/lib.rs"
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
  local want="early returns on a band reading in a loop differ"
  gate_selftest_clean
  gate_selftest_case "$want" plant_unregistered
  gate_selftest_case "$want" plant_in_a_while_loop
  gate_selftest_case "$want" plant_stale_entry
  gate_selftest_case "$want" plant_callee_in_a_search
  gate_selftest_case "$want" plant_in_an_expression_loop
  gate_selftest_case "$want" plant_callee_returning_a_binding
  gate_selftest_passes "a callee read in a loop that answers nothing early" plant_callee_in_a_fold
  gate_selftest_passes "a registered site" plant_registered
  gate_selftest_passes "a ? outside any loop" plant_outside_a_loop
  gate_selftest_passes "a site inside a #[cfg(test)] module" plant_in_a_test_module
  printf '%s selftest OK: passes a clean tree, a registered site, one outside any loop and one in a test module; fires on an unregistered site in a for, a while or an expression loop body, on a callee read in band inside a loop that answers early (by a value or by a binding), and on a register entry the tree no longer carries; passes a callee read in a loop that answers nothing early\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
