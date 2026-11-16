#!/usr/bin/env bash
# dual-doors-only-through-the-record.sh — a dual scalar is instantiated
# in production code only on the evaluation service's record-bound path.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE SUBJECT. A dual's finished body is `topo::Replay`, tier 2 alone
# (DUAL-DESIGN DL3), and topo's readers that lean on a tier-3 premise
# hold it at a dual only because the evaluation service binds every
# dual run to its base-scalar build of record
# (`editor_core::evaluate_recorded`, `topo::Replay`'s "A tier-3 premise at
# a dual"). A caller that instantiated topo's or sweep's doors at a dual
# directly, with no record, would reach those readers on a body tier 3
# never read, and they read wrong rather than refuse. This gate holds the
# contract that no such caller exists: production code names a concrete
# dual only in the HOMES below. It is a gate, not a type: the type-level
# form is `work/onecopy/topos-doors-at-a-dual-take-no-tier-3-evidence-by-type.md`'s
# option B, which waits on a ruling.
#
# WHAT IS READ AS AN INSTANTIATION, over `lib.sh`'s code-only statement
# view of production code (`--skip-cfg-test`; comments and string
# literals are gone):
#   - the aliases `Dual64`, `DualInterval`, `DualProbe`;
#   - `Dual<X>` or `Dual::<X>` with a type argument longer than one
#     character (a concrete scalar; a one-letter generic `Dual<T>` is an
#     impl over every base, not an instantiation);
#   - a struct literal `Dual {` and an associated path `Dual::name`;
#   - a rename `Dual as …` (and of either alias), through which a file
#     would name a dual without the token.
#
# THE HOMES:
#   - `geom-core/src/dual.rs` and `geom-core/src/lib.rs`: the type, its
#     aliases and their re-export;
#   - `editor-core/src/stackup.rs`: the E4/E5 driver, which evaluates at
#     `Dual64` through `evaluate_recorded` only;
#   - `editor-core/src/analysis.rs`: the lane environment's binding of a
#     dual's tangent (`Dual::constant`, `Dual::variable`), read by an
#     evaluation the service runs.
#
# KNOWN GAPS, stated rather than hidden:
#   1. TEST CODE IS NOT READ: `#[cfg(test)]` items, `crates/*/tests/`,
#      and every path `lib.sh` resolves as a test-only mount. A test may
#      call topo at a dual directly, and many do.
#   2. A MACRO is read as written, never as expanded: a `macro_rules!`
#      body that writes a dual's name, or an instantiation built from
#      `concat!`-style tokens, is not seen.
#   3. A TYPE ALIAS DEFINED IN A HOME under another name and used
#      elsewhere (`pub type D = Dual64;` in `stackup.rs`, `D` outside) is
#      read only where it is defined.
#   4. A GENERIC caller instantiated at a dual through its parameter is
#      not an instantiation here: `evaluate_recorded`'s `T: BoundToRecord`
#      is the point, and a generic `T: EvalScalar` fn that a home calls at
#      `Dual64` is reached through the home.
#   5. `include!` is not followed.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

DUAL_SUBJECT='the dual instantiation homes: the type, the bound E4/E5 driver, the lane environment'
DUAL_HOMES=(
  crates/geom-core/src/dual.rs
  crates/geom-core/src/lib.rs
  crates/editor-core/src/stackup.rs
  crates/editor-core/src/analysis.rs
)

DUAL_NEEDLE='(^|[^A-Za-z0-9_])(Dual64|DualInterval|DualProbe|Dual[[:space:]]*(::)?[[:space:]]*<[[:space:]]*[A-Za-z_][A-Za-z0-9_:]{1,}|Dual[[:space:]]*\{|Dual[[:space:]]*::[[:space:]]*[a-z_]|(Dual|Dual64|DualInterval)[[:space:]]+as[[:space:]])'

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$DUAL_SUBJECT" "${DUAL_HOMES[@]}"
  local hits
  hits=$(gate_rust_code --skip-cfg-test --statements "${GATE_PRODUCTION_FILES[@]}" \
    | gate_grep -E "$DUAL_NEEDLE" \
    | gate_grep -vE "$(gate_record_anchor_any "${DUAL_HOMES[@]}")")
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "a dual scalar instantiated in production code off the record-bound path — a dual reaches topo's doors only through editor_core::evaluate_recorded (topo::Replay, \"A tier-3 premise at a dual\"); route it there, or add a home here with its reason"
    exit 1
  fi
  gate_ok "no dual scalar instantiated in production code outside the record-bound homes"
}

