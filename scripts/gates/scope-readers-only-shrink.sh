#!/usr/bin/env bash
# scope-readers-only-shrink.sh — the records of hand-rolled scope readers
# and of keywords that are no scope question may only shrink against the
# merge base.
#
# THE RULE. `crates/source-scope` is the one home for scope questions
# about Rust source, and `crates/test-utils/tests/reader_census.rs` keeps
# their ledger. A hand-rolled scope reader that predates the home takes a
# `HandRolled` line only for an item named in
# `HAND_ROLLED_AT_THE_HOMES_ARRIVAL`, the record of such readers as they
# stood when the home landed, keyed `path::item`. A site that spells a
# binding keyword for another reason takes a `NotScope` line only for an
# item named, under that reason, in `NOT_SCOPE_AT_THE_HOMES_ARRIVAL`
# (`Reason path::item`). That file's tests hold each line to its record,
# and make a converted item leave it. Only the merge base can show that a
# record did not GROW: an entry added in the same change as its reader
# passes every test in the file. This gate compares both records with the
# base, and refuses any entry the base does not hold: a new reader, a new
# reason, a new file for one, or a new item.
#
# THE BASE, as `gate_base_file` reads it for every pin that only falls:
# ci.yml's `lint` job sets GATE_BASE to the change filter's base, and
# `run-gates.sh` sets it to the merge base with origin/main when a local
# run leaves it unset. An unscoped CI run has no base by design (a
# one-commit checkout, no origin/main): there the gate says it compared
# nothing and passes, as the other only-falls gates do. A base that is
# given but cannot be read is refused.
#
# WHAT IT CANNOT SEE: a reader the ledger's needle cannot find and no
# review lists (the needle's own doc lists those shapes), and a record
# renamed under another constant (a record's absence is refused, so a
# rename reds here first).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

LEDGER=crates/test-utils/tests/reader_census.rs
RECORDS="HAND_ROLLED_AT_THE_HOMES_ARRIVAL NOT_SCOPE_AT_THE_HOMES_ARRIVAL"

# The records' entries in FILE, one per line as `NAME entry`; `ABSENT
# NAME` for a record FILE does not hold.
record_of() {
  python3 - "$1" $RECORDS <<'READER'
import re, sys

text = open(sys.argv[1], encoding="utf-8").read()
for name in sys.argv[2:]:
    m = re.search(r"const " + name + r": \[&str; \d+\] = \[(.*?)\];", text, re.S)
    if not m:
        print("ABSENT " + name)
        continue
    body = re.sub(r"//[^\n]*", "", m.group(1))
    for entry in re.findall(r'"([^"]+)"', body):
        print(name + " " + entry)
READER
}

gate() {
  gate_require_file "$LEDGER" "the reader census and its scope ledger"
  local head base grown absent name new_here=""
  if ! head=$(record_of "$LEDGER"); then
    gate_error "the record reader (python3) failed on $LEDGER, so the records were not read: that is not a pass"
    exit 1
  fi
  absent=$(grep '^ABSENT ' <<<"$head" | sed 's/^ABSENT //' || true)
  if [ -n "$absent" ]; then
    gate_error "$LEDGER holds no $absent record: a record of scope readers is gone or renamed, so this gate cannot hold it; restore it, or move this gate with it"
    exit 1
  fi
  if ! gate_base_file "$LEDGER"; then
    gate_ok "the scope-reader records are read (GATE_BASE unset: compared with nothing)"
    return
  fi
  if ! base=$(record_of "$GATE_BASE_FILE"); then
    gate_error "the record reader (python3) failed on the base's $LEDGER, so the records were not compared: that is not a pass"
    exit 1
  fi
  for name in $RECORDS; do
    if grep -qx "ABSENT $name" <<<"$base"; then
      new_here="$new_here $name"
      head=$(grep -v "^$name " <<<"$head" || true)
    fi
  done
  grown=$(comm -23 <(sed '/^$/d' <<<"$head" | sort -u) <(grep -v '^ABSENT ' <<<"$base" | sort -u))
  if [ -n "$grown" ]; then
    sed 's/^/GREW /' <<<"$grown"
    gate_error "a scope-reader record grew against the merge base (above), and each only shrinks: a new scope reader resolves through \`source_scope::read\`, and a new reason, file or item for a keyword that is no scope question is not recorded here either"
    exit 1
  fi
  if [ -n "$new_here" ]; then
    gate_ok "the scope-reader records hold no entry new against the base (the base holds no${new_here}: this change sets it)"
  else
    gate_ok "the scope-reader records hold no entry new against the base"
  fi
}

