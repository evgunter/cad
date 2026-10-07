---
id: shell-of-a-tangent-dome-refuses-at-the-axial-corner
kind: issue
title: shell of a hemisphere tangent to its cylinder wall refuses TogetherAxialCorner: the moved corner has two equidistant solutions
status: open
opened: 2026-10-06
priority: P2
cost: M
refs: [shell-open-refuses-a-curved-designated-face]
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
