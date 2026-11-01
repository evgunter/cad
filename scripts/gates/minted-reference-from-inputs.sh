#!/usr/bin/env bash
# minted-reference-from-inputs.sh — no carrier the kernel mints takes its
# reference direction from `orthonormal_basis`. ONE home; ci.yml's
# `lint` job runs every gate in this directory.
#
# THE RULE is D10 `[placement.frame]`'s: a minted reference direction is
# a function of the inputs, not of the axes. `Vec3::orthonormal_basis`
# picks its first axis by comparing the normal's components against the
# coordinate axes, so a `u_ref` (or a seam) taken from it turns by a
# finite angle when the computing frame turns. A carrier mint therefore
# takes a direction its caller already holds from its construction, and
# `orthonormal_basis` is read only where its output is scratch that no
# output keeps, or where the coordinates ARE the input.
#
# THE SCAN is the production code (`lib.sh`'s reader, `#[cfg(test)]`
# items and test-only mounts dropped) of every crate, in the statement
# view. Every `orthonormal_basis` as a word outside the homes below reds.
#
# THE HOMES, each a whole-file skip, each with why its reads stay:
#   geom-core/src/linalg/{vec,unit_vec}.rs  the definitions;
#   step-import/src/recognize.rs  an imported plane's frame: the file's
#                                 coordinates are the import's input;
#   topo/src/boolean/boxes.rs     `BoxFrame::aimed`, a box frame read
#                                 only as a support along its aim;
#   topo/src/side_reader.rs       a ray's aim, which no output keeps;
#   topo/src/boolean/section_cert.rs  a ruling chart and the section
#                                 witnesses: no output keeps them, but a
#                                 verdict may read them, which is filed
#                                 (`work/issues/a-section-certificates-
#                                 witness-point-is-picked-off-the-axes.md`).
#
# KNOWN GAPS. A home's file can grow a second read that does reach a
# stored reference, and nothing here sees it: a skip is per file. A
# helper that wraps `orthonormal_basis` under another name inside a home
# carries it out unseen. And a reference picked off the axes WITHOUT this
# function — a literal world axis crossed with a normal — is not this
# gate's shape at all; `crates/sweep/tests/a_body_built_turned_is_the_body_turned.rs`
# is the row that sees a reference move with the axes, wherever it was
# picked, on the constructions it builds.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

BASIS_SUBJECT='a scratch or input-coordinate read of orthonormal_basis, the one place it may be called'
BASIS_HOMES=(
  crates/geom-core/src/linalg/vec.rs
  crates/geom-core/src/linalg/unit_vec.rs
  crates/step-import/src/recognize.rs
  crates/topo/src/boolean/boxes.rs
  crates/topo/src/side_reader.rs
  crates/topo/src/boolean/section_cert.rs
)

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$BASIS_SUBJECT" "${BASIS_HOMES[@]}"
  local view hits
  if ! view=$(gate_rust_code --skip-cfg-test --statements "${GATE_PRODUCTION_FILES[@]}"); then
    gate_error "$(gate_name): the shared Rust reader could not build the statement view, so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  hits=$(printf '%s\n' "$view" \
    | gate_grep -E '(^|[^A-Za-z0-9_])orthonormal_basis([^A-Za-z0-9_]|$)' \
    | gate_grep -vE "$(gate_record_anchor_any "${BASIS_HOMES[@]}")")
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "orthonormal_basis outside its homes — a minted reference direction is a function of the inputs, not of the axes (D10 [placement.frame]): pass the direction the caller holds from its construction, or, for scratch no output keeps, add the file to this gate's homes with its reason"
    exit 1
  fi
  gate_ok "orthonormal_basis is read only in its homes: no minted carrier reference is picked off the axes"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "${BASIS_HOMES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub fn f(n: V) -> V { n.orthonormal_basis().0 }\n' > "$1/$home"
  done
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn cap(n: V) -> V { let (u_ref, _) = n.orthonormal_basis(); u_ref }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# The call rustfmt leaves when the receiver is long.
plant_wrapped() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn cap(n: V) -> V {\n'
    printf '    let (u_ref, _) = n\n'
    printf '        .orthonormal_basis();\n'
    printf '    u_ref\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_ufcs() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn cap(n: UnitVec3) -> V { UnitVec3::orthonormal_basis(n).0 }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_prose_and_tests_only() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! `orthonormal_basis` picks off the axes.\n'
    printf 'pub const WHY: &str = "orthonormal_basis";\n'
    printf 'pub fn near(n: V) -> V { n.orthonormal_basis_of_mine() }\n'
    printf '#[cfg(test)]\n'
    printf 'mod tests {\n'
    printf '    fn t(n: V) -> V { n.orthonormal_basis().0 }\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="orthonormal_basis outside its homes"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_case "$want" plant_ufcs
  gate_selftest_passes "prose, a string literal, a longer name and a #[cfg(test)] module" plant_prose_and_tests_only
  gate_selftest_homes --narrowed --subject "$BASIS_SUBJECT" "${BASIS_HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying every home, prose/string/longer-name mentions and a test module; fires on a plain call, a rustfmt-wrapped call and a UFCS call outside the homes; stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