# Each home carries an instantiation it is exempted FOR, so the clean
# case reds the moment one of them stops being covered.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "${DUAL_HOMES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub type D = geom_core::Dual64;\n' > "$1/$home"
  done
}

planted() {
  mkdir -p "$1/crates/planted/src"
  printf '%s\n' "$2" > "$1/crates/planted/src/lib.rs"
}
plant_alias() { planted "$1" 'pub fn f(x: geom_core::Dual64) -> geom_core::Dual64 { x }'; }
plant_interval_alias() { planted "$1" 'pub fn f(_x: DualInterval) {}'; }
plant_turbofish() { planted "$1" 'pub fn f() { topo::boolean::<geom_core::Dual::<f64>>(); }'; }
plant_angle_arg() { planted "$1" 'pub fn f(_b: topo::Body<Dual<f64>>) {}'; }
plant_literal() { planted "$1" 'pub fn f() -> Dual<f64> { Dual { value: 1.0, deriv: 0.0 } }'; }
plant_assoc() { planted "$1" 'pub fn f() { let _ = geom_core::Dual::variable(1.0); }'; }
plant_rename() { planted "$1" 'use geom_core::Dual as D;'; }
plant_wrapped() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn f(\n'
    printf '    body: topo::Body<\n'
    printf '        geom_core::Dual64,\n'
    printf '    >,\n'
    printf ') {}\n'
  } > "$1/crates/planted/src/lib.rs"
}
plant_after_block_comment() { planted "$1" 'pub fn f(/* why */ _x: Dual64) {}'; }

plant_near_misses() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! `Dual64` is the dual scalar, `Dual<f64>` its spelling.\n'
    printf '/// Never write `Dual { value, deriv }` outside a home.\n'
    printf '/* nor Dual::variable(x) in a block comment */\n'
    printf 'pub const WHY: &str = "Dual64";\n'
    printf 'pub trait Dualism {}\n'
    printf 'impl<T: Copy> Lift for Dual<T> {}\n'
    printf 'pub fn g<B: Copy>(_x: Dual<B>) {}\n'
    printf '#[cfg(test)]\n'
    printf 'mod tests {\n'
    printf '    fn at_a_dual() { let _ = topo::boolean::<geom_core::Dual64>(); }\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="a dual scalar instantiated in production code off the record-bound path"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_alias
  gate_selftest_case "$want" plant_interval_alias
  gate_selftest_case "$want" plant_turbofish
  gate_selftest_case "$want" plant_angle_arg
  gate_selftest_case "$want" plant_literal
  gate_selftest_case "$want" plant_assoc
  gate_selftest_case "$want" plant_rename
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_passes "prose, doc and block comments, a string literal, a longer name, a generic Dual<T> impl and fn, and a dual in a #[cfg(test)] module" plant_near_misses
  gate_selftest_homes --narrowed --subject "$DUAL_SUBJECT" "${DUAL_HOMES[@]}"
  gate_selftest_test_module_homes "$want" plant_alias_line
  printf '%s selftest OK: passes a clean fixture carrying every home and the near misses (comments, strings, Dualism, generic Dual<T>, a test module); fires on each alias, a turbofish, a type argument, a struct literal, an associated path, a rename, a rustfmt-wrapped argument and one behind a block comment; and stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

# The resolver's own cases (`lib.sh`) append the breach to a file.
plant_alias_line() { printf 'pub fn f(_x: geom_core::Dual64) {}\n' >> "$1"; }

gate_parse_args "$@"
gate_main
