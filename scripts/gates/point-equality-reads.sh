#!/usr/bin/env bash
# point-equality-reads.sh — two points (or vectors) compared exactly are
# compared through geom-core's two readers, each named for its question.
# ONE home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE: `Point3::equals_exactly` (and its `Point2`, `Vec2`, `Vec3`
# twins) asks whether the difference is zero: `+0.0` equals `-0.0`, and a
# NaN equals nothing. `Point3::same_bits` asks whether the representation
# is one: a dedupe key, a reproducibility check. Their docs say which to
# use when. The hand spellings are refused everywhere else, tests
# included, since a test's equality is the same question:
#
#   * the zero read of a difference's or sum's `.norm()` or
#     `.norm_squared()`, or of a `.distance(…)` or `.distance_squared(…)`,
#     by `==`, `!=` or
#     `<= 0.0`, either way round: NOT exact, since the squared norm
#     underflows and points about 1e-170 apart read as one;
#   * componentwise `a.x == b.x && a.y == b.y …`, or its `!=`/`||`
#     negation;
#   * array or tuple literals `[a.x, a.y, …] == [b.x, …]`,
#     `(a.x, a.y, …) != (b.x, …)`;
#   * `to_array() ==`/`!=`, and `assert_eq!`/`assert_ne!` of two
#     `to_array()`s (with or without `.map(f64::to_bits)`);
#   * componentwise `.x.to_bits() ==`/`!=` … `.y.to_bits()`.
#
# KNOWN GAPS: a single vector's `v.norm() == 0.0` passes, being the
# question "is this vector zero", not an equality of two; a comparison
# against coordinates that are not a second point's (`p.x.to_bits() ==
# u.to_bits()`, a point against two scalars) is not a point equality and
# passes; a componentwise spelling through differently named operands
# per coordinate passes; a whole-array compare through a local binding
# (`let a = p.to_array(); a == q`) passes; `.distance(…)` is read on
# its name, so a line's `distance(p) == 0.0` (a point on a line, a
# different question) would fire, and none is in the tree.
#
# FALSE FIRES (none in the tree; each would be routed or reworded): a
# componentwise compare of two structs that are not points but carry `x`
# and `y` fields; `to_array() == [a literal array]`, a point against
# literal coordinates, which `equals_exactly(Point3::from_array(…))`
# says as well; a grouping parenthesis holding a difference that is not
# two points' (`(s - t).norm() == 0.0` over two vectors whose difference
# is the question).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILES='crates/geom-core/src/linalg/(point|vec)\.rs'
NAME='[[:alnum:]_.()*&]+'
ZERO='0\.0(_f64)?'
CMP='(==|!=|<=)'
RCMP='(==|!=|>=)'
NORM='\.(norm|norm_squared)\(\)'
# A difference's or sum's norm, or a distance, read zero, either way round.
# A grouping parenthesis, not a call's: none follows a name, and its
# content is balanced to one level, so `c.eval(t + 1.0).norm()` is a
# vector's norm and passes.
BAL='([^()]|\([^()]*\))*'
DIFF="(^|[^[:alnum:]_])\\($BAL [-+] $BAL\\)$NORM"
DIST="\\.distance(_squared)?\\($BAL\\)"
ZEROED="($DIFF|$DIST)[[:space:]]*$CMP[[:space:]]*$ZERO|$ZERO[[:space:]]*$RCMP[^;]*($DIFF|$DIST)"
# Componentwise, as a chain or as array literals; whole arrays; bits.
COMPONENTS="($NAME)\\.x[[:space:]]*(==|!=)[[:space:]]*($NAME)\\.x[^;]*(&&|\\|\\|)[[:space:]]*\\1\\.y[[:space:]]*(==|!=)[[:space:]]*\\3\\.y"
# Both sides a point's coordinates: a point against literal coordinates
# is a point against scalars, and passes.
LITERALS="\\[[[:space:]]*$NAME\\.x,[[:space:]]*$NAME\\.y[^]]*\\][[:space:]]*(==|!=)[[:space:]]*\\[[[:space:]]*$NAME\\.x"
TUPLES="\\([[:space:]]*$NAME\\.x,[[:space:]]*$NAME\\.y[^)]*\\)[[:space:]]*(==|!=)[[:space:]]*\\([[:space:]]*$NAME\\.x"
ARRAYS="to_array\\(\\)[[:space:]]*(==|!=)|assert_(eq|ne)!\\([^;]*to_array\\(\\)[^;]*,[^;]*to_array\\(\\)"
BITS="\\.x\\.to_bits\\(\\)[[:space:]]*(==|!=)[^;]*\\.x\\.to_bits\\(\\)[^;]*\\.y\\.to_bits\\(\\)[[:space:]]*(==|!=)[^;]*\\.y\\.to_bits\\(\\)"
# One pass per shape: COMPONENTS numbers its own back references.
SHAPES=("$ZEROED" "$COMPONENTS" "$LITERALS" "$TUPLES" "$ARRAYS" "$BITS")

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
  local shape
  hits=$(for shape in "${SHAPES[@]}"; do gate_grep -E "$shape" <<<"$code"; done \
    | sort -u | gate_grep -vE "^$HOME_FILES:")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a point equality spelled by hand: read Point3::equals_exactly (the value) or Point3::same_bits (the representation)"
    exit 1
  fi
  gate_ok "no point equality spelled by hand outside geom-core's two readers"
}

