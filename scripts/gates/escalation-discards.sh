#!/usr/bin/env bash
# escalation-discards.sh — every production arm that passes over a
# refusal unread is registered, per file, at a pinned count. ONE home;
# ci.yml's `lint` job runs every gate in this directory.
#
# THE INVARIANT. A search that reads candidates until one decides and
# drops each one the band does not decide (`Ok(Sign::Zero) | Err(_) =>
# continue`) refuses, where none decides, with a cause that is not the
# reading that stopped it: `splitting::finish::whole_body_side` refused
# every curved edge in band as a corrupt body. The rule is stated once,
# in `geom_brep::ladder::climb`, which keeps what it passes over, ranks
# it (`ladder::Evidence`) and owns the recording. This gate makes the
# hand-rolled spelling visible: a new arm of the shape reds until the
# register below names why it is not a search's kept reading.
#
# THE REGISTER, `escalation-discards.counts`: `<file> <count>` per line,
# `#` comments carrying each file's disposition. A file whose count is
# above its entry (or that has none) reds: a new discard. One below it
# reds too, asking for the entry to be lowered, so the slack cannot let
# one back in unseen.
#
# WHAT IS READ: production sources (`crates/*/src`, less test-only mounts
# and `#[cfg(test)]` items), in the code view (comments and literals
# blanked), for `Err(_) =>` followed by `continue`, `break` or `{}` on
# the same line.
#
# WHAT IT CANNOT SEE, each a pass where a discard exists: a binding that
# is dropped by name (`Err(_e) => continue`, `Err(..) => {}`); a
# catch-all (`_ => continue`) after the decided arms; an arm whose body
# is on the next line; `.ok()`, `if let Ok(..)` and `matches!(.., Ok(..))`
# reads that never name the error. Those shapes also carry legitimate
# reads too often for a line grep to tell apart, so they are a reviewer's.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

REGISTER=scripts/gates/escalation-discards.counts
DISCARD_RE='Err\(_\)[[:space:]]*=>[[:space:]]*(continue|break|\{[[:space:]]*\})'

gate() {
  gate_require_crate_sources
  gate_require_file "$REGISTER" "the escalation-discard register"
  gate_production_sources
  local view
  view=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}")
  local live want
  live=$(printf '%s\n' "$view" | gate_grep -E "$DISCARD_RE" | gate_record_file | sort | uniq -c \
    | awk '{ print $2, $1 }')
  want=$(awk '!/^[[:space:]]*(#|$)/ { print $1, $2 }' "$REGISTER" | sort)
  local diff
  diff=$(diff <(printf '%s\n' "$live" | sed '/^$/d' | sort) <(printf '%s\n' "$want" | sed '/^$/d') || true)
  if [ -n "$diff" ]; then
    printf '%s\n' "$diff"
    gate_error "escalation discards differ from $REGISTER ('<' is the tree, '>' the register): port a search to geom_brep::ladder::climb, or register the arm with why it is no search's kept reading"
    exit 1
  fi
  gate_ok "every production arm that passes over a refusal unread is registered"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/scripts/gates"
  printf '# none\n' > "$1/$REGISTER"
}

plant_unregistered() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f(v: &[Result<u8, ()>]) { for r in v { match r { Ok(_) => {}\n Err(_) => continue, } } }\n' \
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
  printf '#[cfg(test)]\nmod rows { fn f(r: Result<u8, ()>) { match r { Ok(_) => {}\n Err(_) => {} } } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="escalation discards differ"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_unregistered
  gate_selftest_case "$want" plant_stale_entry
  gate_selftest_passes "a registered discard" plant_registered
  gate_selftest_passes "a discard inside a #[cfg(test)] module" plant_in_a_test_module
  printf '%s selftest OK: passes a clean tree, a registered discard and one in a test module; fires on an unregistered discard and on a register entry the tree no longer carries, and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
