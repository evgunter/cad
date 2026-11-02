---
id: no-row-guards-the-krawczyk-existence-test-on-a-spline-critical-point
kind: issue
title: No row guards the Krawczyk existence test that decides a spline × implicit critical point
status: open
opened: 2026-11-02
priority: P3
cost: M
refs: [6347]
---


Found by PR 6347's full review (stage 2 of the NURBS × implicit arm) and
measured in its fix pass.

`crates/geom-brep/src/pair_section/spline_implicit/sweep/critical.rs`
decides a critical point of `φ = f∘S` only where the Krawczyk image `K`
lies strictly inside the cluster box `X`. For a touch, uniqueness
already follows from the Hessian's determinant being one-signed over a
convex box. What containment adds is **existence**: without it, a box
over a regular stretch where `∇φ` is small but nowhere zero could read
as a critical point that is not there, and the pose would read a Touch
over a real arc.

The mutant "containment not required" survives every row. The fix
pass measured `z = a(x − ½) + (y − ½)²` against the plane `z = 0`:
- at `a` = 0.3, 1e-3 and 1e-6 the sweep is `LoopFree([])`;
- at `a` = ε/2 and 2ε more than 256 survivors run along the strip, and
  the sweep reads `Budget` before any cluster is decided.

So the 256-survivor cap (`exhaust::REGULAR_SURVIVORS`) masks it. A
biquadratic wall's quadratic `φ` always has a critical point, so the
row needs a wall of higher degree. Its `φ` must have a small-gradient
region compact enough to be one cluster, with no zero of `∇φ` in it.

Done when such a row exists and the containment mutant goes red on it.
