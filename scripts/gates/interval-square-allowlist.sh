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
# `*`. Reviewing new geometry: this grep only sees `crates/*/src`,
# so also eyeball predicate-path diffs for `* self`, `x * x`, and
# `.dot(` on possibly-zero vectors. Comment lines are
# stripped (same as the Bounds step). Allowlist rationale, per file:
#  - geom-core real.rs / interval.rs / dual.rs / ring_interval.rs —
#    the scalar implementations themselves: `x * x` is definitional
#    (powi is BUILT from it) and their tests deliberately contrast
#    the plain product with the tight square;
#  - geom-core linalg/mat.rs — `rotation_about`'s evaluation order
#    is D9-ratified and doc'd in-file ("exactly as parenthesized");
#    tightening `t·x·x` to `t·x²` is a design change, not a cleanup
#    (the test hits are `r * r.transpose()` matrix products);
#  - geom-core linalg/svd.rs / lsq.rs — documented f64-only
#    selection lanes (lsq's hits are usize buffer sizing);
#  - geom-brep ssi/jet.rs / march.rs / system.rs — f64-only jet and
#    marcher numerics (finite differences, step control).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  local hits
  hits=$(grep -rPn '\b([a-z_][a-z0-9_]*)\s*\*\s*\1\b' crates/*/src --include='*.rs' \
    | grep -vE ':[0-9]+:\s*(//|///|//!)' \
    | cut -d: -f1 | sort -u \
    | grep -vE '^crates/geom-core/src/(real|interval|dual|ring_interval)\.rs$' \
    | grep -vE '^crates/geom-core/src/linalg/(mat|svd|lsq)\.rs$' \
    | grep -vE '^crates/geom-brep/src/ssi/(jet|march|system)\.rs$' || true)
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "use powi(2): it is never wider than x*x and strictly tighter when the enclosure straddles zero; whether THIS enclosure can straddle zero is a global property of upstream callers that refactors change silently — four live bugs arrived exactly that way. Convert, or ratify this file into the allowlist."
    exit 1
  fi
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn sq<T: Real>(x: T) -> T { x * x }\n' > "$1/crates/planted/src/lib.rs"
}

gate_parse_args "$@"
gate_main "use powi(2)" plant
