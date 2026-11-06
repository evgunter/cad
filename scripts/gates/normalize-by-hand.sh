#!/usr/bin/env bash
# normalize-by-hand.sh — a direction is read through `Vec3::normalize`
# (or `Vec2::normalize`), never spelled `v / v.norm()` by hand. ONE
# home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (`work/onecopy/a-direction-is-normalised-by-hand-beside-vec3-normalize.md`).
# `geom_core::Vec3::normalize` (`crates/geom-core/src/linalg/vec.rs`) is
# the home of "this vector's direction". Its body is `self / self.norm()`
# wherever the largest component lies in `(2⁻⁵¹¹, 2⁵¹¹)`, so in range a
# hand spelling is the same bits, and outside it the home's prescale
# answers where the hand spelling yields a zero or infinite vector.
# Production code takes the home; the home's own file is exempt.
#
# WHAT IS REFUSED, in production code under `crates/*/src` (comments and
# string literals out, `#[cfg(test)]` modules and test-only mounts out,
# and files named `test_support*`, `*_rows.rs` or `*_fuzz.rs` out, as
# `atan2-sites.sh` reads them):
#   * `E / E.norm()`, E the same name, field path or call chain twice
#     (`w / w.norm()`, `self.axis / self.axis.norm()`), whitespace
#     between the tokens allowed and rustfmt's wrap across lines read as
#     one statement;
#   * `E * (ONE / E.norm())`, ONE a literal `1.0` or a `::one()` call;
#   * either, with the norm spelled `E.norm_squared().sqrt()`.
#
# BLIND SPOTS. A parenthesised operand (`(q - p) / (q - p).norm()`):
# the matcher pairs only an identifier chain, so a compound expression
# divided by its own norm passes. A norm bound to a name first
# (`let n = v.norm(); … v / n`): many such sites also use the length,
# which `normalize` does not return, so the shape is not refused (the
# row's hit list reads each). A division through a helper, a closure or
# a macro; an `f64` triple held as `[f64; 3]`. The second pass for the first gap is
# `grep -rnP '(\([^()]*\))\s*/\s*\1\.norm\(\)' crates/*/src`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-core/src/linalg/vec.rs
HOME_SUBJECT='the home of a vector'"'"'s direction, Vec3::normalize'

gate() {
  gate_require_crate_sources
  gate_production_sources
  gate_require_homes "$HOME_SUBJECT" "$HOME_FILE"
  local files=() f
  for f in "${GATE_PRODUCTION_FILES[@]}"; do
    case "${f##*/}" in
      test_support*|*_rows.rs|*_fuzz.rs) ;;
      *) files+=("$f") ;;
    esac
  done
  GATE_SCAN_FILES=${#files[@]}
  local chain='[A-Za-z_][A-Za-z0-9_]*(\(\))?(\.[A-Za-z_][A-Za-z0-9_]*(\(\))?)*'
  local norm='(norm\(\)|norm_squared\(\)\.sqrt\(\))'
  local hits
  hits=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -E "(^|[^A-Za-z0-9_.])($chain)[[:space:]]*(/[[:space:]]*\\2\\.$norm([^.?[]|\$)|\\*[[:space:]]*\\([[:space:]]*(1\\.0|[A-Za-z0-9_:<>]*::one\\(\\))[[:space:]]*/[[:space:]]*\\2\\.$norm[[:space:]]*\\))" \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a direction normalised by hand (above): read it as v.normalize() (geom_core::Vec3::normalize / Vec2::normalize), the one home"
    exit 1
  fi
  gate_ok "no direction normalised by hand outside Vec3::normalize ($GATE_SCAN_FILES files)"
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}"
  printf 'pub fn normalize(v: V) -> V { v / v.norm() }\n' > "$1/$HOME_FILE"
}

plant_division() {
  printf 'pub fn dir(w: V) -> V { w / w.norm() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_field_path() {
  printf 'pub fn dir(s: S) -> V { s.frame.axis / s.frame.axis.norm() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_reciprocal() {
  printf 'pub fn dir<T>(far: V) -> V { far * (T::one() / far.norm()) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_literal_reciprocal() {
  printf 'pub fn dir(a: V) -> V { a * (1.0 / a.norm()) }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_squared_root() {
  printf 'pub fn dir(v: V) -> V { v / v.norm_squared().sqrt() }\n' >> "$1/crates/clean/src/lib.rs"
}

plant_wrapped() {
  printf 'pub fn dir(w: V) -> V {\n    w\n        / w.norm()\n}\n' >> "$1/crates/clean/src/lib.rs"
}

# A divisor that only begins with the norm, a different numerator, a
# norm read beside a different vector, prose, a test module, a fixture
# file and the home itself are not the shape.
plant_not_sites() {
  {
    printf 'pub fn a(v: V, s: f64) -> V { v / v.norm().max(s) }\n'
    printf 'pub fn b(v: V, w: V) -> V { v / w.norm() }\n'
    printf 'pub fn c(v: V, xv: V) -> V { xv / v.norm() }\n'
    printf '// w / w.norm() by hand\npub const S: &str = "w / w.norm()";\n'
    printf '#[cfg(test)]\nmod tests {\n    fn u(v: V) -> V { v / v.norm() }\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
  printf 'pub fn u(v: V) -> V { v / v.norm() }\n' > "$1/crates/clean/src/test_support.rs"
  printf 'pub fn u(v: V) -> V { v / v.norm() }\n' > "$1/crates/clean/src/walk_fuzz.rs"
}

gate_selftest() {
  local want="a direction normalised by hand"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_division
  gate_selftest_case "$want" plant_field_path
  gate_selftest_case "$want" plant_reciprocal
  gate_selftest_case "$want" plant_literal_reciprocal
  gate_selftest_case "$want" plant_wrapped
  gate_selftest_case "$want" plant_squared_root
  gate_selftest_passes "a longer divisor, other operands, prose, a test module, fixture files and the home" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "$HOME_FILE"
  printf '%s selftest OK: passes a clean fixture carrying the home, and a longer divisor, other operands, prose, a test module and fixture files; fires on v / v.norm() by name, by field path, wrapped across lines and through norm_squared().sqrt(), and on v * (1 / v.norm()) with T::one() and 1.0; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
