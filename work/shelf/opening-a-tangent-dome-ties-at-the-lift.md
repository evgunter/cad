---
id: opening-a-tangent-dome-ties-at-the-lift
kind: issue
title: shell_open through a tangent dome refuses TogetherAxialCorner at the lift: two equidistant roots, only one on the designated face
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [shell-of-a-tangent-dome-refuses-at-the-axial-corner]
---


`topo::shell_open` of `common::shell_operands::domed_vessel(0.5, 0.6)`
designating its two hemisphere half-faces, at `t = 0.05`,
`Tol::witness()`, refuses at the lift:

    Lift { error: TogetherAxialCorner { surfaces: 0,
      what: "two solutions stand the same distance from the corner being moved, ..." } }

The lift moves the cavity's sphere (`0.45`) back to `0.5` with the
cavity's cylinder (`0.45`) held, and the two now CROSS at stations
`0.6 ± √(0.5² − 0.45²) = 0.6 ± 0.218`, symmetric about the old corner
on the equator. `offset_axial::nearest` picks the root nearest the old
corner, and here there is no nearest, so it refuses. That refusal is
right for `nearest`: two distinct roots at one distance are two answers
to the question it asks.

The question is the wrong one for this corner, though. Only the upper
root lies on the designated face's own window (the hemisphere is
`y ≥ 0.6`); the lower one is on the sphere's other half, where the
face does not reach. A tie-break by the moving chart's own window, or
by the side of the corner the counterpart face lies on, would answer
it. Opening the floor of the same body builds today.

Pinned in `sweep`'s
`shell_curved_mouth::opening_a_tangent_dome_refuses_two_equidistant_roots`,
which is also the row keeping the genuine tie refusal reachable.