plant() { mkdir -p "$1/crates/planted/src"; printf '%s\n' "$2" > "$1/crates/planted/src/lib.rs"; }
plant_norm() { plant "$1" 'pub fn e(a: P, b: P) -> bool { (a - b).norm() == 0.0 }'; }
plant_norm_reversed() { plant "$1" 'pub fn e(a: P, b: P) -> bool { 0.0 != (a - b).norm_squared() }'; }
plant_norm_at_most() { plant "$1" 'pub fn e(a: P, b: P) -> bool { (a + b).norm() <= 0.0 }'; }
plant_distance_squared() { plant "$1" 'pub fn e(a: P, b: P) -> bool { 0.0 == a.distance_squared(b) }'; }
plant_tuples() { plant "$1" 'pub fn e(a: P, b: P) -> bool { (a.x, a.y) != (b.x, b.y) }'; }
plant_literal_coordinates() { plant "$1" 'pub fn e(p: P) -> bool { (p.x, p.y, p.z) == (1.0, 0.0, 0.0) || [p.x, p.y] == [1.0, 0.0] }'; }
plant_call_argument() { plant "$1" 'pub fn z(c: C, t: f64) -> bool { assert!(c.eval(t + 1.0).norm() == 0.0); true }'; }
plant_distance() { plant "$1" 'pub fn e(a: P, b: P) -> bool { a.distance(b) == 0.0 }'; }
plant_components() { plant "$1" 'pub fn e(a: P, b: P) -> bool { a.x == b.x && a.y == b.y && a.z == b.z }'; }
plant_components_apart() { plant "$1" 'pub fn e(a: P, b: P) -> bool { a.x != b.x || a.y != b.y }'; }
plant_literals() { plant "$1" 'pub fn e(a: P, b: P) -> bool { [a.x, a.y, a.z] == [b.x, b.y, b.z] }'; }
plant_array() { mkdir -p "$1/crates/planted/tests"; printf 'fn e(a: P, b: P) -> bool { a.to_array() == b.to_array() }\n' > "$1/crates/planted/tests/t.rs"; }
plant_array_assert() { plant "$1" $'fn t(a: P, b: P) {\n    assert_eq!(\n        a.to_array().map(f64::to_bits),\n        b.to_array().map(f64::to_bits)\n    );\n}'; }
plant_bits() { plant "$1" $'pub fn e(a: P, b: P) -> bool {\n    a.x.to_bits() == b.x.to_bits()\n        && a.y.to_bits() == b.y.to_bits()\n}'; }
plant_scalars() { plant "$1" 'pub fn e(m: P, u: f64, v: f64) -> bool { m.x.to_bits() == u.to_bits() && m.y.to_bits() == v.to_bits() }'; }
plant_zero_vector() { plant "$1" 'pub fn z(a: V, b: V) -> bool { a.cross(b).norm() == 0.0 }'; }
plant_reader() { plant "$1" 'pub fn e(a: P, b: P) -> bool { a.equals_exactly(b) || a.same_bits(b) }'; }

gate_selftest() {
  local want="a point equality spelled by hand" planter
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"
  for planter in plant_norm plant_norm_reversed plant_norm_at_most plant_distance \
    plant_distance_squared plant_components plant_components_apart plant_literals plant_tuples \
    plant_array plant_array_assert plant_bits; do
    gate_selftest_case "$want" "$planter"
  done
  gate_selftest_passes "the two readers" plant_reader
  gate_selftest_passes "a single vector read zero" plant_zero_vector
  gate_selftest_passes "a vector at a call whose argument is a sum, read zero" plant_call_argument
  gate_selftest_passes "a point against literal coordinates, as a tuple or an array" plant_literal_coordinates
  gate_selftest_passes "a point's coordinates against two scalars" plant_scalars
  printf '%s selftest OK: fires on a difference'"'"'s norm read zero (==, a reversed !=, <=, norm_squared), distance(…) == 0.0 and a reversed distance_squared, componentwise == and its != negation, array and tuple literals, to_array() == (in a test) and an assert_eq! of to_bits arrays, and componentwise to_bits split over lines; passes the two readers, a single vector read zero (also at a call whose argument is a sum) a point against two scalars, and a point against literal coordinates; stays RED, with a diagnosis, when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
