#!/usr/bin/env bash
# line-distance-reads-distance.sh — a point's distance from a
# `geom::Line` is `Line::distance`. ONE home; ci.yml's `lint` job runs
# every gate in this directory.
#
# THE RULE: a reader of a point's distance from a line reads
# `geom::Line::distance` (`|(p − origin) × dir| / |dir|`, correct for any
# nonzero `dir`), not the norm of an offset spelled by hand or through
# `Line::offset`/`Vec3::reject_from`. The regime, measured (the table is
# `Line::distance`'s doc): where the point carries width, `distance` is
# the tightest spelling, about 1.6× tighter than the offset's norm; where
# only the axis does, it is about the offset's width, and twice a unit
# read's. A unit read (no division) is not offered, because a stored axis
# is unit only to ~1e-10 in `|dir|²`, and a unit read of one is wrong by
# more than the band.
#
# KNOWN GAPS: production sources only (`#[cfg(test)]` items and
# test-only mounts are skipped, so oracles may spell it their own way);
# an argument nested more than two parentheses deep is not matched; a
# cross product whose operand is not a written `(a − b)` difference (a
# named vector) is not matched, since a free vector's cross norm is a
# different fact; a norm spelled `.norm_squared().sqrt()` passes; and a
# norm taken of a local helper that returns a line's offset vector (as
# `geom::cover`'s `Site::relative` was read) passes, since the helper's
# name carries no shape a grep can key on. Where such a helper exists,
# give its owner a distance reading that calls `Line::distance`, as
# `Site::distance` does.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# The three spellings of the norm of a point's offset from a line: the
# offset's, the rejection's, and a hand cross product of a point
# difference with a direction.
# THE ONE EXEMPTION: `emit_topo`'s segment row passes the area
# `|w × d|` to `Margin::over_lever` over the segment's length, the door
# that is the home of a measure over its lever.
EXEMPT='crates/editor-core/src/names/emit_topo\.rs:[0-9]+: .*Margin::over_lever'
SHAPES='\.offset\([^()]*(\([^()]*\)[^()]*)*\)[[:space:]]*\.norm\(\)|\.reject_from\([^()]*(\([^()]*\)[^()]*)*\)[[:space:]]*\.norm\(\)|\([[:alnum:]_.*]+ - [[:alnum:]_.*]+\)[[:space:]]*\.cross\([^()]*(\([^()]*\)[^()]*)*\)[[:space:]]*\.norm\(\)'

gate() {
  gate_require_crate_sources
  gate_production_sources
  local hits
  hits=$(gate_rust_code --skip-cfg-test --statements "${GATE_PRODUCTION_FILES[@]}" \
    | gate_grep -E "$SHAPES" \
    | gate_grep -vE "^($EXEMPT)")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a line's distance read as offset(…).norm(): read Line::distance, correct for any nonzero dir"
    exit 1
  fi
  gate_ok "no line distance read as offset(…).norm()"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn d(l: geom::Line<f64>, p: geom_core::Point3<f64>) -> f64 { l.offset(p).norm() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_reject() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn d(o: P, p: P, a: V) -> f64 { (p - o).reject_from(a).norm() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_cross_across_lines() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn d(o: P, p: P, a: V) -> f64 {\n    (p - o)\n        .cross(a)\n        .norm()\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_offset_inside_an_expression() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn g(l: L, p: P, n: V) -> f64 { (l.offset(p) - n).norm() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_vector_use() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn r(l: geom::Line<f64>, p: geom_core::Point3<f64>) -> geom_core::Vec3<f64> { l.offset(p).normalize() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="a line's distance read as offset(…).norm()"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_reject
  gate_selftest_case "$want" plant_cross_across_lines
  gate_selftest_passes "the norm of a vector expression holding an offset" plant_offset_inside_an_expression
  gate_selftest_passes "an offset read as the vector it is" plant_vector_use
  printf '%s selftest OK: passes a clean fixture and an offset read as a vector; fires on offset(…).norm(), reject_from(…).norm() and a cross norm split over lines; passes the norm of a vector expression; and stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
