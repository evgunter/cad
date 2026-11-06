#!/usr/bin/env bash
# normalize-by-hand.sh — a direction is read through `Vec3::normalize`
# (or `Vec2::normalize`), and a direction with its length through
# `direction_and_length`, never spelled `v / v.norm()` by hand. ONE
# home; ci.yml's `lint` job runs every gate in this directory.
#
# THE RULE (`work/onecopy/a-direction-is-normalised-by-hand-beside-vec3-normalize.md`,
# `work/onecopy/a-direction-and-its-length-are-spelled-as-a-pair-at-23-sites.md`).
# `geom_core::Vec3::direction_and_length` (`crates/geom-core/src/linalg/vec.rs`)
# is the home of "this vector's direction and length", and `normalize`
# is its first half. Its body is `(self / n, n)`, `n = self.norm()`,
# wherever the largest component lies in `(2⁻⁵¹¹, 2⁵¹¹)`, so in range a
# hand spelling is the same bits, and outside it the home's prescale
# answers where the hand spelling yields a zero or infinite vector and
# an infinite or zero length. Production code takes the home; the
# home's own file is exempt.
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
#   * either, with the norm spelled `E.norm_squared().sqrt()`;
#   * THE PAIR: `let N = E.norm();` (or `E.norm_squared().sqrt()`), and
#     `E / N` or `E * (ONE / N)` in a later statement of the same file,
#     before the next `fn` and before `N` is bound again.
#
# BLIND SPOTS, each one a shape that passes:
#   * A PARENTHESISED OPERAND, `(q - p) / (q - p).norm()` or
#     `let d = (q - p).norm(); … (q - p) / d`: E is an identifier chain,
#     so a compound expression passes. The second pass for it,
#     `grep -rnP '(\([^()]*\))\s*/\s*\1\.norm\(\)' crates/*/src`,
#     is itself blind to NESTED parentheses inside the operand
#     (`(a - f(b)) / (a - f(b)).norm()`), since `[^()]*` stops at them.
#   * A CALL WITH ARGUMENTS in the chain: `c.deriv(t) / c.deriv(t).norm()`
#     passes, since a chain link is a name or a `()` call only.
#   * The pair's binding in any other form: a tuple or pattern binding,
#     a binding whose norm is taken of a different expression than the
#     one divided (`let n = w.norm(); … (w * 2.0) / n`), a use in a
#     closure body past the next `fn` keyword, and a division by a
#     guarded copy of the length (`v / n.select_le_zero(one, n)`), which
#     is a different zero behaviour and is meant to pass.
#   * A division through a helper, a closure or a macro; an `f64`
#     triple held as `[f64; 3]`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

HOME_FILE=crates/geom-core/src/linalg/vec.rs
HOME_SUBJECT='the home of a vector'"'"'s direction, Vec3::normalize'

# The pair matcher: reads the statement view (`path:line:text`) on stdin
# and prints each division that pairs with a `let N = E.norm();` above it.
# shellcheck disable=SC2016
PAIR_MATCHER='
import re, sys
CHAIN = r"[A-Za-z_][A-Za-z0-9_]*(?:\(\))?(?:\.[A-Za-z_][A-Za-z0-9_]*(?:\(\))?)*"
NORM = r"\s*\.\s*(?:norm\(\)|norm_squared\(\)\s*\.\s*sqrt\(\))\s*;?\s*$"
LET = re.compile(r"\blet\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\s*(?::[^=;]*)?=\s*(" + CHAIN + ")" + NORM)
FN = re.compile(r"\bfn\s")
ONE = r"(?:1\.0|[A-Za-z0-9_:<>]*::one\(\))"
path, live = None, {}
for record in sys.stdin:
    m = re.match(r"([^:]*):(\d+):(.*)$", record.rstrip("\n"))
    if not m:
        continue
    if m.group(1) != path:
        path, live = m.group(1), {}
    text = m.group(3)
    if FN.search(text):
        live = {}
    for name, expr in list(live.items()):
        e = re.escape(expr)
        if re.search(r"(?<![A-Za-z0-9_.])" + e + r"\s*(?:/\s*" + name + r"|\*\s*\(\s*" + ONE + r"\s*/\s*" + name + r"\s*\))(?![A-Za-z0-9_.(\[])", text):
            print(record.rstrip("\n"))
    for b in re.finditer(r"\blet\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\b", text):
        live.pop(b.group(1), None)
    let = LET.search(text)
    if let:
        live[let.group(1)] = let.group(2)
