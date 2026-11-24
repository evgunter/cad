---
id: the-junction-end-window-refuses-in-a-belt-around-a-neutral-point
kind: issue
title: An edge ending within about 2e-7 to 2e-4 rad of a junction's neutral point refuses JunctionEndUnread (Window): the order's term decides but no window holds its spread against the quartic tail
status: dispatched
opened: 2026-11-23
priority: P1
cost: M
branch: heldrecords/belt
---


Found in review of `heldrecords/elbow-ends` (PR 7463) and measured on its
fix pass, 2026-11-23.

## Measured

**Union level** (`editor-core`
`a_pipe_unions_with_its_elbow::an_edge_ending_near_a_neutral_point_stops_where_its_distance_says`):
the pipe (ring 1 / 0.6, length 2) and its elbow bent away off the End cap,
revolved about the line through `3·(−sin δ, cos δ)` heading `δ`, so the
bend's neutral point on each rim sits `δ` round the rim from the pipe's
seam ruling and the rim's vertex. At `ε = 1e-9`, `K = 10` (the
figures move with the band: the order's in-band stretch scales with
`ε`, and the belt runs from about `200ε` to about `7√ε`, the spread
`√(6ε/δ)` meeting the window `√(3δ)`):

| δ (rad) | the union stops at |
|---|---|
| 0 | the finish (the end's root of order 4, read past) |
| 2e-10 … 5e-9 | `CurvedPierceEscalated { JunctionEndOrder }` (the quadratic term in band) |
| 1e-8 … 1e-7 | `Escalated { Coincidence(Contact, Moot) }` at the glue door, not this door's |
| 2.2e-7 … 2.2e-4 | **`JunctionEndUnread { gap: Window }`**, the belt |
| ≥ 4.6e-4 | the finish |

**Door level** (`geom_brep::curve_face::end_root`, the ruling at
`θ = π/2 − δ` against the bend, span 2): `Order` at `δ = 1e-8`, `Spread`
from `1.8e-8` to `1.5e-4`, a window from `1.8e-4` on.

## The cause

Off the neutral point the bend's relative curvature across the rim is
`κ ≈ δ/3`, so `c₂` decides nonzero once `δ` clears the band, but the
quartic term `≈ s⁴/36` dominates `c₂·s²` past `s ≈ √(3δ)`: the window
halves to about that. The spread the dropped terms can move is
`√(2ε/κ) ≈ √(6ε/δ)`, so it fits the window only where
`√(6ε/δ) < √(3δ)`, `δ ≳ √(2ε)`, about `4.5e-5` at `ε = 1e-9`; with the
window's factor-two halvings and the tail's half-term margin the
measured edge is `1.8e-4`. Between the band and that edge no window holds.

The cause is the door's shape, not a constant: it certifies one term
against the rest (`|Q| ≥ |c_m|/2`). A window that holds the order's term
and the next ones together (the quotient's lower bound over the window
from `c₂` and `c₄` jointly, as a polynomial's positivity rather than one
term's dominance) would close the belt. That is a redesign of the
window's certificate, not a small fix, so it is not done in PR 7463.

## What the user sees, and the recourse

A normal elbow whose bend axis is turned by `1e-5` rad, so that a
neutral point sits that close to where an edge ends, refuses. The
refusal's sentence (`JunctionEndGap::Window`) says what is true: turning
the parts so the edge ends farther from where the two faces curve alike
across the rim (here, `δ ≳ 5e-4`, or exactly onto it) lets it through.

## Done when

The pose above builds past the crossing layer across the belt (it stops
at the finish, PR 3's subject, as `δ = 0` does), with a row at
`δ = 1e-5`.
