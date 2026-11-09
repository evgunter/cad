#!/usr/bin/env bash
# point-equality-reads.sh — two points (or vectors) compared exactly are
# compared through geom-core's two readers, each named for its question.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE: `Point3::equals_exactly` (and its `Point2`, `Vec2`, `Vec3`
# twins) asks whether the difference is zero: `+0.0` equals `-0.0`, and a
# NaN equals nothing. `Point3::same_bits` asks whether the representation
# is one: a dedupe key, a reproducibility check. Their docs say which to
# use when. Four hand spellings are refused everywhere else, tests
# included, since a test's equality is the same question:
#
#   * `(a - b).norm() == 0.0` (or `(a + b)`), which is NOT exact: the
#     squared norm underflows, so points about 1e-170 apart read as one;
#   * componentwise `a.x == b.x && a.y == b.y …`;
#   * `a.to_array() == b.to_array()`;
#   * componentwise `a.x.to_bits() == b.x.to_bits() && … .y.to_bits()`.
#
# KNOWN GAPS: a single vector's `v.norm() == 0.0` passes, being the
# question "is this vector zero", not an equality of two; a comparison against coordinates that are not a second
# point's (`p.x.to_bits() == u.to_bits()`, a point against two scalars)
# is not a point equality and passes; a componentwise spelling through
# differently named operands per coordinate passes; a `.norm()` compared
# with `<= 0.0` passes; a whole-array compare through a local binding
# (`let a = p.to_array(); a == q`) passes.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILES='crates/geom-core/src/linalg/(point|vec)\.rs'
NAME='[[:alnum:]_.()]+'
SHAPES="\\([^;]* [-+] [^;]*\\)\\.norm\\(\\)[[:space:]]*==[[:space:]]*0\\.0|to_array\\(\\)[[:space:]]*==|($NAME)\\.x[[:space:]]*==[[:space:]]*($NAME)\\.x[^&|]*&&[[:space:]]*\\1\\.y[[:space:]]*==[[:space:]]*\\2\\.y|\\.x\\.to_bits\\(\\)[[:space:]]*==[^&|]*\\.x\\.to_bits\\(\\)[^;]*\\.y\\.to_bits\\(\\)[[:space:]]*==[^&|;]*\\.y\\.to_bits\\(\\)"

gate() {
  gate_require_crate_sources
  local files code hits
  mapfile -t files < <(find crates/*/src crates/*/tests crates/*/examples crates/*/benches \
    -type f -name '*.rs' -not -path '*/target/*' 2>/dev/null | sort)
  GATE_SCAN_FILES=${#files[@]}
  if ! code=$(gate_rust_code --statements "${files[@]}"); then
    gate_error "$(gate_name): could not read the sources under $PWD — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  hits=$(gate_grep -E "$SHAPES" <<<"$code" | gate_grep -vE "^$HOME_FILES:")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a point equality spelled by hand: read Point3::equals_exactly (the value) or Point3::same_bits (the representation)"
    exit 1
  fi
  gate_ok "no point equality spelled by hand outside geom-core's two readers"
}

plant() { mkdir -p "$1/crates/planted/src"; printf '%s\n' "$2" > "$1/crates/planted/src/lib.rs"; }
plant_norm() { plant "$1" 'pub fn e(a: P, b: P) -> bool { (a - b).norm() == 0.0 }'; }
plant_components() { plant "$1" 'pub fn e(a: P, b: P) -> bool { a.x == b.x && a.y == b.y && a.z == b.z }'; }
plant_array() { mkdir -p "$1/crates/planted/tests"; printf 'fn e(a: P, b: P) -> bool { a.to_array() == b.to_array() }\n' > "$1/crates/planted/tests/t.rs"; }
plant_bits() { plant "$1" $'pub fn e(a: P, b: P) -> bool {\n    a.x.to_bits() == b.x.to_bits()\n        && a.y.to_bits() == b.y.to_bits()\n}'; }
plant_scalars() { plant "$1" 'pub fn e(m: P, u: f64, v: f64) -> bool { m.x.to_bits() == u.to_bits() && m.y.to_bits() == v.to_bits() }'; }
plant_zero_vector() { plant "$1" 'pub fn z(a: V, b: V) -> bool { a.cross(b).norm() == 0.0 }'; }
plant_reader() { plant "$1" 'pub fn e(a: P, b: P) -> bool { a.equals_exactly(b) || a.same_bits(b) }'; }

gate_selftest() {
  local want="a point equality spelled by hand"
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_norm
  gate_selftest_case "$want" plant_components
  gate_selftest_case "$want" plant_array
  gate_selftest_case "$want" plant_bits
  gate_selftest_passes "a point's coordinates against two scalars" plant_scalars
  gate_selftest_passes "the two readers" plant_reader
  gate_selftest_passes "a single vector read zero" plant_zero_vector
  printf '%s selftest OK: fires on norm() == 0.0, componentwise ==, to_array() == (in a test) and componentwise to_bits split over lines; passes the two readers, a single vector read zero and a point against two scalars; stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