# The two records, each from a comma-separated list.
record_text() {
  local name entries e
  for name in $RECORDS; do
    case $name in
      HAND_ROLLED_*) entries=$1 ;;
      *) entries=$2 ;;
    esac
    printf 'const %s: [&str; %s] = [\n' "$name" "$(tr ',' '\n' <<<"$entries" | grep -c .)"
    while IFS= read -r e; do
      [ -n "$e" ] && printf '    "%s",\n' "$e"
    done < <(tr ',' '\n' <<<"$entries")
    printf '];\n'
  done
}

plant_base() {
  git -C "$1" init -q
  git -C "$1" add -A
  git -C "$1" update-ref refs/gate-base "$(git -C "$1" write-tree)"
}

# The clean tree is its own base, so the clean case compares.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/test-utils/tests"
  record_text "a.rs::one,b.rs::two" "Message c.rs::m" > "$1/$LEDGER"
  plant_base "$1"
}

plant_grown()        { record_text "a.rs::one,b.rs::two,c.rs::new" "Message c.rs::m" > "$1/$LEDGER"; }
plant_renamed()      { record_text "a.rs::one,b.rs::other" "Message c.rs::m" > "$1/$LEDGER"; }
plant_reason_grown() { record_text "a.rs::one,b.rs::two" "Message c.rs::m,Message d.rs::new" > "$1/$LEDGER"; }
plant_reason_moved() { record_text "a.rs::one,b.rs::two" "Visibility c.rs::m" > "$1/$LEDGER"; }
plant_shrunk()       { record_text "a.rs::one" "" > "$1/$LEDGER"; }
plant_absent()       { printf '// no record\n' > "$1/$LEDGER"; }
plant_one_absent()   { record_text "a.rs::one" "Message c.rs::m" | sed '/NOT_SCOPE/,$d' > "$1/$LEDGER"; }
plant_new_here() {
  printf '// no record\n' > "$1/$LEDGER"
  plant_base "$1"
  record_text "a.rs::one" "Message c.rs::m" > "$1/$LEDGER"
}
plant_commented_entry() {
  { printf 'const HAND_ROLLED_AT_THE_HOMES_ARRIVAL: [&str; 2] = [\n    "a.rs::one",\n'
    printf '    "b.rs::two", // "c.rs::not_an_entry"\n];\n'
    printf 'const NOT_SCOPE_AT_THE_HOMES_ARRIVAL: [&str; 1] = ["Message c.rs::m"];\n'; } > "$1/$LEDGER"
}
plant_nothing() { :; }

gate_selftest() {
  export GATE_BASE=refs/gate-base
  gate_selftest_clean
  gate_selftest_without_tool python3 "the record reader (python3) failed"
  GATE_BASE="" gate_selftest_passes "no base given (an unscoped run)" plant_nothing
  GATE_BASE=refs/no-such-base gate_selftest_case "is not a commit" plant_nothing
  gate_selftest_case "holds no HAND_ROLLED_AT_THE_HOMES_ARRIVAL" plant_absent
  gate_selftest_case "holds no NOT_SCOPE_AT_THE_HOMES_ARRIVAL" plant_one_absent
  gate_selftest_case --also "GREW HAND_ROLLED_AT_THE_HOMES_ARRIVAL c.rs::new" "only shrinks" plant_grown
  gate_selftest_case --also "GREW HAND_ROLLED_AT_THE_HOMES_ARRIVAL b.rs::other" "only shrinks" plant_renamed
  gate_selftest_case --also "GREW NOT_SCOPE_AT_THE_HOMES_ARRIVAL Message d.rs::new" "only shrinks" plant_reason_grown
  gate_selftest_case --also "GREW NOT_SCOPE_AT_THE_HOMES_ARRIVAL Visibility c.rs::m" "only shrinks" plant_reason_moved
  gate_selftest_passes "records that shrank" plant_shrunk
  gate_selftest_passes "records the base does not hold yet" plant_new_here
  gate_selftest_passes "an entry named only in a comment" plant_commented_entry
  printf '%s selftest OK: passes a clean tree, no base given, shrunk records, records the base does not hold yet and a commented name; fires on a base it cannot read, a grown or renamed hand-rolled entry, a grown or re-reasoned NotScope entry and a missing record of either kind, and stays RED when python3 cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
