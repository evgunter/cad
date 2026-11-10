#!/usr/bin/env bash
# recourse-ending-by-hand.sh — a recourse fragment that has a whole
# ending beside it is ended through that ending, never labelled by hand.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE: where `geom_core` defines both `X_RECOURSE` (the fragment, for
# a sentence that composes it with words of its own) and `X_ENDING` (the
# fragment labelled `Recourse: …` as a whole ending), a refusal that ends
# on the labelled fragment writes `X_ENDING`. Spelling the label beside
# the fragment is a second copy of the ending, which drifts from it.
# The pairs are read off `crates/geom-core/src`, so a new `X_ENDING` is
# held the day it lands. Today: `RANGE_RECOURSE` / `RANGE_ENDING`. One
# pair is named otherwise and is held by name (`NAMED_PAIRS` below):
# `COINCIDENCE_RECOURSE` / `MOVE_RECOURSE`, the move-recourse class's
# fragment and its labelled whole ending. A move a site names more
# closely is ended through `geom_core::move_recourse!("so …")`, its
# words after the fragment's.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (string
# literals kept, comments out, `#[cfg(test)]` and test-only mounts out),
# read across lines (a hit does not cross a `;`):
#   * `Recourse: {X_RECOURSE}`, the fragment named inline after the label
#     (a `\` line continuation between them allowed), with or without a
#     path on the name;
#   * `Recourse: {}` in a statement whose last argument is `X_RECOURSE`;
#   * `Recourse(X_RECOURSE)`, the label type wrapped round the fragment.
#   * the label and the move arm's WORDS as a literal: `Recourse: move
#     the geometry` spelled by hand (across `\` continuations too) rather
#     than through `MOVE_RECOURSE` or `move_recourse!("so …")`
#     (`work/onecopy/move-the-geometry-is-spelled-as-a-literal-at-a-hundred-sites.md`);
#   * the move arm's words opening a literal, unlabelled: `"move the
#     geometry` at a word boundary (across `\` continuations too),
#     whatever follows, rather than `geom_core::move_lever!("so …")`, for
#     a slot a recourse label is put before (a `LeverOnly`'s or a
#     `SizedDecision`'s `lever`). geom-core, the arm's home, is out. A
#     lever that opens the arm with a continuation `move_lever!` does not
#     take (its compile-time check takes `so ` alone) is a RECORDED
#     EXCEPTION (`MOVE_ARM_EXCEPTIONS` below), each with its reason and
#     its site count: a site no entry names reds, and so does an entry
#     whose count is no longer the tree's, which is also what retires it
#     (`work/onecopy/a-lever-opening-the-move-arm-without-so-passes-the-gate.md`).
#   * `move_lever!(@words …)` outside `crates/geom-core`: the arm's
#     inner form, which `#[macro_export]` makes public and which skips
#     the `so ` check that `move_lever!` and `move_recourse!` make.
#   Test code is out of all three, so a pin may spell the text it pins; the
#   other subjects of a `Recourse: move …` (`the parts`, `the point`, …)
#   are an operation's own levers and pass.
#
# BLIND SPOTS, each one a shape that passes:
#   * the label and the fragment across a `;` (a `let` holding the
#     format string, a helper that prefixes `Recourse: `);
#   * `Recourse: {}` with `X_RECOURSE` as an argument other than the last;
#   * a pair named neither `X_RECOURSE` / `X_ENDING` nor in
#     `NAMED_PAIRS`;
#   * the move arm's words mid-literal, after words of the site's own
#     (`"…, or move the geometry so the contact exists"`), which is the
#     fragment composed into a sentence of the site's own;
#   * the macro forms: `move_recourse!("so …")` and `move_lever!("so …")`
#     pass, and so does a hand
#     `concat!("Recourse: ", geom_core::coincidence_move_arm!(), …)`, which
#     no shape reads (it names the arm's macro, not a fragment constant);
#   * a fragment composed mid-sentence after words of the site's own
#     (`profile::path`'s "Recourse: only the ratio …, {RANGE_RECOURSE}"),
#     which is the fragment's intended use and is meant to pass.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# Pairs held by name, `FRAGMENT:ENDING`, each checked to exist in
# geom-core: a pair that went missing is a gate reading nothing, not a pass.
NAMED_PAIRS="COINCIDENCE_RECOURSE:MOVE_RECOURSE"

