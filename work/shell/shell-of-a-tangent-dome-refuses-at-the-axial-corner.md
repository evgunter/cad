---
id: shell-of-a-tangent-dome-refuses-at-the-axial-corner
kind: issue
title: shell of a hemisphere tangent to its cylinder wall refuses TogetherAxialCorner: the moved corner has two equidistant solutions
status: closed
opened: 2026-10-06
priority: P2
cost: M
refs: [shell-open-refuses-a-curved-designated-face]
pr: 4356
closed: 2026-10-08
branch: shell/apex-and-tangent-corner
---


`topo::shell` (sealed, and so every `shell_open`) of a cylinder
`r = 0.5`, `h = 0.6` under a hemisphere of the same radius, tangent to
the wall at the equator (`common::shell_operands::domed_vessel`),
refuses at `t = 0.05`, `Tol::witness()`:

    Face { error: TogetherAxialCorner { surfaces: 0,
      what: "two solutions stand the same distance from the corner being moved, so which one the offset keeps is not determined" } }

The cavity's sphere (`0.45`) and cylinder (`0.45`) are tangent again
at the equator, so the corner solve in the meridian half-plane has a
double root. The corner is not ambiguous: the two roots coincide, and
it is the tangent circle. Whether `offset_charts_together` should take
a coincident pair as one solution, or the tangency as its own arm, is
the decision.

Pinned in `sweep`'s `shell_curved_mouth::a_cone_tip_and_a_tangent_dome_refuse_in_the_sealed_arm`.
It blocks the tangent dome cap that `shell-open-refuses-a-curved-designated-face`
owed. A cap meeting its wall at an angle opens today (`a_spherical_cap_opens_to_a_seamed_band`).

## Closed

The meridian corner solve's line–circle pairs, at both of its solves
(`solve_corner`'s profile pairs and `cap_pair_corner`'s cap line),
resolve a near tangency in three steps (`offset_axial::branch`,
`offset_axial::tangent_foot`):

1. **The nearest root**, unchanged, wherever the pair is well
   conditioned and one root is nearer the old corner.
2. **On a tie, the old corner's side of the pair's foot** along the
   line, a length. Near tangency the two roots stand symmetric about the
   foot, and every point near the foot is nearly equidistant from them,
   so nearness cannot choose; the side can, because the unmoved pair
   crossed on the same side of its own foot. A cap short of tangent by
   `1e-14` to `1e-11` tied and refused before; it now takes its upper
   root.
3. **The foot**, only where no root is determined — the pair too
   ill-conditioned for the meter to resolve, or tied with the old corner
   on the foot itself — and only where the gap `r − |d|` decides Zero.
   That is the exact tangency: the dome (tied, side Zero) and the
   tangent bullet (`3/64` scale, unresolvable). A tangency the band
   cannot decide escalates only if no other pair solves.

The foot is a fallback, not a reading of the gap. Taking it whenever
the gap decided Zero regressed caps short of tangent by `3e-10` to
`9e-10`, which build on the nearest root: the foot sits `√(2r·gap)`
off it, where the moved surfaces are tangent and the edge's crossing
description does not certify.

Pinned in `shell_curved_mouth`: `a_tangent_dome_shells_at_its_tangent_circle`
(both scales, sealed and floor-opened) and
`a_dome_short_of_tangent_shells_at_its_upper_root` (gaps `1e-12`,
`5e-10`, `2e-8`), each tier 3 with its corner and volume against the
closed form; and in `sf2b_axial`'s
`a_torus_belly_and_a_tangent_bullet_hollow_through_the_axial_door`.

Two distinct roots at one distance from the corner, with the old corner
on their foot and the pair not tangent, still refuse: opening the dome
lifts the cavity sphere back to `r`, which crosses the cavity cylinder
at `h ± 0.218`
(`shell_curved_mouth::opening_a_tangent_dome_refuses_two_equidistant_roots`),
filed as `opening-a-tangent-dome-ties-at-the-lift`. Filed from the
review: `axial-corner-nearly-tangent-refusal-has-no-row` and
`axial-corner-solves-no-circle-circle-pair`.
