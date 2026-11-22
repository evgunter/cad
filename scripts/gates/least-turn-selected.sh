#!/usr/bin/env bash
# least-turn-selected.sh — the least turn with no conditioning has the
# callers it names, and no others. ONE home; ci.yml's `lint` job runs
# every gate in this directory.
#
# THE RULE (`crates/sweep/README.md` `[sweep.frame]`; the geom-core
# README, "Float rounding"). `RigidMotion::least_turn` decides its pole
# verdict and its conditioning, the lever against the run's rounding
# reach. `RigidMotion::least_turn_selected` decides the pole alone. It is
# for a turn that only selects (DESIGN `[scalars]`: a choice, never a
# claim), whose landing no claim consumes. Its one such caller is the
# path sweep's station step, which selects each station's roll as a
# reference vector, while each lane mints the station's frame from the
# tangent read there. A turn a claim consumes owes the conditioning, and
# nothing in the type tells the two apart, so the callers are pinned here.
#
# THE SCAN: every Rust file under crates/, demos/ and benches/ (src,
# tests, examples and benches, `#[cfg(test)]` modules included), comments
# and string literals stripped (`gate_rust_code`). The identifier
# `least_turn_selected` is counted per file. CALLERS lists the files that
# may name it and how many times: the home (its definition) and the
# station step. A file the list does not name reds, and so does a count
# that moved, so a second call beside the station step reds too. A new
# caller is a design question first (does any claim consume the turn?),
# and then a line here.
#
# KNOWN GAPS. A `use … as` rename is not followed, so the renamed call
# goes unseen, although the `use` line itself names the identifier and is
# counted. A function pointer to the door is counted like a call.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-core/src/linalg/rigid_motion.rs
# file count: the definition, and the sweep's station step.
CALLERS=(
  "$HOME_FILE 1"
  'crates/sweep/src/skin.rs 1'
)

# Records on stdin (`FILE:LINE:TEXT`); prints `FILE COUNT` for each file
# whose code names the identifier.
COUNT='
{
  if (!gate_record_split($0)) { next }
  t = GR_TEXT
  while (match(t, /(^|[^A-Za-z0-9_])least_turn_selected([^A-Za-z0-9_]|$)/)) {
    n[GR_FILE]++
    t = substr(t, RSTART + RLENGTH - 1)
  }
}
END { for (f in n) print f, n[f] }
'

gate() {
  gate_require_file "$HOME_FILE"
  local files=() f
  mapfile -t files < <(find crates demos benches -name target -prune -o -name '*.rs' -type f -print 2>/dev/null | sort)
  if [ "${#files[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no Rust file under crates/, demos/ or benches/ in $PWD, so the scan read nothing, which is not a pass"
    exit 1
  fi
  GATE_SCAN_FILES=${#files[@]}
  local counts
  if ! counts=$(gate_rust_code "${files[@]}" | gate_record_awk "$COUNT"); then
    gate_error "$(gate_name): the shared Rust reader exited, so what it did not count is unknown, which is not a pass"
    exit 1
  fi
  local bad=0 entry file want got
  declare -A allowed=()
  for entry in "${CALLERS[@]}"; do
    allowed[${entry% *}]=${entry##* }
  done
  while read -r file got; do
    [ -n "$file" ] || continue
    want=${allowed[$file]:-}
    if [ -z "$want" ]; then
      printf '%s: names least_turn_selected %s time(s)\n' "$file" "$got"
      bad=1
    elif [ "$got" != "$want" ]; then
      printf '%s: names least_turn_selected %s time(s), pinned at %s\n' "$file" "$got" "$want"
      bad=1
    fi
  done <<< "$counts"
  for file in "${!allowed[@]}"; do
    if ! grep -q "^$file " <<< "$counts"; then
      printf '%s: pinned at %s, names least_turn_selected nowhere\n' "$file" "${allowed[$file]}"
      bad=1
    fi
  done
  if [ "$bad" -ne 0 ]; then
    gate_error "least_turn_selected is named where CALLERS does not pin it (above): it decides no conditioning, so a caller is a turn no claim consumes (crates/sweep/README.md [sweep.frame]); a turn a claim consumes reads RigidMotion::least_turn, and a moved pin is a line in CALLERS"
    exit 1
  fi
  gate_ok "least_turn_selected is named at its home and the sweep's station step only"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}" "$1/crates/sweep/src"
  printf 'impl M {\n    pub fn least_turn_selected(a: A) -> M { m(a) }\n}\n' > "$1/$HOME_FILE"
  printf 'pub fn step(a: A) -> M {\n    RigidMotion::least_turn_selected(a)\n}\n' > "$1/crates/sweep/src/skin.rs"
}

plant_new_caller() {
  printf 'pub fn s(a: A) -> M { geom_core::RigidMotion::least_turn_selected(a) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_second_step() {
  printf 'pub fn again(a: A) -> M { RigidMotion::least_turn_selected(a) }\n' >> "$1/crates/sweep/src/skin.rs"
}

plant_test_caller() {
  mkdir -p "$1/crates/clean/tests"
  printf '#[test]\nfn t() { RigidMotion::least_turn_selected(a); }\n' > "$1/crates/clean/tests/t.rs"
}

plant_demo_caller() {
  mkdir -p "$1/demos/tour/src"
  printf 'pub fn s(a: A) -> M { RigidMotion::least_turn_selected(a) }\n' > "$1/demos/tour/src/s.rs"
}

plant_step_gone() {
  printf 'pub fn step(a: A) -> M { RigidMotion::least_turn(a) }\n' > "$1/crates/sweep/src/skin.rs"
}

plant_prose_only() {
  {
    printf '/// Not [`RigidMotion::least_turn_selected`]: a claim consumes this turn.\n'
    printf 'pub const WHY: &str = "least_turn_selected";\n'
    printf 'pub fn s(a: A) -> M { RigidMotion::least_turn(a); least_turn_selected_by_hand(a) }\n'
  } >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  local want="least_turn_selected is named where CALLERS does not pin it"
  gate_selftest_clean
  gate_selftest_without_tool awk "the shared Rust reader exited"
  gate_selftest_case "$want" plant_new_caller
  gate_selftest_case "$want" plant_second_step
  gate_selftest_case "$want" plant_test_caller
  gate_selftest_case "$want" plant_demo_caller
  gate_selftest_case "$want" plant_step_gone
  gate_selftest_passes "prose, a string literal and a longer name" plant_prose_only
  printf '%s selftest OK: passes a clean fixture and prose, string and longer-name mentions; fires on a new caller in a crate, a second call beside the station step, a test caller, a demo caller and a station step that no longer names it; stays RED when awk cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
