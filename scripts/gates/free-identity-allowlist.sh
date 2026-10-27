#!/usr/bin/env bash
# free-identity-allowlist.sh — `Decide::free_identity` is called from
# LIMB 2 only. ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# WHY IT EXISTS. `Decide::free_identity` (C2's limb 2 at a
# parameter-family scalar; fork row 204) attaches a symbolic identity to
# a margin: at `Sym` the margin keeps its numeric channel and takes the
# identity's node, so a margin the numeric channel cannot decide is
# decided `Zero` wherever the identity's normal form is the zero form.
# That is sound only where the identity is zero exactly when the margin
# is — limb 2's residual of a carrier against the surface it bounds the
# distance from, formed by the limb from the same carrier and surface.
# Any other caller hands the tier a claim about a margin it did not
# form, which is the capability `register_equal`'s allowlist exists to
# keep in view, reached here without `register_equal`. So the method
# stays, and the files that may call it are a list a reviewer can read.
#
# THE SITE. `crates/geom-brep/src/edge_nurbs.rs` — `limb2_enclosure`,
# fed by `free_residual`: the margin is the composite's enclosure of the
# carrier's distance from the operand, and the identity is the
# carrier's own description composed into the operand's implicit form
# (its frame made orthonormal, as the composite reads it), with the
# carrier's parameter free. Zero as a form means the carrier lies on
# that surface at every parameter, for every member of the box, which
# is what makes the distance zero.
#
# WHAT IT MATCHES: the method called (`m.free_identity(f)`) and the path
# named, which is how a call is spelt in UFCS (`Decide::free_identity(m,
# f)`, `<T as Decide>::free_identity(m, f)`, `T::free_identity`) and how
# it is handed on as a function value (`.map(Decide::free_identity)`).
# A definition (`fn free_identity`) names it after neither.
#
# WHAT THIS GATE DOES NOT COVER: it reads source text, so a call reached
# through a macro, a re-export under another name, or a trait object is
# invisible to it; and it says nothing about whether the site's identity
# is the right one. Adding a site is a design decision.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

CALLER_SUBJECT='limb 2, the only file that may CALL Decide::free_identity'
DEFINITION_SUBJECT='the files that DEFINE Decide::free_identity, where a mention is a definition and not a call'
CALLER_HOMES=(
  crates/geom-brep/src/edge_nurbs.rs
)
DEFINITION_HOMES=(
  crates/geom-core/src/predicate.rs
  crates/geom-core/src/sym.rs
)

gate() {
  gate_require_crate_sources
  gate_require_homes "$DEFINITION_SUBJECT" "${DEFINITION_HOMES[@]}"
  gate_require_homes "$CALLER_SUBJECT" "${CALLER_HOMES[@]}"
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E '(\.|::)[[:space:]]*free_identity([^[:alnum:]_]|$)' \
    | gate_grep -vE "$(gate_record_anchor_any "${DEFINITION_HOMES[@]}")" \
    | gate_grep -vE "$(gate_record_anchor_any "${CALLER_HOMES[@]}")" \
    | cut -c1-160)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "$(gate_name): a call to Decide::free_identity outside limb 2 (this file's header says why). The door attaches an identity the tier then decides a margin by; a new site is a design decision, not a call"
    exit 1
  fi
  gate_ok "Decide::free_identity is called only from limb 2"
}

# Every exempted file is in the clean fixture, each carrying a call, so
# the clean case reds the moment one of them stops being covered; beside
# them the trait's default body and prose naming the method.
gate_plant_clean() {
  local home
  mkdir -p "$1/crates/topo/src"
  for home in "${DEFINITION_HOMES[@]}" "${CALLER_HOMES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'fn f<T: Decide>(a: T) -> T { a.free_identity(|_| a) }\n' > "$1/$home"
  done
  cat >> "$1/crates/geom-core/src/predicate.rs" <<'RS'
pub trait Decide {
    fn free_identity<F>(self, identity: F) -> Self {
        self
    }
}
RS
  cat > "$1/crates/topo/src/margin.rs" <<'RS'
// Never call `m.free_identity(f)` here: only limb 2 forms both.
pub const WHY: &str = "m.free_identity(f)";
RS
}

plant_colon_after_the_home_that_is_not_a_line_number() {
  local home
  for home in "${DEFINITION_HOMES[@]}" "${CALLER_HOMES[@]}"; do
    printf 'fn f<T: Decide>(a: T) -> T { a.free_identity(|_| a) }\n' > "$1/$home:x.rs"
  done
}

plant_call_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f<T: Decide>(a: T) -> T { a.free_identity(|_| a) }\n' > "$1/crates/topo/src/margin.rs"
}

plant_ufcs_call() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f<T: Decide>(a: T) -> T { Decide::free_identity(a, |_| a) }\n' \
    > "$1/crates/topo/src/margin.rs"
}

plant_qualified_ufcs_call() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f<T: Decide>(a: T) -> T { <T as Decide>::free_identity(a, |_| a) }\n' \
    > "$1/crates/topo/src/margin.rs"
}

plant_path_as_a_value() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f<T: Decide>(a: Option<T>) -> Option<T> { a.map(|m| T::free_identity(m, |_| m)) }\n' \
    > "$1/crates/topo/src/margin.rs"
}

plant_call_after_block_comment() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f<T: Decide>(a: T) -> T { /* why */ a.free_identity(|_| a) }\n' \
    > "$1/crates/topo/src/margin.rs"
}

# Another file inside geom-brep: the allowlist is per FILE.
plant_call_in_another_geom_brep_file() {
  mkdir -p "$1/crates/geom-brep/src"
  printf 'fn f<T: Decide>(a: T) -> T { a.free_identity(|_| a) }\n' \
    > "$1/crates/geom-brep/src/certify.rs"
}

gate_selftest() {
  local want="a call to Decide::free_identity outside limb 2"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_call_outside
  gate_selftest_case "$want" plant_call_after_block_comment
  gate_selftest_case "$want" plant_ufcs_call
  gate_selftest_case "$want" plant_qualified_ufcs_call
  gate_selftest_case "$want" plant_path_as_a_value
  gate_selftest_case "$want" plant_call_in_another_geom_brep_file
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "the trait default, the definitions, limb 2 and prose" gate_plant_clean
  gate_selftest_homes \
    --subject "$DEFINITION_SUBJECT" "${DEFINITION_HOMES[@]}" \
    --subject "$CALLER_SUBJECT" "${CALLER_HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying every definition and caller file it exempts, the trait default and prose that names the method; fires on a call from another crate, on one spelt in UFCS (bare, qualified `<T as Decide>::`, and through the scalar `T::`), on one hidden behind a block comment, on a second file inside geom-brep, and at the colon-carrying path a home skip that ends at `:` exempts; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