# THE RECORDED EXCEPTIONS to the unlabelled-arm shape, one per line,
# `FILE|NEEDLE|COUNT|REASON`: the NEEDLE is the words that open the
# exempted literal (its whitespace and `\` continuations collapsed), the
# COUNT the sites it covers in FILE, no more and no fewer.
MOVE_ARM_EXCEPTIONS=(
  "crates/geom-brep/src/certify.rs|move the geometry to a scale where|1|a scale lever (the spline face's control points past the finite numbers), not a coincidence's move: move_lever! takes only a condition after so"
  "crates/geom-brep/src/ssi.rs|move the geometry, or the searched region,|1|two levers in one sentence, the geometry or the searched region, which no so-condition after the arm alone says"
  "crates/geom-brep/src/ssi/exhaust.rs|move the geometry nearer the origin|1|a resolution lever (coordinates too far from the origin to resolve ε), not a coincidence's move"
  "crates/profile/src/fillet_recourse.rs|move the geometry nearer the origin|1|a resolution lever (the corner too far from the origin for its radius), not a coincidence's move"
  "crates/test-utils/src/refusal.rs|move the geometry\"]|1|BARE_RECOURSES restates COINCIDENCE_RECOURSE in a dependency-free leaf; editor-core's the_bare_recourses_are_geom_cores_constants holds it equal"
)

# THE LIST IS OVERRIDABLE FOR A PLANTED TREE, AND ONLY FOR ONE, as
# viewer-module-kinds.sh's: honoured only under `--root` (`GATE_ROOT`),
# which only the self-test passes, so no environment exempts the repo.
move_arm_exceptions_from_env() {
  [ -n "${GATE_ROOT:-}" ] || return 0
  [ -n "${GATE_SELFTEST_MOVE_ARM_EXCEPTIONS+x}" ] || return 0
  MOVE_ARM_EXCEPTIONS=()
  if [ -n "$GATE_SELFTEST_MOVE_ARM_EXCEPTIONS" ]; then
    mapfile -t MOVE_ARM_EXCEPTIONS <<< "$GATE_SELFTEST_MOVE_ARM_EXCEPTIONS"
  fi
}

# The stems with both a fragment and an ending in geom-core.
stems() {
  local consts
  consts=$(grep -rhoE 'pub const [A-Z_]+_(RECOURSE|ENDING): &str' crates/geom-core/src 2>/dev/null \
    | sed -E 's/pub const ([A-Z_]+): &str/\1/' | sort -u) || true
  printf '%s\n' "$consts" | sed -nE 's/^([A-Z_]+)_ENDING$/\1/p' | while read -r s; do
    if printf '%s\n' "$consts" | grep -qx "${s}_RECOURSE"; then printf '%s\n' "$s"; fi
  done
}

# The matcher: reads the line view (`path:line:text`, literals kept) on
# stdin, rejoins each file's lines, and prints each hit as
# `path:line: text` at the line the hit starts on. Its one argument is
# the stems' fragments as an alternation.
# shellcheck disable=SC2016
MATCHER='
import re, sys
frag = sys.argv[1]
name = r"(?:[A-Za-z_]+::)*(?:" + frag + r")"
shapes = re.compile(
    r"Recourse: (?:\\\s*)?\{" + name + r"\}"
    + r"|Recourse: \{\}\"[^;]*?[(,]\s*" + name + r"\s*,?\s*\)"
    + r"|Recourse\(\s*" + name + r"\s*\)"
)
files = {}
for record in sys.stdin:
    path, line, text = record.rstrip("\n").split(":", 2)
    files.setdefault(path, []).append((int(line), text))
