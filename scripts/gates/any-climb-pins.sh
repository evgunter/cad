#!/usr/bin/env bash
# any-climb-pins.sh — every production `climb`/`climb_evidence` under
# `Answering::Any` is registered: pinned by a named test, or on a named
# work row with the fixture its pin needs. ONE home; ci.yml's `lint` job
# runs every gate in this directory.
#
# THE RULE (work/onecopy/door-searches-left-on-the-hand-rolled-ladder.md,
# closed into work/onecopy/t4-ports-unpinned-for-a-kept-reading-ahead-of-an-answer.md).
# An `Any` ladder answers past a reading it kept in band, and records that
# run superseded. That path is the port's whole reason, and a first-refusal
# loop put back in its place passes every test that never keeps a reading
# ahead of an answer. So each site names its pin: a test that reads the
# production function with a kept reading ahead of an answering rung (or
# ranks two kept readings), red under a first-refusal loop. A site with no
# cheap fixture names the row that carries it, and that row must exist.
#
# WHAT IS READ: production sources (`crates/*/src`, less test-only mounts
# and `#[cfg(test)]` items) for every ladder that answers past a kept rung:
# `Answering::Any` (the argument of an entry point that takes an
# `Answering`), and each call of an entry point that takes none
# (`climb_agreeing`, `climb_differing`), the entry points read off
# `crates/geom-core/src/ladder.rs` (`gate_ladder_entry_points`). Each is
# keyed by the nearest `fn` above it (the function the ladder runs in),
# counted per (file, function). The register lists every (file, function, count) with
# its disposition, `pin <test fn>` (that `fn` must exist under `crates/`)
# or `row <id>` (that `work/**/<id>.md` must exist), or `proof <slug>`
# where no input reaches past a kept rung: the reason is stated at the
# site, in a comment holding `any-climb-pins: proof <slug>` in that file.
#
# WHAT IT CANNOT SEE: a ladder whose `Answering` value is a variable, or
# `Any` through `read_identity` (its own rows pin it); and whether a pin
# reaches its site, which `scripts/any-first-mutant.sh` runs nightly.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

REGISTER=scripts/gates/any-climb-pins.counts
HOME_FILE=crates/geom-core/src/ladder.rs

gate() {
  gate_require_crate_sources
  gate_require_file "$REGISTER" "the Any-climb pin register"
  gate_production_sources
  local -a files=()
  local f
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    [ "$f" = "$HOME_FILE" ] || files+=("$f")
  done
  # The ladder's entry points (`gate_ladder_entry_points`, the one list):
  # one that takes an `Answering` supersedes kept rungs only under `Any`,
  # so its sites are the `Answering::Any` it is called with; one that
  # takes none (`climb_agreeing`, `climb_differing`) supersedes them on
  # every answer, so each call of it is a site.
  gate_ladder_entry_points
  local unanswered
  unanswered=$(GATE_ENTRIES="$GATE_LADDER_ENTRY_POINTS" python3 - "$HOME_FILE" <<'PY'
import os, re, sys
src = open(sys.argv[1]).read()
out = []
for name in os.environ["GATE_ENTRIES"].split():
    m = re.search(r"^pub fn " + re.escape(name) + r"\b(.*?)\{", src, re.M | re.S)
    if m and not re.search(r"\bAnswering\b", m.group(1)):
        out.append(name)
print("|".join(out))
PY
)
  local site='Answering::Any\b'
  [ -n "$unanswered" ] && site="$site|(^|[^A-Za-z_0-9:]|::)($unanswered)[[:space:]]*\("
  local hits=""
  if [ "${#files[@]}" -gt 0 ]; then
    hits=$(gate_rust_code --skip-cfg-test "${files[@]}" | gate_grep -E "$site" | awk -F: '{ print $1 ":" $2 }')
  fi
  local report
  report=$(GATE_HITS="$hits" python3 - "$REGISTER" <<'PY'
import collections, glob, os, re, sys
register = sys.argv[1]
FN = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?fn\s+([A-Za-z_0-9]+)")
live = collections.Counter()
for rec in os.environ.get("GATE_HITS", "").split("\n"):
    if not rec:
        continue
    path, line = rec.split(":")[0], int(rec.split(":")[1])
    name = None
    with open(path) as fh:
        for i, l in enumerate(fh, 1):
            if i > line:
                break
            m = FN.match(l)
            if m:
                name = m.group(1)
    live[(path, name)] += 1
want = collections.Counter()
problems = []
# A pin names a test: a fn under a `#[test]` or `#[rstest]` attribute,
# with any other attributes and doc comments between them.
TEST_ATTR = re.compile(r"^\s*#\[(?:test|rstest)\b")
SKIP = re.compile(r"^\s*(?:#\[|///|//)")
tests = set()
for p in glob.glob("crates/**/*.rs", recursive=True):
    with open(p, errors="replace") as fh:
        armed = False
        for l in fh:
            if TEST_ATTR.match(l):
                armed = True
                continue
            m = FN.match(l)
            if m:
                if armed:
                    tests.add(m.group(1))
                armed = False
            elif armed and l.strip() and not SKIP.match(l):
                armed = False
rows = {os.path.basename(p)[:-3] for p in glob.glob("work/**/*.md", recursive=True)}
with open(register) as fh:
    for n, raw in enumerate(fh, 1):
        line = raw.split("#", 1)[0].split()
        if not line:
            continue
        if len(line) != 5 or line[3] not in ("pin", "row", "proof"):
            problems.append(f"{register}:{n}: want `<file> <fn> <count> pin|row|proof <name>`: {raw.strip()}")
            continue
        path, name, count, kind, ref = line
        want[(path, name)] += int(count)
        if kind == "pin" and ref not in tests:
            problems.append(f"{register}:{n}: pin `{ref}` names no #[test] or #[rstest] fn under crates/")
        if kind == "row" and ref not in rows:
            problems.append(f"{register}:{n}: row `{ref}` names no work/**/{ref}.md")
        if kind == "proof":
            marker = f"any-climb-pins: proof {ref}"
            try:
                quoted = marker in open(path, errors="replace").read()
            except OSError:
                quoted = False
            if not quoted:
                problems.append(f"{register}:{n}: proof `{ref}` is not stated at its site (no `{marker}` in {path})")
for key in sorted(set(live) | set(want), key=lambda k: (k[0], k[1] or "")):
    if live[key] != want[key]:
        problems.append(f"{key[0]} {key[1]}: {live[key]} Any ladder(s) in the tree, {want[key]} registered")
print("\n".join(problems))
PY
)
  if [ -n "$report" ]; then
    printf '%s\n' "$report"
    gate_error "Any ladders differ from $REGISTER (above): name each one's pin (a test with a kept reading ahead of an answer, red under a first-refusal loop), or the row that carries it"
    exit 1
  fi
  gate_ok "every production ladder that answers past a kept rung names its pin or its row"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/geom-core/src"
  printf 'pub fn climb<I>(answering: Answering, rungs: I) {}\npub fn climb_differing<I>(rungs: I) {}\n' > "$1/$HOME_FILE"
  mkdir -p "$1/scripts/gates" "$1/work/p"
  printf '# none\n' > "$1/$REGISTER"
  printf -- '---\nid: carrier\n---\n' > "$1/work/p/carrier.md"
}

