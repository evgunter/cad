#!/usr/bin/env bash
# evalscalar-allowlist.sh — the Bounds gate, over the compound bound's
# NAME. ONE home; ci.yml's "EvalScalar allowlist (the Bounds gate, over
# the name)" step and local-scripts/ci-local.sh's discipline row both
# call this file.
#
# The SAME gate, over the compound bound's NAME (ASM-2A review
# MINOR-4). `editor_core::EvalScalar` is the evaluation-service
# bound — `Decide + ContentBits + Bounds + Send + Sync +
# PropsQuadLane` — declared once at the seam the rule above
# already ratifies, so `eval/parts.rs` names the requirement
# instead of restating `+ Bounds`. That is a naming, and it must
# stay one: the trait is `pub` (the integration suites need it),
# so without this step any file in any crate could acquire a
# compound Bounds bound invisibly to the grep above. Allowlist =
# exactly the seam real.rs already names, so the two steps agree
# about where the seam is; growing it needs the same ratification
# the Bounds allowlist does. Both bound positions are caught
# (`T: EvalScalar` and `+ EvalScalar`), over `lib.sh`'s code-only view:
# comments and string literals are gone in both directions, where the
# leading-`//` filter this gate carried stripped neither a trailing
# comment nor a block one.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  gate_require_crate_sources
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | grep -E '(:|\+)[[:space:]]*(editor_core::)?EvalScalar([^A-Za-z0-9_]|$)' \
    | grep -vE '^crates/editor-core/src/eval/(mod|parts)\.rs:' || true)
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "EvalScalar (a compound Bounds bound by another name) outside the evaluation-service seam — see geom-core/src/real.rs (Bounds scope rule) and editor-core/src/eval/mod.rs; ratify before allowlisting"
    exit 1
  fi
  gate_ok "no EvalScalar bound outside the evaluation-service seam"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: editor_core::EvalScalar>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f/* why */<T: editor_core::EvalScalar>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_plus_position() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Clone + EvalScalar>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_prose_only() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! `T: EvalScalar` is the evaluation-service bound.\n'
    printf '/// Never write `+ EvalScalar` outside the seam.\n'
    printf '/*\n * Nor T: editor_core::EvalScalar in a block comment.\n */\n'
    printf 'pub const WHY: &str = "T: EvalScalar";\n'
    printf 'pub fn ok<T: Real>(_t: T) {} // nor : EvalScalar in a trailing one\n'
    printf 'pub fn near<T: EvalScalarish>(_t: T) {}\n'
  } > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="EvalScalar (a compound Bounds bound by another name)"
  gate_selftest_clean
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_plus_position
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_passes "prose, doc comments, a string literal and a longer name starting with EvalScalar" plant_prose_only
  printf '%s selftest OK: passes a clean fixture, prose/doc/string mentions and `EvalScalarish`; fires in both bound positions and on a bound hidden behind a block comment\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
