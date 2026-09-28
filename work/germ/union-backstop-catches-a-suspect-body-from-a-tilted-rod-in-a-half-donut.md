---
id: union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut
kind: issue
title: A tilted rod entering a half donut's cap and poking an oval out of the inner equator reaches the volume backstop on a tier-valid planar body: the pipeline's upstream result is suspect
status: open
opened: 2026-09-28
priority: P1
cost: M
---


## What

Found by PR 3336's review (2026-09-28). A tilted rod enters the half donut's
planar cap and pokes an oval out of the inner equator; no events lie on the
oval. `union` refuses at `ClassificationInvariant "volume backstop: mass
properties refused on a tier-valid planar body"`, with the interior-loop
guard on, off or mutated. The backstop catches it, so no wrong answer
reaches a caller. But the body the pipeline built before the backstop is
suspect, and the backstop is the last line. Measure what the pipeline built,
and whether it is the interior-loop class meeting the backstop first.

## Home

GERM: the torus boolean lane.

## Measured (GERM measurement lane, 2026-09-28, on `378f66744`)

**Answer: this is the interior-loop class. The backstop meets it first
only because the closed-form mass-properties lane cannot integrate the
result; its bounds would have passed the wrong body.**

**Fixture:**

- `half_donut` as in `germ_interior_oval.rs`.
- The rod is `germ_pair::cyl(0.15, 0.85)`, spun `π/2` about its own axis
  (the spin moves its seam), then rotated `β = 0.5` about `y`. Its axis
  runs from `(1.8, 0, 0)` along `(−sin β, 0, −cos β)`, over
  `t ∈ [−0.3, 1.4]`.
- It enters the `x > 0` cap and bulges out of the inner equator
  (closest approach: axis `ρ = 1.5796`, wall `ρ ≈ 1.43`). It ends
  inside the tube.
- With spin `0`, `1.0` or `2.5` the same rod refuses earlier, at
  `CurvedSectorSideUnsupported`. The backstop is reached only with
  spin `π/2`, across seven `(β, x0, r, L)` variants.

**Instrumented** (a throwaway patch, reverted): the Seamed build's
backstop and guard were logged instead of raised.

- **The backstop's cause.** `mass_properties_closed_form(result)`
  returned `Err(Face { face: 6v3, NotIsoRectangle: "cylinder boundary
  carries an ellipse arc (curved cut)" })`. The rod wall is cut
  obliquely by the cap plane, and the closed-form lane has no ellipse
  arc. The operands measure fine (H `4.934802`, rod `0.120166`). So the
  "ClassificationInvariant" is a lane limit. It is not a detected defect.
- **The guard would refuse too:** `CurvedPairUnsupported { op: Union,
  Torus 3v1 × Plane 2v1 }`, the torus half on reach.
- **The body built before the backstop.** It is `Seamed` with faces
  `[Plane, Plane, Torus, Torus, Cylinder, Plane, Cylinder]`, and
  `validate_geometric` is `Ok`.
  - Its quadrature volume is `4.956008 = vol(H) + 0.021206`, where
    `0.021206 = π·0.15²·0.3` is exactly the rod's part above the cap.
  - It is H plus the rod's stub above the cap, and nothing else. The
    lens outside the inner equator is dropped: Monte Carlo gives
    `0.00782 ± 0.00002`, so the true ∪ is `≈ 4.96383`.
  - `point_in_solid`: `c + 0.13·n̂` and `c + 0.11·n̂` (with `c` the
    closest-approach axis point and `n̂` toward the hole) are rod
    `In`, H `Out`, and result `Out`.
  - `(2, 0, −0.5)` (H only) and the stub point `t = −0.2` read `In`,
    which is correct.
- **If the closed-form lane learned ellipse arcs,** `4.956 ≥ max(vA, vB)`
  and `≤ vA + vB` would both pass. The interior-loop guard, which runs
  after, would be the only barrier.