for path, lines in files.items():
    blob, starts, at = "", [], 0
    for line, text in lines:
        starts.append((at, line))
        blob += text + "\n"
        at = len(blob)
    for m in shapes.finditer(blob):
        line = max(l for o, l in starts if o <= m.start())
        hit = " ".join(m.group(0).split())
        print(f"{path}:{line}: {hit}")
'

# The words matcher: the line view on stdin, each file's lines rejoined.
# It prints each `Recourse: move the geometry` spelled by hand, and each
# `"move the geometry` opening a literal outside geom-core that no
# recorded exception (its one argument, the entries one per line) names,
# as `path:line: text`; and each entry whose count is not the tree's, as
# `path: recorded N, found M: needle`. The words may break at a `\`
# continuation.
# shellcheck disable=SC2016
WORDS='
import re, sys
gap = r"(?:\s|\\)+"
arm = "move" + gap + "the" + gap + "geometry"
labelled = re.compile(r"Recourse:" + gap + arm)
lever = re.compile(r"\"" + arm + r"\b")
entries = []
for spec in sys.argv[1].split("\n"):
    if spec:
        path, needle, count, _reason = spec.split("|", 3)
        entries.append([path, needle, int(count), 0])
def flat(text):
    return " ".join(re.sub(r"\\\n", " ", text).split())
files = {}
for record in sys.stdin:
    path, line, text = record.rstrip("\n").split(":", 2)
    files.setdefault(path, []).append((int(line), text))
for path, lines in files.items():
    blob, starts, at = "", [], 0
    for line, text in lines:
        starts.append((at, line))
        blob += text + "\n"
        at = len(blob)
    def at_line(m):
        return max(l for o, l in starts if o <= m.start())
    for m in labelled.finditer(blob):
        print(f"{path}:{at_line(m)}: " + " ".join(m.group(0).split()))
    if path.startswith("crates/geom-core/"):
        continue
    for m in lever.finditer(blob):
        opens = flat(blob[m.start() + 1 : m.start() + 200])
        entry = next((e for e in entries if e[0] == path and opens.startswith(e[1])), None)
        if entry is None:
            print(f"{path}:{at_line(m)}: \"" + opens[:60])
        else:
            entry[3] += 1
for path, needle, count, found in entries:
    if found != count:
        print(f"{path}: recorded {count}, found {found}: {needle}")
'