plant_unregistered() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn search() { climb(Answering::Any, xs, f); }\n' > "$1/crates/planted/src/lib.rs"
}

plant_pinned() {
  plant_unregistered "$1"
  printf '#[cfg(test)]\nmod rows {\n    /// A later rung answers.\n    #[test]\n    #[ignore]\n    fn a_later_rung_answers() {}\n}\n' >> "$1/crates/planted/src/lib.rs"
  printf 'crates/planted/src/lib.rs search 1 pin a_later_rung_answers\n' >> "$1/$REGISTER"
}

plant_pin_gone() {
  plant_unregistered "$1"
  printf 'crates/planted/src/lib.rs search 1 pin no_such_test\n' >> "$1/$REGISTER"
}

plant_pin_not_a_test() {
  plant_unregistered "$1"
  printf 'crates/planted/src/lib.rs search 1 pin search\n' >> "$1/$REGISTER"
}

plant_count_short() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn search() {\n    climb(Answering::Any, xs, f);\n    climb(Answering::Any, ys, g);\n}\n' > "$1/crates/planted/src/lib.rs"
  printf 'crates/planted/src/lib.rs search 1 row carrier\n' >> "$1/$REGISTER"
}

plant_differing_unregistered() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn mixed() {\n    match climb_differing(xs, f, g) {}\n}\n' > "$1/crates/planted/src/lib.rs"
}

plant_differing_on_a_row() {
  plant_differing_unregistered "$1"
  printf 'crates/planted/src/lib.rs mixed 1 row carrier\n' >> "$1/$REGISTER"
}

plant_proved() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn search() {\n    // any-climb-pins: proof ordered - a kept rung is never followed by an answer.\n    climb(Answering::Any, xs, f);\n}\n' > "$1/crates/planted/src/lib.rs"
  printf 'crates/planted/src/lib.rs search 1 proof ordered\n' >> "$1/$REGISTER"
}

plant_proof_unstated() {
  plant_unregistered "$1"
  printf 'crates/planted/src/lib.rs search 1 proof ordered\n' >> "$1/$REGISTER"
}

plant_on_a_row() {
  plant_unregistered "$1"
  printf 'crates/planted/src/lib.rs search 1 row carrier\n' >> "$1/$REGISTER"
}

plant_row_gone() {
  plant_unregistered "$1"
  printf 'crates/planted/src/lib.rs search 1 row no-such-row\n' >> "$1/$REGISTER"
}

plant_in_a_test_module() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod rows { fn f() { climb(Answering::Any, xs, g); } }\n' > "$1/crates/planted/src/lib.rs"
}

plant_stale_entry() {
  printf 'crates/clean/src/lib.rs gone 1 row carrier\n' >> "$1/$REGISTER"
}

gate_selftest() {
  local want="Any ladders differ"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_unregistered
  gate_selftest_case "$want" plant_pin_gone
  gate_selftest_case "$want" plant_pin_not_a_test
  gate_selftest_case "$want" plant_count_short
  gate_selftest_case "$want" plant_row_gone
  gate_selftest_case "$want" plant_stale_entry
  gate_selftest_case "$want" plant_differing_unregistered
  gate_selftest_case "$want" plant_proof_unstated
  gate_selftest_passes "a ladder with its proof stated" plant_proved
  gate_selftest_passes "a differing ladder on a row" plant_differing_on_a_row
  gate_selftest_passes "a pinned ladder" plant_pinned
  gate_selftest_passes "a ladder on a row" plant_on_a_row
  gate_selftest_passes "a ladder in a test module" plant_in_a_test_module
  printf '%s selftest OK: passes a clean tree, a pinned ladder, one on a row, one with its proof stated and one in a test module; fires on an unregistered ladder, a pin or row that names nothing, a pin that names a fn that is no test, a function with more ladders than registered, a register entry the tree no longer carries, and a call of an entry point that takes no Answering (read off ladder.rs) left unregistered, a proof not stated at its site, and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
