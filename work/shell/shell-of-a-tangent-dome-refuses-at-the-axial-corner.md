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

Tangency is its own arm of the meridian corner solve
(`offset_axial::tangent_foot`), at both line–circle solves
(`solve_corner`'s profile pairs and `cap_pair_corner`'s cap line). A
line–circle pair whose gap `r − |d|` decides Zero is tangent, and its
one meeting point is the foot of the circle's centre on the line. The
gap, a length, is what is decided, not the roots: a tangent pair's
roots stand `2√(2r·gap)` apart, so whether the conditioning meter or
`nearest`'s tie refused it was decided by rounding and scale. The
tangent bullet (`3/64` scale) refused at the meter while the `0.5` dome
refused at the tie. Both now shell at the tangent circle, tier 3, at
the closed form (`shell_curved_mouth::a_tangent_dome_shells_at_its_tangent_circle`,
`sf2b_axial::the_axial_door_names_its_own_boundary`).

A pair with a gap outside the band takes the transversal route
unchanged, and two distinct roots at one distance from the corner still
refuse: opening the dome lifts the cavity sphere back to `r`, which
crosses the cavity cylinder at `h ± 0.218`
(`shell_curved_mouth::opening_a_tangent_dome_refuses_two_equidistant_roots`).
That opened row is filed as `opening-a-tangent-dome-ties-at-the-lift`.
