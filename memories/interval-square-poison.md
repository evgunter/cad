---
name: interval-square-poison
description: Interval squares of possibly-zero-straddling quantities MUST use powi(2), never x*x — CI-enforced; this file is the named home the kernel's comments cite.
metadata:
  type: project
---

**The rule.** In any interval lane, square a quantity that can straddle
zero via `Real::powi(2)` (inari `pown`) — never `x * x` or `v.dot(v)`.
Squares of definitely-nonzero singletons (stored radii, bulges) may stay
as `*`.

**Why.** `x * x` treats the factors as independent, so `[-a, b]` squares
to a lower bound of `-ab` instead of `0`; a downstream `sqrt` then clamps
its domain, the decoration degrades below `Def`, and the predicate
refuses geometry it should accept. `powi(2)` knows both factors are the
same variable and returns the tight nonnegative enclosure. Four live
bugs arrived this way.

**Enforced** by the `discipline` job's "interval-square powi(2)
allowlist" step in `.github/workflows/ci.yml` — that step carries the
allowlist and its per-file rationale. Convert, or ratify the file into
the allowlist; do not silence it any other way.

**Reviewing new geometry**: grep predicate-path diffs for `* self`,
`x * x`, and `.dot(` on possibly-zero vectors — the CI grep only sees
`crates/*/src`.
