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

## Measured on `sectarms/two-smalls` (2026-11-17): no such row found

A wall cubic in `u`, quadratic in `v`, `z = L·(c·(t³/3 + δ·t) + s²)`
(`t = x − ½`, `s = y − ½`, the chart `[0, 1]²` spanning `L` metres)
against the plane `z = 0`: `∇φ = (c·(t² + δ), 2s)`, nowhere zero for
`δ > 0`, smallest at the chart's middle. At ε 1e-9:

- `L` = 1 m, `c` 1 and 10: `δ` ≥ 1e-7 (≥ 1e-9 at `c` 10) reads every
  cell regular, `LoopFree([])`; any smaller `δ`, 0 included, passes
  256 survivors and reads `Budget`. The floor is about 0.7·ε metres,
  and the small-gradient strip runs `√(ε/(c·L))` along `t`, some 1e5
  floor widths.
- Shrinking the wall shrinks the count only as `√(L/(c·ε))`: at
  `L` = 100ε and 1000ε, `c` 10 and 100, the survivors number 1288 to
  over 4000, never under the cap.
- With the cap lifted locally, those that form one cluster read
  `Floor` **with and without** the mutant "containment not required"
  (`decide_split`'s `inside && first.is_none()` read as
  `first.is_none()`, `crate::ssi::node`): the first Krawczyk round
  returns `K ∩ X = X`, no narrower, so the final box is the cluster
  box, and its Hessian determinant straddles zero
  (`±1.5e-11` at `L` = 100ε, `c` 10), so `decide` refuses before
  containment is asked about.

That last reading is not particular to the fixture. Where `∇φ` is
small but nowhere zero, the point `p` at which `|∇φ|` is least has
`H(p)·∇φ(p) = 0` (its derivative there), so `H(p)` is singular; at a
knot, some convex combination of the two sides' Hessians is. The
small-gradient cluster holds `p`, so the cluster box's Hessian
enclosure holds a singular matrix, `I − Y·∇²φ(X)` does not contract,
and the final box keeps `p` and a determinant that straddles zero. A
box that keeps contracting away from `p` with no zero in it empties,
and `range` refuses it. The one escape is a `p` on the piece's edge,
where the final box hugs that edge and the sweep's strictly-inside
test declines it. So on the sweep's path the mutant looks equivalent
— the determinant test already does the work containment does here —
and a row that reds it may not exist; what is open is whether to close
this row on that argument or keep looking (a cap-free reading of
`crate::ssi::node::decide` on a hand-built box is the other place a
row could sit).