'

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
  local view hits pairs
  view=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")")
  hits=$(printf '%s\n' "$view" \
    | gate_grep -E "(^|[^A-Za-z0-9_.])($chain)[[:space:]]*(/[[:space:]]*\\2\\.$norm([^.?[]|\$)|\\*[[:space:]]*\\([[:space:]]*(1\\.0|[A-Za-z0-9_:<>]*::one\\(\\))[[:space:]]*/[[:space:]]*\\2\\.$norm[[:space:]]*\\))")
  if ! pairs=$(printf '%s\n' "$view" | python3 -c "$PAIR_MATCHER"); then
    gate_error "$(gate_name): the pair matcher failed under $PWD (its own message is above, if it had one) — the gate could not read its subject, which is not a pass"
    exit 1
  fi
  if [ -n "$pairs" ]; then
    printf '%s\n' "$pairs"
    gate_error "a direction and its length paired by hand (above): read them as let (dir, len) = v.direction_and_length(), the one home"
    exit 1
  fi
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

plant_pair() {
  printf 'pub fn dir(w: V) -> (V, f64) {\n    let n = w.norm();\n    let u = w / n;\n    (u, n)\n}\n' >> "$1/crates/clean/src/lib.rs"
}

plant_pair_reciprocal() {
  printf 'pub fn dir(w: V) -> Option<V> {\n    let n = w.norm_squared().sqrt();\n    (n > 0.0).then(|| w * (1.0 / n))\n}\n' >> "$1/crates/clean/src/lib.rs"
}

plant_pair_field_path() {
  printf 'pub fn dir(s: S) -> V {\n    let len = s.frame.u_ref.norm();\n    F { u: s.frame.u_ref / len, len }\n}\n' >> "$1/crates/clean/src/lib.rs"
}

# A pair's name bound again, read past the next `fn`, divided into a
# different expression, or behind a guarded divisor is not the pair.
plant_not_pairs() {
  {
    printf 'pub fn a(w: V, x: V) -> V {\n    let n = w.norm();\n    let n = x.norm();\n    w / n\n}\n'
    printf 'pub fn b(w: V) -> f64 {\n    let n = w.norm();\n    n\n}\npub fn c(w: V, n: f64) -> V {\n    w / n\n}\n'
    printf 'pub fn d(w: V, x: V) -> V {\n    let n = w.norm();\n    x / n\n}\n'
    printf 'pub fn e(w: V) -> V {\n    let n = w.norm();\n    w / n.select_le_zero(1.0, n)\n}\n'
  } >> "$1/crates/clean/src/lib.rs"
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
  local pair="a direction and its length paired by hand"
  gate_selftest_case "$pair" plant_pair
  gate_selftest_case "$pair" plant_pair_reciprocal
  gate_selftest_case "$pair" plant_pair_field_path
  gate_selftest_passes "a rebound name, a use past the next fn, another numerator and a guarded divisor" plant_not_pairs
  gate_selftest_passes "a longer divisor, other operands, prose, a test module, fixture files and the home" plant_not_sites
  gate_selftest_homes --narrowed --subject "$HOME_SUBJECT" "$HOME_FILE"
  printf '%s selftest OK: passes a clean fixture carrying the home, and a longer divisor, other operands, prose, a test module and fixture files; fires on v / v.norm() by name, by field path, wrapped across lines and through norm_squared().sqrt(), and on v * (1 / v.norm()) with T::one() and 1.0; fires on the pair let n = v.norm(); … v / n by name, by field path and as a reciprocal through norm_squared().sqrt(), and passes a rebound name, a use past the next fn, another numerator and a guarded divisor; and stays RED when grep cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
