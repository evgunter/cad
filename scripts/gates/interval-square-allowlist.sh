#!/usr/bin/env bash
# interval-square-allowlist.sh — the interval-square `powi(2)` allowlist
# (ratified 2026-08-01). ONE home; ci.yml's "interval-square powi(2)
# allowlist (ratified 2026-08-01)" step and local-scripts/ci-local.sh's
# discipline row both call this file. Kernel comments that name that
# step name still resolve: the step is still there, and it runs this.
#
# Interval-square discipline tripwire — the rule lives HERE, and
# the kernel's comments point at this step. FOUR live bugs came
# from this one class: squaring via plain `x * x` treats the
# factors as independent, so a zero-straddling enclosure gets a
# spurious negative lower bound that poisons downstream
# sqrt/decoration. `Real::powi(2)` (inari `pown`) knows both
# factors are the same variable and returns the tight nonnegative
# enclosure with the decoration preserved. Squares of
# definitely-nonzero singletons (stored radii, bulges) may stay as
# `*`. Reviewing new geometry: this grep only sees production code
# under `crates/*/src`, so also eyeball predicate-path diffs for
# `* self`, `x * x`, and `.dot(` on possibly-zero vectors.
#
# WHAT THE MATCHER SEES, and why the shape changed. The operand is a
# FIELD PATH, not a bare identifier: `self.x * self.x` is the commonest
# spelling of this bug in vector code and the bare-identifier
# backreference could not see it at all — the live instance it walked
# past for two milestones was `linalg/vec.rs`'s `orthonormal_basis`,
# production code generic over `Real` in a file nobody had allowlisted.
# And the second operand may not be followed by `.`, `(` or `[`, because
# `a * a.norm()` is not a square and the gate used to say it was. Those
# two failures compound: a false red is a nudge toward the allowlist
# rather than the fix, and this gate is where that already happened —
# `linalg/mat.rs`'s entry was justified in writing partly by `r *
# r.transpose()` test hits that were never violations.
#
# KNOWN GAP 1: the scan is PRODUCTION CODE ONLY — `#[cfg(test)]` items
# are dropped, and so is a module file whose `mod` declaration is
# `#[cfg(test)]`-gated. A `x * x` inside a test cannot poison a shipped
# enclosure, and scanning them is what produced the false positives
# above. The cost is that a test-only helper later promoted to
# production arrives unscanned; the promotion is a diff a human reads.
#
# KNOWN GAP 2: an operand starting with an uppercase letter is invisible
# (`SOME_CONST * SOME_CONST`). Deliberate: the ALL-CAPS population in
# this tree is `usize` buffer sizing, where the rule does not apply and
# a red would be pure cry-wolf.
#
# KNOWN GAP 3: an INDEXED square (`v[i] * v[i]`) and a repeated CALL
# (`f(t) * f(t)`) are both invisible. The first is a real hole; the
# second is deliberate, since a repeated call is not obviously one
# value.
#
# KNOWN GAP 4: the allowlist is FILE-granular while its reasons are
# per-seam, so a second unrelated `x * x` added to an allowlisted file
# inherits the first entry's ratification silently. That is S159/D103's
# class and it is not closed here.
#
# Allowlist rationale, per file:
#  - geom-core real.rs / ring_interval.rs — the scalar implementations
#    themselves: `x * x` is definitional (powi is BUILT from it) and
#    their tests deliberately contrast the plain product with the tight
#    square;
#  - geom-core linalg/mat.rs — `rotation_about`'s evaluation order
#    is D9-ratified and doc'd in-file ("exactly as parenthesized");
#    tightening `t·x·x` to `t·x²` is a design change, not a cleanup;
#  - geom-core linalg/svd.rs / lsq.rs — documented f64-only
#    selection lanes (lsq's hits are usize buffer sizing);
#  - geom-brep ssi/jet.rs / march.rs / system.rs — f64-only jet and
#    marcher numerics (finite differences, step control).
#
# `interval.rs` and `dual.rs` were on this list and are NOT any more:
# once the scan stopped reading test modules, neither file had a
# production hit left. An allowlist entry with nothing behind it is a
# ratification waiting to be inherited by the next line added to the
# file.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# `x * x` where x is an identifier or a field path. The trailing
# look-ahead is the half that stops `a * a.method()` from reading as a
# square.
SQUARE_RE='(?<![\w.])((?:[a-z_][a-z0-9_]*)(?:\.[a-z_][a-z0-9_]*)*)\s*\*\s*\1(?![\w.(\[])'

