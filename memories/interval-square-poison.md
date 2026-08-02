---
name: interval-square-poison
description: Any interval square of a possibly-zero-straddling quantity MUST use powi(2)/pown, never x*x — plain Mul gives a spurious negative lower bound that poisons downstream sqrt/decoration
metadata:
  type: project
---

**The bug class (three independent occurrences in M2 alone):** squaring an
interval enclosure via plain multiplication (`x * x`, `v.dot(v)`) treats the
factors as independent: a straddling-zero enclosure `[-a, b]` squares to a
lower bound of `-ab` instead of `0`. A downstream `sqrt` then clamps its
domain, the inari decoration degrades below `Def`, and `Decide` correctly
reads poison — the predicate refuses geometry it should accept (or an entire
lane refuses all inexact inputs).

Occurrences: (1) `Vec2/Vec3::norm_squared` — PR 3 review BLOCKER B1, found
convergently by the PR 4 implementer; (2) torus `implicit_residual`
(d²+h² straddle) — B1's sibling audit; (3) `props_rim_level` in PR 7's mass
properties — found live on the donut.

**Why:** the true range of x² is [0, max(a²,b²)]; the dedicated power
operation (`Real::powi(2)`, inari `pown`) knows both factors are the same
variable and returns the tight nonnegative enclosure with decoration `Com`
preserved.

**How to apply:** every interval-lane square of a quantity that can straddle
zero (components, dot-with-self, residual differences like d²−r²) goes
through `powi(2)` — never `*`. Squares of definitely-nonzero singletons
(stored radii, bulges) may stay as `*`. When reviewing: grep new geometry
code for `* self`, `.dot(` on possibly-zero vectors, and `x * x` patterns in
certification/predicate paths. Bit-identity caveat (PR 4 NIT-1): the f64 and
Dual VALUE channels are unconditionally bit-identical under this rewrite,
but the Dual DERIVATIVE channel is not (subnormal/overflow witnesses exist);
tangents never decide (D8), so this is doc-scope only.

**Enforced by CI** (M5, after the fourth occurrence): the `discipline` job's
"interval-square powi(2) allowlist" step in `.github/workflows/ci.yml` greps
`crates/*/src` for `x * x` self-products outside the ratified allowlist
(scalar impls, D9-pinned mat.rs, f64-only svd/lsq/jet/march/system) — convert
to `powi(2)` or ratify the file into that allowlist.
