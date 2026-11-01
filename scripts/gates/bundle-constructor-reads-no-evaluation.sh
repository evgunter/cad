#!/usr/bin/env bash
# bundle-constructor-reads-no-evaluation.sh — the bundle constructor
# reads no evaluation. ONE home; ci.yml's `lint` job runs every gate in
# this directory.
#
# THE RULE is the helpers ruling's (`docs/INTENT-STAGE3-SPEC.md` §5′.2,
# D′2): `mate::pin` keeps one pure constructor of a bundle from the
# mates and moves its caller wrote, with the constant `0` on each
# freedom left unnamed. It reads no geometry and writes no number its
# caller did not write, so a placement stands exactly where its author
# means it. A helper that read a `Frame` off an evaluation and wrote it
# back as numbers is what the ruling retired (`place_at`, `pin_at`,
# `frame_at`, `place_against`): the frame it read came back rounded, and
# a turn its author meant as `turn/4` came back as `atan2`'s.
#
# THE SCAN is `crates/editor-core/src/mate/pin.rs`'s production code
# (`lib.sh`'s reader, `#[cfg(test)]` items dropped) in the statement
# view. `Evaluation` or `evaluate` as a word reds. A frame read off
# geometry has its own home, `mate/corner.rs` (`corner_frame`), which a
# caller asks before it writes the moves it means.
#
# KNOWN GAPS. The subject is one file: a helper that reads an
# evaluation from another file and hands `pin.rs` the numbers is not
# this gate's shape, and a re-export or an alias under another name
# carries the read in unseen. The words are matched case-sensitively at
# identifier boundaries, so `EvaluationLike` is not a read.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

PIN=crates/editor-core/src/mate/pin.rs

gate() {
  gate_require_file "$PIN"
  local view hits
  if ! view=$(gate_rust_code --skip-cfg-test --statements "$PIN"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  hits=$(printf '%s\n' "$view" \
    | gate_grep -E '(^|[^A-Za-z0-9_])(Evaluation|evaluate)([^A-Za-z0-9_]|$)')
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "$PIN reads an evaluation — the bundle constructor writes the mates and moves its caller wrote and reads no geometry (the helpers ruling, D′2): read a frame through mate/corner.rs and write the moves it means, or a mate and values"
    exit 1
  fi
  gate_ok "$PIN reads no evaluation: a bundle is written from the mates and moves its caller wrote"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${PIN%/*}"
  printf 'pub fn of(moves: Vec<M>) -> Placed { Placed { moves } }\n' > "$1/$PIN"
}

plant() {
  printf 'pub fn at(ev: &Evaluation<f64>) -> F { ev.frame() }\n' >> "$1/$PIN"
}

plant_call() {
  printf 'pub fn at(doc: &D) -> F { frame(crate::evaluate(doc)) }\n' >> "$1/$PIN"
}

# The call rustfmt leaves when the arguments are long.
plant_wrapped() {
  {
    printf 'pub fn at(doc: &D) -> F {\n'
    printf '    let ev = crate::evaluate(\n'
    printf '        doc,\n'
    printf '    );\n'
    printf '    frame(ev)\n'
    printf '}\n'
  } >> "$1/$PIN"
}

plant_prose_and_tests_only() {
  {
    printf '//! No `Evaluation` is read here.\n'
    printf 'pub const WHY: &str = "evaluate";\n'
    printf 'pub fn near(x: EvaluationLike) -> EvaluationLike { x }\n'
    printf '#[cfg(test)]\n'
    printf 'mod tests {\n'
    printf '    fn t(doc: &D) -> F { frame(crate::evaluate(doc)) }\n'
    printf '}\n'
  } >> "$1/$PIN"
}

gate_selftest() {
  local want="reads an evaluation"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_call
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_passes "prose, a string literal, a longer name and a #[cfg(test)] module" plant_prose_and_tests_only
  printf '%s selftest OK: passes a clean fixture and prose/string/longer-name mentions and a test module; fires on a type, a call and a rustfmt-wrapped call; stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
