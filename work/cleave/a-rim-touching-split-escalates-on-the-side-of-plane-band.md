---
id: a-rim-touching-split-escalates-on-the-side-of-plane-band
kind: issue
title: a tube split by a plane whose section touches a rim at azimuth 0.3 escalates on the side-of-plane band
status: closed
opened: 2026-10-06
priority: P3
cost: E
closed: 2026-10-07
pr: 4223
branch: cleave/rim-touch
---


## What

The tube revolved a full turn about `y` from `(0.5, 0)–(1, 0)–(1, 1)–(0.5, 1)`,
cut by the plane leaning `t = 0.2` whose outer ellipse touches a cap rim
at one point. This is the construction of
`split_across_a_revolve_seam::a_section_touching_a_rim_splits_at_the_closed_form`
(the plane through `(cos a, rim, sin a)` with normal
`ŷ cos t ∓ (cos a, 0, sin a) sin t`). At azimuth `a = 0.3` the split
escalates on the side-of-plane band, margin `5.9e-9`, on main and on
PR 4120's head alike. At the azimuths that row pins (0, π/2, π) it
splits at the closed form.

Unmeasured, a theory only: the touch point is a computed point on the
rim circle, at a rounding-scale distance from the plane, so whatever
the side reading reads there lands in the band. A rim touch is a
tangency of the section conic with the rim circle, and nothing declares
it.

## Where to look

The review reports a side-of-plane band escalation; the raising site
is not traced. Measure first: read the payload and the vertex
it names. The question is whether a rim tangency at an off-vertex
azimuth can be decided structurally, or whether it is a declared
contact the split has no channel for.

## Found by

The review of PR 4120.

## Built (branch `cleave/rim-touch`)

**Measured first, then traced.** The escalation does not reproduce on
`origin/main` (`dfcd7f35`): the tube's rim-touch split builds at 72
azimuths (64 evenly spaced, with every odd one shifted by +0.01, plus the
measured witnesses), × both rims × both normals. Each half passes tiers
1, 3 and 3′, at ε = 1e-9, 1e-6 and 1e-12. Every half that measures holds
its closed form within 1e-8 plus its certified pad. The halves that do
not measure are listed under the sweep below.

On PR 4120's head (`9fcfebd8`) the payload is
`Reduce(SliverSector { vertex 6v3, face 4v1 (top) / 5v1 (bottom), margin
±5.8876e-9, predicate "split_conic_departure" })`. That is not a
side-of-plane decision but the curved-edge departure decision in
`splitting::neighborhood::classify_neighborhood`, read at the rim's
graze vertex. It fires at six of the 65 azimuths: 0.3, 6π/32,
29π/32 + 0.01, 42π/32, 47π/32 + 0.01 and 58π/32, at all four poses
each.

**Cause.** By construction, the rim circle (radius r, centre on the
axis) reaches the plane at exactly one point, `P = (cos a, rim, sin a)`.
It is the rim's extremum toward the plane: `D = (centre − P)·n̂ = sin t`
and `R = r·|n̂ − (n̂·ŷ)ŷ| = sin t`, so `R − |D| = 0` exactly, and
`split_conic_belly_graze` reads `Zero` (a graze). `P` is a B-rep vertex
only at a = 0, the seam vertex. Elsewhere it lies between vertices,
and the split inserts it.

Before PR 4179, `splitting::classify::conic_plane_meet` placed the graze
root at `φ + acos(−D/R)`. Where the f64 ratio rounds off ±1 by an ulp,
that acos is about 1.5e-8 rad, and the inserted vertex lands that far
along the rim from `P`. The rim's tangent there leans out of the plane by
the same angle. Metered at the edge's extent, that is the 5.9e-9
departure margin, and it falls in the band. So case (a): the kernel read
a computed point √-scale off the tangency, while the tangency itself
(`R − |D|` Zero) was decided.

PR 4179 (`cleave/inband-graze`, merged 2026-10-06) moved the graze root to
the sinusoid's extremum (`φ` or `φ + π`, by `split_conic_graze_side`).
That is the decision that owns it, and it is the fix. The row was filed
against 4120's head before 4179 landed.

**A/B on current main.** Restoring only the acos placement in the
`split_conic_graze_side` arm reproduces the original failure exactly:
the same six azimuths, the same `split_conic_departure` margin
5.887565e-9, the same vertex and face keys. At 1e-12 those azimuths
refuse `Join(SectionCrossings { NotAlternating })` instead, and at 1e-6
the offset falls inside ε and they build. With main's extremum root,
all of them build.

**Pinned.** `split_across_a_revolve_seam::a_section_touching_a_rim_splits_at_the_closed_form`
now runs the six witness azimuths (`RESIDUE_OFF_P`) beside 0, π/2 and π,
at both rims and both normals. Each half must hold exactly one vertex
within ε of `P`, the cap side must hold `A·tan t`, and the halves must sum
to `3π/4`. With the acos root restored it fails at 1e-9 and at 1e-12
(`NotAlternating` at azimuth 0.3). The test costs 4.4 s at the default
ε and 31 s at 1e-12, so it moves to the `ci` slow set. The mechanism's
unit pins (`classify::tests::belly_graze_trio`) stay in the fast set.

**Sweep: off-vertex rim tangencies in other carriers.** Each fixture
was run at 96 azimuths × both normals × the three ε rows:

- A frustum-walled tube (outer cone, r 1 → 0.8), touching either outer
  rim. At 1e-9 and 1e-12, everything builds clean.
  - At 1e-6, `SliverSector` on `split_bisector_side` appears at
    azimuths 0.078 and 6.242, within 0.08 rad of the seam vertex, with
    margins ±7.4e-6 and ±1.1e-6.
  - The margin scales with the cube of the touch's distance from the
    seam vertex. That is a real geometric size inside Kε, not rounding,
    so the escalation is honest.
  - The other 1e-6 failures are quadrature.
- An extruded rod (r 1, two arcs), touching either rim. The split
  builds at every pose and ε, and the cap side holds `π·tan t`. The only
  failures are `mass_properties` quadrature, which is evidence for QUAD's
  row.
- The tube's bore rim (r 0.5). This is a new finding, filed as
  `a-plane-touching-a-bore-rim-splits-into-a-pinched-side-under-one-normal-and-refuses-under-the-other`.
  It is a different shape: the touch pinches the cap-side half at `P`.
- A sphere zone (and the torus from the inward bulge) refuses
  `Reduce(CurvedBooleanUnsupported { Sphere | Torus })` at the split's
  carrier gate. That is by design and not this unit.

Quadrature-only failures on these fixtures (`props_quad_converged` in
band, `QuadratureBudget` at 13 rounds) went onto
`work/quad/quadrature-convergence-test-escalates-instead-of-refining.md`.

## Closed (PR 4223, 2026-10-07)

Fixed by PR 4179 before it was taken. The escalation was `split_conic_departure` at a graze vertex
that `acos` had placed 1e-8 rad off the rim's touch point. The graze root is now the extremum,
which the decided side chooses. `a_section_touching_a_rim_splits_at_the_closed_form` pins the
six witness azimuths at every ε, with one vertex on the touch point and the closed form on both
sides. The bore-rim pinch is filed separately.
