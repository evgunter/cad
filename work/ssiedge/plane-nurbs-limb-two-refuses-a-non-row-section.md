---
id: plane-nurbs-limb-two-refuses-a-non-row-section
kind: issue
title: the plane x NURBS certificate's limb 2 refuses every section that is not a wall's row, by micrometres on a polynomial wall and millimetres on a rational one, while the march's own certificate passes
status: open
opened: 2026-10-08
priority: P1
cost: H
refs: [plane-nurbs-certificate-bound-does-not-refine-with-eps, shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam]
---


Filed by SHELL's `shell/oblique-corner-derives` lane (unit 9), which
measured the plane × NURBS lane before deriving a moved cap's rims by
section.

## Measured

`geom_brep::plane_nurbs_limbs::<f64>` at the default ε, release build,
on the vase: circle sections of radius 1, 1.3 and 1 at z = 0, 1, 2,
lofted at degree 2 (`crates/sweep/tests/encl_curved_loft_shell.rs`'s
`vase`). Its two walls are rational in `u` (weights 1, √2/2, 1, √2/2,
1) and polynomial in `v`. The plane is the top cap, moved 0.05 inward
(z = 1.95) or at rest (z = 2); `extent` 2.

| carrier | limb 2 |
|---|---|
| `plane_nurbs_ssi`'s own branch (one per wall, about 650 ms each) | refused, **4.787e-4 m** |
| the wall's exact row at v* = 0.975, its weights made equal along `v` (the surface moves 5.4e-16 m) | refused, **3.325e-3 m** |
| at rest: the wall's top boundary row (`boundary_iso_v(wall, true)`) | refused, **3.231e-3 m** |

The march's own certificate passes on the first row: an `SsiBranch` is
not built without one. The twisted loft (`common::approx::twisted_loft(0.3)`,
four bilinear walls, polynomial) passes every row: on-locus 2.9e-15,
hull 9.0e-14, min sin θ 0.953 on the march branch; hull 8.8e-14 on the
exact row; hull 2.3e-14 at rest.

The measurement was a scratch probe (one row per carrier above, built
from `nurbs_walls` and the vase); the shipped row that reaches it is
`encl_curved_loft_shell::shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_at_its_rims_certificate`,
which refuses `RechartFalsifies { PlaneNurbs(Limb { HullSup, 4.787e-4 }) }`
on a vase cap.

## A polynomial wall too, wherever the section is not a row

The dual review of PR 4351 measured the same limb on POLYNOMIAL walls.
A straight square prism lofted to a top section tilted 0.2 rad about a
line through its centre (four bilinear walls; `encl_curved_loft_shell`'s
`tilted_top_prism`) has rows that are not level in the top cap's
normal, so a moved top cap's section of a wall is no row of it and the
section lane marches it. The marched branch lies on both surfaces to
~5e-11 m, and limb 2 refuses:

| cap moved | limb 2 |
|---|---|
| −0.05 | **3.743e-6 m** |
| −0.2 | **1.386e-5 m** |

`encl_curved_loft_shell::a_tilted_caps_marched_rim_refuses_at_its_certificate`
pins it. Every section limb 2 has passed in this unit's measurements
is a row of its wall: the image is exact there and term (a) is zero.
So the defect reads as limb 2's bound on any image that is not exact,
growing with the wall's curvature across the chart, and the rational
rows above as the same bound failing even on an exact image. Until it
certifies a marched section, SHELL's section lane builds only row
sections.

## Why this is not `plane-nurbs-certificate-bound-does-not-refine-with-eps`

That row's bound is 1.7e-12 m of ring widening from interval Boehm
insertion, and it grows with the schedule. This one is eight orders
larger, and the exact row fails it: the carrier IS the wall's row, so
the chart image is `(t, v*)` at every foot, the degree-1 image is exact
and term (a) is zero. What is left is the composite's hull bound on a
RATIONAL wall. The likely cause, unconfirmed: `S(P(t)) − C(t)` bounded
by control-net differences of two rational curves that do not share a
weight function, a hull that does not converge to the difference. A
taker measures that first.

## What it blocks

