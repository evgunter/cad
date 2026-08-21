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
# sqrt/decoration. `Real::powi(2)` knows both factors are the same
# variable and returns the tight nonnegative enclosure with the
# decoration preserved. Squares of definitely-nonzero singletons
# (stored radii, bulges) may stay as `*` — for a `lo >= 0`
# enclosure the plain product's four-corner minimum ALREADY is the
# tight square, so the conversion is a no-op there and the reason
# to make it is uniformity, not width.
#
# "NEVER WIDER" IS FALSE AS WRITTEN, AND THIS TEXT USED TO SAY IT.
# It also named `inari` as the backend, which it has not been since
# M5 PR 1 — the backend is the in-repo `interval-transcendentals`
# crate. On that backend `powi(2)` is **1 ulp wider on each side
# once the square drops below `TWO_PROD_VALID_MIN = 2^-960`**, i.e.
# `|x| < 2^-480 ~ 3.2e-145: the closing `mul_hi(1.0, base)` has no
# 2Prod witness that can certify a sub-2^-960 product, so it pads a
# second time. Measured at `x = 2^-481*1.5`; **0 widening cases in
# 3,000,000 samples with `|x|` in [1e-60, 1e60]**, so it is
# unreachable in the live regime — which is a reason to state it
# once, not a reason to keep saying "never". A dead backend named
# in a numerics justification is S39/S112's class in the worst
# place it can land. Reviewing new geometry: this grep only sees production code
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
# KNOWN GAP 1: the scan is PRODUCTION CODE ONLY — a `#[cfg(test)]` item
# is dropped, and so is a module file whose `mod` declaration is one. A
# `x * x` inside a test cannot poison a shipped enclosure, and scanning
# them is what produced the false positives above. **Only a TEST-ONLY
# attribute counts**: `any(test, …)` and `not(test)` are both scanned,
# because an `any(debug_assertions, test, …)` module is every debug
# build — `topo`'s `test_support_impl` is exactly that, and an earlier
# draft skipped it. The cost is that a test-only helper later promoted
# to production arrives unscanned; the promotion is a diff a human
# reads.
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
# THE SCAN IS STATEMENT-SCOPED, not line-scoped, because `rustfmt` wraps
# a long product: `v.long_field\n    * v.long_field` is one expression
# and two lines, and a line matcher sees neither operand beside the
# other. Same ruling as S158's, same reason it is worth stating twice —
# the wrapped form is what the formatter PRODUCES from the caught one.
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
#  - geom-core linalg/mat.rs — `rotation_about` writes `t * x * x`,
#    which is a SCALED square: tightening it reassociates `(t·x)·x`
#    into `t·(x²)` rather than replacing a product by a square. That
#    reassociation is **ratified** (Evan, 2026-08-21 — D9 is
#    determinism at one kernel, not a pin on last year's output), so
#    this entry is a SCHEDULED RESIDUE, not a settled exemption: it
#    holds until D109(a) converts the site and re-cuts whatever
#    goldens `t = 1 − cos θ` moves. Do not widen the matcher to see
#    `(k·x)·x` before that lands — it reds this file and
#    `linalg/vec.rs` together, and greening that by allowlisting is
#    the outcome this gate's own history is a record of;
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
  #
  # The declaration is matched over the STATEMENT view, which is what
  # makes `#[cfg(test)] mod probes;` on ONE line and the same split over
  # two the same record — the earlier `grep -A1` form saw only the split
  # one, and cried wolf on the other.
  mapfile -t cands < <(grep -lF '#[cfg(test)]' "${GATE_SOURCE_FILES[@]}" \
    | xargs -r grep -lE '(^|[[:space:]])mod [a-z_][a-z0-9_]*;' || true)
  if [ "${#cands[@]}" -gt 0 ]; then
    while IFS= read -r decl; do
      [ -n "$decl" ] || continue
      dir=${decl%%:*}; dir=${dir%/*}
      name=${decl##*:}
      excl+=("$dir/$name.rs" "$dir/$name/")
    done < <(gate_rust_code --statements "${cands[@]}" \
      | grep -E '#\[cfg\(([^]]*[(,][[:space:]]*)?test[,)]' \
      | grep -vE '#\[cfg\([^]]*(any|not)\(' \
      | grep -oE '^[^:]*:[0-9]+:.*[[:space:]]mod [a-z_][a-z0-9_]*$' \
      | sed -E 's/:[0-9]+:.*[[:space:]]mod /:/' || true)
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
  hits=$(gate_rust_code --skip-cfg-test --statements "${files[@]}" \
    | grep -P "$SQUARE_RE" \
    | grep -vE '^crates/geom-core/src/(real|ring_interval)\.rs:' \
    | grep -vE '^crates/geom-core/src/linalg/(mat|svd|lsq)\.rs:' \
    | grep -vE '^crates/geom-brep/src/ssi/(jet|march|system)\.rs:' || true)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "use powi(2): it is strictly tighter than x*x when the enclosure straddles zero, and equal elsewhere except for a square below 2^-960, where the backend pads once more (see this gate's header — NOT 'never wider'). Whether THIS enclosure can straddle zero is a global property of upstream callers that refactors change silently — four live bugs arrived exactly that way. Convert, or ratify this file into the allowlist."
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

# WHAT RUSTFMT MAKES OF A LONG PRODUCT. One expression, two lines, and
# a line-scoped matcher sees neither operand beside the other.
plant_wrapped_product() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn sq<T: Real>(v: LongTypeName<T>) -> T {\n'
    printf '    v.some_rather_long_field_name\n'
    printf '        * v.some_rather_long_field_name\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# A test-only module declared on ONE line. The `grep -A1` form that
# preceded the statement view saw only the two-line spelling and cried
# wolf on this one.
plant_gated_module_file_one_line() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(test)] mod probes;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
}

# `any(test, …)` is NOT test-only: an `any(debug_assertions, test, …)`
# module is every debug build, so its square is production code.
plant_any_gated_module_file() {
  mkdir -p "$1/crates/planted/src"
  printf '#[cfg(any(debug_assertions, test))]\nmod probes;\n' > "$1/crates/planted/src/lib.rs"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/probes.rs"
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
  gate_selftest_case "$want" plant_any_gated_module_file
  gate_selftest_case "$want" plant_wrapped_product
  gate_selftest_passes "prose, string literals, mixed products, a * a.method(), and a cfg(test) module" plant_not_squares
  gate_selftest_passes "a square in a module file whose declaration is cfg(test)-gated" plant_gated_module_file
  gate_selftest_passes "the same, declared on one line" plant_gated_module_file_one_line
  printf '%s selftest OK: passes a clean fixture, prose/strings/mixed products/`a * a.method()`/a cfg(test) module, and a test-only module file declared on one line or two; fires on a bare identifier, a field path, a `self.` field, a two-level path, a rustfmt-wrapped product, a square behind a block comment, the same module file registered WITHOUT the cfg gate, and one registered under any(debug_assertions, test), which is every debug build\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
