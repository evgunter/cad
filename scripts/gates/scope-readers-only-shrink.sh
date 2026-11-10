#!/usr/bin/env bash
# scope-readers-only-shrink.sh — the record of hand-rolled scope readers
# may only shrink against the merge base.
#
# THE RULE. `crates/source-scope` is the one home for scope questions
# about Rust source, and `crates/test-utils/tests/reader_census.rs` keeps
# their ledger. A hand-rolled scope reader that predates the home takes a
# `HandRolled` line only for an item named in
# `HAND_ROLLED_AT_THE_HOMES_ARRIVAL`, the record of such readers as they
# stood when the home landed, keyed `path::item`. That file's tests hold
# each line to the record, and make a converted reader leave it. Only the
# merge base can show that the record did not GROW: an entry added in the
# same change as its reader passes every test in the file. This gate
# compares the record against the base, and refuses any entry the base
# does not hold.
#
# WHAT IT CANNOT SEE: a reader the ledger's needle cannot find and no
# review lists (the needle's own doc lists those shapes), and a record
# renamed under another constant (the record's absence is refused, so a
# rename reds here first).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

LEDGER=crates/test-utils/tests/reader_census.rs

# The record's entries in FILE, one per line; `ABSENT` where FILE holds
# no record.
record_of() {
  python3 - "$1" <<'PY'
import re, sys

text = open(sys.argv[1], encoding="utf-8").read()
m = re.search(
    r"const HAND_ROLLED_AT_THE_HOMES_ARRIVAL: \[&str; \d+\] = \[(.*?)\];", text, re.S
)
if not m:
    print("ABSENT")
    sys.exit(0)
body = re.sub(r"//[^\n]*", "", m.group(1))
for entry in re.findall(r'"([^"]+)"', body):
    print(entry)
PY
}

gate() {
  gate_require_file "$LEDGER" "the reader census and its scope ledger"
  local head base grown
  if ! head=$(record_of "$LEDGER"); then
    gate_error "the record reader (python3) failed on $LEDGER, so the record was not read: that is not a pass"
    exit 1
  fi
  if [ "$head" = ABSENT ]; then
    gate_error "$LEDGER holds no HAND_ROLLED_AT_THE_HOMES_ARRIVAL record: the record of hand-rolled scope readers is gone or renamed, so this gate cannot hold it; restore it, or move this gate with it"
    exit 1
  fi
  if ! gate_base_file "$LEDGER"; then
    gate_ok "the hand-rolled scope-reader record holds $(grep -c . <<<"$head") entries (GATE_BASE unset: compared with nothing)"
    return
  fi
  if ! base=$(record_of "$GATE_BASE_FILE"); then
    gate_error "the record reader (python3) failed on the base's $LEDGER, so the record was not compared: that is not a pass"
    exit 1
  fi
  if [ "$base" = ABSENT ]; then
    gate_ok "the hand-rolled scope-reader record holds $(grep -c . <<<"$head") entries (the base holds no record: this change sets it)"
    return
  fi
  grown=$(comm -23 <(sort -u <<<"$head") <(sort -u <<<"$base"))
  if [ -n "$grown" ]; then
    printf 'GREW %s\n' $grown
    gate_error "the hand-rolled scope-reader record grew against the merge base (above), and it only shrinks: a new scope reader resolves through \`source_scope::read\`, it is not recorded as hand-rolled"
    exit 1
  fi
  gate_ok "the hand-rolled scope-reader record holds $(grep -c . <<<"$head") entries, none new against the base"
}

record_text() {
  local entries=$1
  printf 'const HAND_ROLLED_AT_THE_HOMES_ARRIVAL: [&str; %s] = [\n' "$(wc -w <<<"$entries")"
  local e
  for e in $entries; do printf '    "%s",\n' "$e"; done
  printf '];\n'
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/test-utils/tests"
  record_text "a.rs::one b.rs::two" > "$1/$LEDGER"
}

plant_base() {
  git -C "$1" init -q
  git -C "$1" add -A
  git -C "$1" update-ref refs/gate-base "$(git -C "$1" write-tree)"
}
plant_grown()    { plant_base "$1"; record_text "a.rs::one b.rs::two c.rs::new" > "$1/$LEDGER"; }
plant_renamed()  { plant_base "$1"; record_text "a.rs::one b.rs::other" > "$1/$LEDGER"; }
plant_shrunk()   { plant_base "$1"; record_text "a.rs::one" > "$1/$LEDGER"; }
plant_same()     { plant_base "$1"; }
plant_absent()   { printf '// no record\n' > "$1/$LEDGER"; }
plant_new_here() {
  printf '// no record\n' > "$1/$LEDGER"
  plant_base "$1"
  record_text "a.rs::one" > "$1/$LEDGER"
}
plant_commented_entry() {
  plant_base "$1"
  { printf 'const HAND_ROLLED_AT_THE_HOMES_ARRIVAL: [&str; 2] = [\n    "a.rs::one",\n'
    printf '    "b.rs::two", // "c.rs::not_an_entry"\n];\n'; } > "$1/$LEDGER"
}

gate_selftest() {
  unset GATE_BASE
  gate_selftest_clean
  gate_selftest_without_tool python3 "the record reader (python3) failed"
  gate_selftest_case "holds no HAND_ROLLED_AT_THE_HOMES_ARRIVAL record" plant_absent
  GATE_BASE=refs/gate-base gate_selftest_case --also "GREW c.rs::new" "only shrinks" plant_grown
  GATE_BASE=refs/gate-base gate_selftest_case --also "GREW b.rs::other" "only shrinks" plant_renamed
  GATE_BASE=refs/gate-base gate_selftest_passes "a record that shrank" plant_shrunk
  GATE_BASE=refs/gate-base gate_selftest_passes "a record as the base holds it" plant_same
  GATE_BASE=refs/gate-base gate_selftest_passes "a record the base does not hold yet" plant_new_here
  GATE_BASE=refs/gate-base gate_selftest_passes "an entry named only in a comment" plant_commented_entry
  printf '%s selftest OK: passes a clean tree, a shrunk record, an unchanged one, one the base does not hold yet and a commented name; fires on a grown record, a renamed entry and a missing record, and stays RED when python3 cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