A cap's rim on a rational wall cannot be certified as the cap and
wall's `Intersection`, at rest or after a move. SHELL's per-chart door
derives a moved cap's rim by section, so the vase's shell refuses
here, at its cap. The offset fit mints unit weights, so a plane ×
fitted-wall section is polynomial: the rational case is reached only
where a cap is sectioned against a held rational base wall.

## Measured, not a loose hull (encl/hull-bound-refine, 2026-10-10)

Limb 2 now subdivides its composite before refusing, and reads the composite's certified value at each break (a Bernstein row interpolates its end coefficients). Both shipped rows refuse on that value, not on the bound:

| row | bound (before) | certified value at a break |
|---|---|---|
| vase cap rim (`shelling_the_vase_refuses_at_its_rims_certificate`) | 4.787e-4 m | ≥ 4.776e-4 m |
| tilted prism, cap −0.05 (`a_tilted_caps_marched_rim_refuses_at_its_certificate`) | 3.743e-6 m | ≥ 3.016e-6 m |

So `|S(P(t)) − C(t)|` is that large at a point of the carrier: the marched carrier and its fitted pcurve disagree by the stated amount, at a break of the composite. That quantity bounds the carrier's miss from the wall from above (C2 limb 1); it is not the miss, and the marched branch lies on both surfaces to ~5e-11 on the tilted prism. What is left is the stored pair itself (the pcurve fit, or the carrier the pcurve is evaluated against). Both rows refused as `SsiLimb::HullValueChart` (`CertCheck::PlaneNurbsHullValue`, a bound's ending) until PR 5160.

The two exact-pair probes above, re-run on the same branch (debug build, default ε, band zero 1e-9; each wall alike) under its work budget (`SSI_HULL_WORK`, spans recomputed over the rounds):

| carrier | limb 2 before | now |
|---|---|---|
| the wall's exact row at v* = 0.975, weights equal along `v` | refused, 3.325e-3 m | **certifies**: limb 2 clears at 7.94e-10 m, and limbs 1 and 3 pass |
| at rest: the wall's top boundary row | refused, 3.231e-3 m | **certifies**: limb 2 clears at 7.72e-10 m, and limbs 1 and 3 pass |

So the 3.3e-3 m on these pairs was the loose hull, which the "likely cause" above names. Subdividing brings it down fourfold a round, on two cuts a round around a peak two spans wide: 11 rounds, 22 cuts and 484 spans of work each. What this row still holds is the two shipped rows above, which refuse on a value at a break, not on the hull.

## Answered: the pcurve misses, not the carrier (PCERT, PR 5160, 2026-10-15)

PR 5160 adds a tell to limb 2. At the break where the composite reads
past the band, it reads the carrier's own foot, as limb 1 does. When
that decides Zero, the refusal names `SsiLimb::ImageMiss`: the
pcurve's own miss. Both shipped rows now refuse that way. The
carrier's own foot distance at the refusing break, at ε = 1e-9:

| row | carrier's foot at the break | value at the break |
|---|---|---|
| vase cap rim (`shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_at_its_rims_certificate`) | 1.3e-14 m | ~4.8e-4 m |
| tilted prism, cap −0.05 (`a_tilted_caps_marched_rim_refuses_at_its_certificate`) | 8.3e-17 m and 2.8e-17 m (two breaks) | ~3.0e-6 m |

So the marched carrier is on the wall where the refusal reads, and the
miss is the stored pcurve's, the fit against the marched carrier. The
row's remaining question narrows to the pcurve fit of a non-row
section on a rational wall.

## 2026-10-23 — the vase no longer reaches this row (ISO, `iso/vase-cap-moves-alone`)

Since PR 5629 (`skin_on` picks its lane per control row), the vase's
wall weights are constant along `v`. So the moved cap's section of
each wall is the wall's row at `v = 0.025`, exact structure, and the
rim mints on that row and certifies: the vase's cap now moves alone.
The shipped vase row is renamed
`shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_moves_alone_onto_a_wall_row`,
and it now stops at tier 3's volume check
(`a-planar-face-bounded-by-a-rational-spline-rim-has-no-vector-area`, closed by PR 5689).
The rational-wall measurements above are history. The row's live pin
is the polynomial tilted prism
(`a_tilted_caps_marched_rim_refuses_at_its_certificate`), a marched
non-row section.
