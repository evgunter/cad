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
# held the day it lands. Today: `RANGE_RECOURSE` / `RANGE_ENDING`.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (string
# literals kept, comments out, `#[cfg(test)]` and test-only mounts out),
# read across lines (a hit does not cross a `;`):
#   * `Recourse: {X_RECOURSE}`, the fragment named inline after the label
#     (a `\` line continuation between them allowed), with or without a
#     path on the name;
#   * `Recourse: {}` in a statement whose last argument is `X_RECOURSE`;
#   * `Recourse(X_RECOURSE)`, the label type wrapped round the fragment.
#
# BLIND SPOTS, each one a shape that passes:
#   * the label and the fragment across a `;` (a `let` holding the
#     format string, a helper that prefixes `Recourse: `);
#   * `Recourse: {}` with `X_RECOURSE` as an argument other than the last;
#   * a pair not named `X_RECOURSE` / `X_ENDING`: `COINCIDENCE_RECOURSE`
#     and its labelled form `MOVE_RECOURSE` are not held, and about a dozen
#     sites write `Recourse: {COINCIDENCE_RECOURSE}` (several continuing
#     it, "; {note}" or ", or …"). That is the move-recourse class's to
#     decide (`work/onecopy/carry-refusal-copies-and-hand-spelled-move-recourses.md`);
#   * a fragment composed mid-sentence after words of the site's own
#     (`profile::path`'s "Recourse: only the ratio …, {RANGE_RECOURSE}"),
#     which is the fragment's intended use and is meant to pass.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

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

gate() {
  gate_require_crate_sources
  gate_production_sources
  local list alt hits
  list=$(stems)
  if [ -z "$list" ]; then
    gate_ok "no geom-core recourse has both a fragment and an ending"
    return
  fi
  alt=$(printf '%s\n' "$list" | sed 's/$/_RECOURSE/' | paste -sd'|')
  hits=$(gate_rust_code --skip-cfg-test --keep-literals "${GATE_PRODUCTION_FILES[@]}" \
    | python3 -c "$MATCHER" "$alt")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a recourse fragment labelled by hand beside its ending: write the X_ENDING constant"
    exit 1
  fi
  gate_ok "every recourse fragment with an ending is ended through it ($(printf '%s' "$list" | paste -sd, -))"
}

plant_home() {
  mkdir -p "$1/crates/geom-core/src"
  printf 'pub const RANGE_RECOURSE: &str = "scale it";\npub const RANGE_ENDING: &str = "Recourse: scale it";\n' \
    > "$1/crates/geom-core/src/lib.rs"
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

gate_selftest() {
  local want="a recourse fragment labelled by hand beside its ending"
  gate_selftest_clean
  gate_selftest_case "$want" plant_named
  gate_selftest_case "$want" plant_positional
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_passes "the ending written whole" plant_ending
  gate_selftest_passes "the fragment composed mid-sentence" plant_mid_sentence
  printf '%s selftest OK: passes a clean fixture, the ending written whole and the fragment mid-sentence; fires on the fragment named after the label, passed as the last argument after `Recourse: {}`, and wrapped in Recourse(…)\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
