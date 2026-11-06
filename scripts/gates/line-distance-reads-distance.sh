#!/usr/bin/env bash
# line-distance-reads-distance.sh — a point's distance from a
# `geom::Line` is `Line::distance`. ONE home; ci.yml's `lint` job runs
# every gate in this directory.
#
# THE RULE: a reader that needs only a point's distance from a line
# reads `Line::distance` (`|(p − origin) × dir| / |dir|`), never
# `Line::offset(p).norm()`. The two are the same quantity, but at
# `Interval` a point that carries width reads about 1.5× tighter
# through the cross product (`Line::distance`'s doc has the widths).
# `offset` stays for a reader that needs the vector: a radial
# direction, a gradient, a component along a normal.
#
# KNOWN GAP: the matcher reads one line, so a chain that rustfmt breaks
# between `.offset(…)` and `.norm()` passes. It also cannot tell a
# `geom::Line`'s `offset` from another type's, and no other type in
# the tree has an `offset(…)` whose `.norm()` is read, so a hit is
# this rule's until one does.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  gate_require_crate_sources
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E '\.offset\([^;]*\)\.norm\(\)')
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a line's distance read as offset(…).norm(): read Line::distance, the tight reading"
    exit 1
  fi
  gate_ok "no line distance read as offset(…).norm()"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn d(l: geom::Line<f64>, p: geom_core::Point3<f64>) -> f64 { l.offset(p).norm() }\n' \
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
  gate_selftest_passes "an offset read as the vector it is" plant_vector_use
  printf '%s selftest OK: passes a clean fixture and an offset read as a vector; fires on offset(…).norm(); and stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
