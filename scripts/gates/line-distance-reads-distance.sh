#!/usr/bin/env bash
# line-distance-reads-distance.sh — a point's distance from a
# `geom::Line` is `Line::distance`, and from a span of one
# `Line::distance_within`. ONE home each; ci.yml's `lint` job runs every
# gate in this directory.
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
# A point's distance from a SPAN of a line (a segment, a ray held at an
# end) reads `geom::Line::distance_within` (`geom_core::Line2`'s in the
# plane), not the norm of the point less its `foot_within`, spelled by
# hand or through `Margin::norm2`/`norm3`. The two homes are the one
# exemption, by file (`geom/src/curves.rs`, `geom-core/src/line2.rs`):
# their own body is that norm, read on `self`.
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
# `Site::distance` does. A span foot bound to a name before its norm
# (`let foot = l.foot_within(..); (p - foot).norm()`) passes, and so does
# a foot read as the point it is.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# The three spellings of the norm of a point's offset from a line: the
# offset's, the rejection's, and a hand cross product of a point
# difference with a direction.
# THE ONE EXEMPTION: `emit_topo`'s segment row passes the area
# `|w × d|` to `Margin::over_lever` over the segment's length, the door
# that is the home of a measure over its lever.
EXEMPT='crates/editor-core/src/names/emit_topo\.rs:[0-9]+: .*Margin::over_lever|crates/(geom/src/curves|geom-core/src/line2)\.rs:[0-9]+: .*\(p - self\.foot_within\(p, span\)\)\.norm\(\)'
# A span foot's arguments nest three parentheses deep (`(q, (t0.min(t1),
# t0.max(t1)))`); its norm is the difference's either way round.
NEST1='[^()]*(\([^()]*\)[^()]*)*'
NEST2="[^()]*(\\(${NEST1}\\)[^()]*)*"
SPAN_FOOT="foot_within\\(${NEST2}\\)"
SPAN_SHAPES="${SPAN_FOOT}\\)[[:space:]]*\\.norm\\(\\)|${SPAN_FOOT}[[:space:]]*-[^;()]*\\)[[:space:]]*\\.norm\\(\\)|Margin::norm[23]\\([^;]*\\.foot_within\\("
SHAPES='\.offset\([^()]*(\([^()]*\)[^()]*)*\)[[:space:]]*\.norm\(\)|\.reject_from\([^()]*(\([^()]*\)[^()]*)*\)[[:space:]]*\.norm\(\)|\([[:alnum:]_.*]+ - [[:alnum:]_.*]+\)[[:space:]]*\.cross\([^()]*(\([^()]*\)[^()]*)*\)[[:space:]]*\.norm\(\)'

gate() {
  gate_require_crate_sources
  gate_production_sources
  local hits
  hits=$(gate_rust_code --skip-cfg-test --statements "${GATE_PRODUCTION_FILES[@]}" \
    | gate_grep -E "$SHAPES|$SPAN_SHAPES" \
    | gate_grep -vE "^($EXEMPT)")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a line's distance read as offset(…).norm(): read Line::distance, correct for any nonzero dir; a span's as its foot's norm: read Line::distance_within"
    exit 1
  fi
  gate_ok "no line distance read as offset(…).norm(), no span distance as its foot's norm"
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

plant_span_foot_norm() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn d(l: geom::Line<f64>, p: P) -> f64 {\n    (p - l.foot_within(p, (0.0, 1.0)))\n        .norm()\n}\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_span_foot_margin() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn d(l: L, u: P, at: S) -> M { Margin::norm3(u - l.line().foot_within(u, at)) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_span_foot_reversed_deep() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn d(l: L, c: P) -> T { (l.foot_within(c, (T::zero(), T::one())) - c).norm() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_home_text_elsewhere() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn d(&self, p: P, span: S) -> T { (p - self.foot_within(p, span)).norm() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_span_foot_as_point() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn m(l: L, p: P) -> P { l.foot_within(p, (0.0, 1.0)) }\n' \
    > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="a line's distance read as offset(…).norm()"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_reject
  gate_selftest_case "$want" plant_cross_across_lines
  gate_selftest_case "$want" plant_span_foot_norm
  gate_selftest_case "$want" plant_span_foot_margin
  gate_selftest_case "$want" plant_span_foot_reversed_deep
  gate_selftest_case "$want" plant_home_text_elsewhere
  gate_selftest_passes "a span foot read as the point it is" plant_span_foot_as_point
  gate_selftest_passes "the norm of a vector expression holding an offset" plant_offset_inside_an_expression
  gate_selftest_passes "an offset read as the vector it is" plant_vector_use
  printf '%s selftest OK: passes a clean fixture and an offset read as a vector; fires on offset(…).norm(), reject_from(…).norm(), a cross norm split over lines, the norm of a span foot split over lines and a span foot under Margin::norm3, one less the point with its span three parentheses deep, and the text of a home outside its file; passes a span foot read as a point and the norm of a vector expression; and stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