# A module whose `mod` declaration is `#[cfg(test)]`-gated is test-only
# code that happens to live in its own file, and the reader's per-item
# skip cannot see across files. Resolved here, from the declaration
# rather than from a list of names, so a new one needs no edit.
production_sources() {
  local decl dir name cands=() excl=()
  # TWO STAGES, because reading every source twice costs more than this
  # gate is worth: raw `grep` narrows to the handful of files that carry
  # both the attribute and a `mod` declaration, and only those are read
  # properly. A file that fails the raw narrowing carries no
  # `#[cfg(test)]` text at all, so it cannot carry a gated declaration.
  mapfile -t cands < <(grep -lF '#[cfg(test)]' "${GATE_SOURCE_FILES[@]}" \
    | xargs -r grep -lE '^[[:space:]]*(pub[[:space:]]+)?mod [a-z_][a-z0-9_]*;' || true)
  if [ "${#cands[@]}" -gt 0 ]; then
    while IFS= read -r decl; do
      [ -n "$decl" ] || continue
      dir=${decl%%:*}; dir=${dir%/*}
      name=${decl##*:}
      excl+=("$dir/$name.rs" "$dir/$name/")
    done < <(gate_rust_code "${cands[@]}" \
      | grep -A1 -F '#[cfg(test)]' \
      | grep -oE '^[^:]*:[0-9]+:[[:space:]]*(pub[[:space:]]+)?mod [a-z_][a-z0-9_]*;' \
      | sed -E 's/:[0-9]+:.*mod /:/; s/;$//' || true)
  fi
  if [ "${#excl[@]}" -eq 0 ]; then
    printf '%s\n' "${GATE_SOURCE_FILES[@]}"
    return 0
  fi
  printf '%s\n' "${GATE_SOURCE_FILES[@]}" \
    | grep -vF "$(printf '%s\n' "${excl[@]}")" || true
}

gate() {
  gate_require_crate_sources
  local files hits
  mapfile -t files < <(production_sources)
  GATE_SCAN_FILES=${#files[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): every source under crates/*/src in $PWD is test-only — the gate scanned no production code, which is not a pass"
    exit 1
  fi
  hits=$(gate_rust_code --skip-cfg-test "${files[@]}" \
    | grep -P "$SQUARE_RE" \
    | grep -vE '^crates/geom-core/src/(real|ring_interval)\.rs:' \
    | grep -vE '^crates/geom-core/src/linalg/(mat|svd|lsq)\.rs:' \
    | grep -vE '^crates/geom-brep/src/ssi/(jet|march|system)\.rs:' || true)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "use powi(2): it is never wider than x*x and strictly tighter when the enclosure straddles zero; whether THIS enclosure can straddle zero is a global property of upstream callers that refactors change silently — four live bugs arrived exactly that way. Convert, or ratify this file into the allowlist."
    exit 1
  fi
  gate_ok "no unratified x*x outside the allowlisted files"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/lib.rs"
}

# THE SPELLING THE MATCHER USED TO WALK PAST, and the one this gate
# exists for: a field path, which is how vector code writes a square.
plant_field_path() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(v: Vec3<T>) -> T { v.y * v.y }\n' > "$1/crates/planted/src/lib.rs"
}

plant_self_field() {
  mkdir -p "$1/crates/planted/src"
  printf 'impl<T: Real> V<T> { pub fn n(self) -> T { self.x * self.x } }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_nested_field_path() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(s: S<T>) -> T { s.dir.z * s.dir.z }\n' > "$1/crates/planted/src/lib.rs"
}

# Behind a block comment on one line — the strip has to be real.
plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(x: T) -> T { /* why */ x * x }\n' > "$1/crates/planted/src/lib.rs"
}

# A test-only module in its own file, registered WITHOUT the cfg gate:
# ordinary production code, and it must fire.
plant_ungated_module_file() {
  mkdir -p "$1/crates/planted/src"
  printf 'mod probes;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
}

# THE NEAR MISSES, bundled: in the must-NOT-fire direction any one line
# firing fails the case, so a bundle is strictly stronger than separate
# fixtures. Every line here is correct code, or prose, or test-only.
plant_not_squares() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '// the rule forbids x * x here\n'
    printf '/// and `self.y * self.y` in a doc comment\n'
    printf '/*\n * and v.z * v.z inside a block comment\n */\n'
    printf 'pub const S: &str = "x * x";\n'
    printf 'pub fn a<T: Real>(x: T) -> T { x * x.recip() } // and a * a.m()\n'
    printf 'pub fn b<T: Real>(v: V<T>) -> T { v.x * v.y }\n'
    printf 'pub fn c<T: Real>(v: V<T>) -> T { v.x * v.x.abs() }\n'
    printf 'pub fn d<T: Real>(v: V<T>) -> T { v.norm() * v.norm() }\n'
    printf 'pub fn e<T: Real>(x: T) -> T { x.powi(2) }\n'
    printf '#[cfg(test)]\nmod tests {\n    fn t() { let q = 2.0; let _ = q * q; }\n}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# The same square, in a module file whose declaration is cfg(test)-gated:
# test-only code, and it must NOT fire. This case and the one above are
# the two directions of the same rule, and neither is evidence without
# the other.
plant_gated_module_file() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)]\nmod probes;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
}

gate_selftest() {
  local want="use powi(2)"
  gate_selftest_clean
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_field_path
  gate_selftest_case "$want" plant_self_field
  gate_selftest_case "$want" plant_nested_field_path
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_case "$want" plant_ungated_module_file
  gate_selftest_passes "prose, string literals, mixed products, a * a.method(), and a cfg(test) module" plant_not_squares
  gate_selftest_passes "a square in a module file whose declaration is cfg(test)-gated" plant_gated_module_file
  printf '%s selftest OK: passes a clean fixture, prose/strings/mixed products/`a * a.method()`/a cfg(test) module, and a test-only module file; fires on a bare identifier, a field path, a `self.` field, a two-level path, a square behind a block comment, and the same module file registered WITHOUT the cfg gate\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