gate() {
  gate_require_crate_sources
  gate_production_sources
  local list alt hits view pair frag ending named=""
  list=$(stems)
  for pair in $NAMED_PAIRS; do
    frag=${pair%%:*} ending=${pair#*:}
    for c in "$frag" "$ending"; do
      if ! grep -rqE "pub const $c: &str" crates/geom-core/src; then
        gate_error "$(gate_name): the named pair $pair has no \`pub const $c\` in crates/geom-core/src — the gate would read nothing for it, which is not a pass"
        exit 1
      fi
    done
    named="$named $frag"
  done
  if [ -z "$list" ] && [ -z "$named" ]; then
    gate_ok "no geom-core recourse has both a fragment and an ending"
    return
  fi
  alt=$( { printf '%s\n' "$list" | sed '/^$/d; s/$/_RECOURSE/'; printf '%s\n' $named; } | paste -sd'|')
  view=$(gate_rust_code --skip-cfg-test --keep-literals "${GATE_PRODUCTION_FILES[@]}")
  hits=$(printf '%s\n' "$view" | python3 -c "$MATCHER" "$alt")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a recourse fragment labelled by hand beside its ending: write the X_ENDING constant (for COINCIDENCE_RECOURSE, MOVE_RECOURSE or move_recourse!(\"so …\"))"
    exit 1
  fi
  move_arm_exceptions_from_env
  hits=$(printf '%s\n' "$view" | python3 -c "$WORDS" "$(printf '%s\n' ${MOVE_ARM_EXCEPTIONS[@]+"${MOVE_ARM_EXCEPTIONS[@]}"})")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "the move recourse spelled by hand: write MOVE_RECOURSE for the whole ending, geom_core::move_recourse!(\"so …\") for a move named more closely (a format! site passes it as an argument), or geom_core::move_lever!(\"so …\") for that move as an unlabelled lever; a lever that opens the arm with another continuation is a recorded exception (MOVE_ARM_EXCEPTIONS, with its reason), and an entry whose count is not the tree's is moved to it"
    exit 1
  fi
  hits=$(printf '%s\n' "$view" | grep -v '^crates/geom-core/' \
    | grep -E 'move_lever![[:space:]]*[(\[{][[:space:]]*@words' || true)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "move_lever!(@words …) outside geom-core skips the so-check: write geom_core::move_lever!(\"so …\")"
    exit 1
  fi
  gate_ok "every recourse fragment with an ending is ended through it ($(printf '%s\n' "$list" | sed 's/$/_RECOURSE/' | paste -sd, -),$named), and the move recourse is never spelled by hand, labelled or as a lever, nor its inner @words form read outside geom-core"
}

plant_home() {
  mkdir -p "$1/crates/geom-core/src"
  printf 'pub const RANGE_RECOURSE: &str = "scale it";\npub const RANGE_ENDING: &str = "Recourse: scale it";\npub const COINCIDENCE_RECOURSE: &str = "move it";\npub const MOVE_RECOURSE: &str = "Recourse: move it";\n' \
    > "$1/crates/geom-core/src/lib.rs"
}

# The clean fixture carries the constants the gate reads its pairs off.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  plant_home "$1"
}

plant_named_pair_missing() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/crates/geom-core/src"
  printf 'pub const RANGE_RECOURSE: &str = "scale it";\npub const RANGE_ENDING: &str = "Recourse: scale it";\n' \
    > "$1/crates/geom-core/src/lib.rs"
}

plant_named_pair() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn w(f: &mut F) -> R {\n    write!(f, "too thin. Recourse: {COINCIDENCE_RECOURSE}; {note}")\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_named_pair_ending() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn w(f: &mut F) -> R {\n    write!(f, "too thin. {MOVE_RECOURSE}; {note}")\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_named() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn w(f: &mut F) -> R {\n    write!(f, "too far. Recourse: {RANGE_RECOURSE}")\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_positional() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn w(f: &mut F, r: &str) -> R {\n    write!(\n        f,\n        "the {} is not finite. Recourse: {}",\n        r,\n        geom_core::RANGE_RECOURSE\n    )\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_wrapped() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub const E: &dyn D = &geom_core::Recourse(geom_core::RANGE_RECOURSE);\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_ending() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn w(f: &mut F) -> R {\n    write!(f, "too far. {}", geom_core::RANGE_ENDING)\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_mid_sentence() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn w(f: &mut F) -> R {\n    write!(f, "Recourse: only the ratio matters, {RANGE_RECOURSE}")\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_move_words() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub const E: &str = "Recourse: move the \\\n     geometry so it parts";\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_move_words_in_a_test() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod tests {\n    const END: &str = "\\n}\\n";\n    #[test]\n    fn t() {\n        assert!(e.ends_with("Recourse: move the geometry"));\n    }\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_move_other_subject() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub const E: &str = "Recourse: move the parts apart";\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_move_lever() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub const L: Lever = Lever { lever: "move the \\\n     geometry so it parts" };\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_move_lever_mid_sentence() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub const L: &str = "drop the declaration, or move the geometry so it parts";\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_inner_words() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub const L: &str = geom_core::move_lever!(@words "nearer it");\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_inner_words_at_home() {
  plant_home "$1"
  printf 'macro_rules! move_recourse {\n    ($so:expr) => { concat!("Recourse: ", $crate::move_lever!(@words $so)) };\n}\n' \
    >> "$1/crates/geom-core/src/lib.rs"
}

plant_arm_other_continuation() {
  plant_home "$1"
  mkdir -p "$1/crates/planted/src"
  printf 'pub const L: &str = "move the geometry \\\n     nearer the origin, where it resolves";\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_arm_at_home() {
  plant_home "$1"
  printf 'macro_rules! coincidence_move_arm {\n    () => { "move the geometry" };\n}\n' \
    >> "$1/crates/geom-core/src/lib.rs"
}

gate_selftest() {
  # The repo's own exceptions name files no planted tree has, so every
  # case starts from none, and the exception cases supply their own.
  export GATE_SELFTEST_MOVE_ARM_EXCEPTIONS=""
  local want="a recourse fragment labelled by hand beside its ending"
  gate_selftest_clean
  gate_selftest_case "$want" plant_named
  gate_selftest_case "$want" plant_positional
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_case "$want" plant_named_pair
  gate_selftest_passes "the named pair's ending written whole" plant_named_pair_ending
  gate_selftest_case "the named pair COINCIDENCE_RECOURSE:MOVE_RECOURSE has no" plant_named_pair_missing
  gate_selftest_passes "the ending written whole" plant_ending
  gate_selftest_passes "the fragment composed mid-sentence" plant_mid_sentence
  gate_selftest_case "the move recourse spelled by hand" plant_move_words
  gate_selftest_passes "a test pin spelling the move recourse, past a literal closing brace" \
    plant_move_words_in_a_test
  gate_selftest_passes "another subject of a move" plant_move_other_subject
  gate_selftest_case "the move recourse spelled by hand" plant_move_lever
  gate_selftest_passes "the move arm composed mid-sentence" plant_move_lever_mid_sentence
  gate_selftest_case "the move recourse spelled by hand" plant_arm_other_continuation
  gate_selftest_passes "the arm in its home" plant_arm_at_home
  GATE_SELFTEST_MOVE_ARM_EXCEPTIONS="crates/planted/src/lib.rs|move the geometry nearer the origin|1|a planted reason"
  gate_selftest_passes "a recorded exception at its count" plant_arm_other_continuation
  GATE_SELFTEST_MOVE_ARM_EXCEPTIONS="crates/planted/src/lib.rs|move the geometry nearer the origin|2|a planted reason"
  gate_selftest_case "recorded 2, found 1" plant_arm_other_continuation
  GATE_SELFTEST_MOVE_ARM_EXCEPTIONS="crates/ghost/src/lib.rs|move the geometry nearer the origin|1|a planted reason"
  gate_selftest_case "recorded 1, found 0" plant_arm_other_continuation
  GATE_SELFTEST_MOVE_ARM_EXCEPTIONS=""
  gate_selftest_case "skips the so-check" plant_inner_words
  gate_selftest_passes "the inner @words form in its home" plant_inner_words_at_home
  printf '%s selftest OK: passes a clean fixture, the ending written whole and the fragment mid-sentence; fires on the fragment named after the label, passed as the last argument after `Recourse: {}`, and wrapped in Recourse(…), and on the named pair COINCIDENCE_RECOURSE after the label (its MOVE_RECOURSE passes), and on a named pair geom-core no longer defines; fires on the words of the move recourse spelled by hand across a continuation, and passes them in a test module whose literal holds a closing brace, and another subject of a move; fires on the words of the move arm opening a lever across a continuation, and passes them mid-sentence; fires on the inner @words form outside geom-core, and passes it in its home; fires on the arm opening a lever with another continuation and passes it in its home, passes it as a recorded exception at its count, and fires on an entry whose count is above the tree'"'"'s or whose file has none\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
