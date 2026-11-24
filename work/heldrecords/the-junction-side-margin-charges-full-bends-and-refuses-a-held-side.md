---
id: the-junction-side-margin-charges-full-bends-and-refuses-a-held-side
kind: issue
title: The junction's side margin charges the edge's and the rim's full bends, so it refuses an end whose side holds (a sphere's small circle near a cone–sphere rim's equator)
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by the second review of PR 7463 (`heldrecords/elbow-ends`) and
measured on its fix pass, 2026-11-23.

## Measured

`reduce::junction_end_window` on a unit sphere's small circle leaving
the cone–sphere rim at latitude `z = h`, at angle `β` from the rim's
tangent into the sphere's side, against the cone tangent along that
latitude, span a quarter radian (`junction_end_rows`'s `ice_cream_at`),
at `ε = 1e-9`, `K = 10`:

| h | β where the end-root window holds and the side refuses |
|---|---|
| 0.02 | about 2.9e-3 to 7.9e-3 |
| 0.05 | about 3.8e-3 to 7.9e-3 |
| 0.1, 0.2 | some β in the same decade (the review's scan) |
| 0.5 | none: the window refuses first |

Pinned: `a_small_circle_near_the_equator_refuses_on_its_side_where_the_window_holds`
(h = 0.02, β = 5e-3).

## The cause

The side margin is `g(σ) = d·σ·(1 − (κ_e·σ)²/6) − (κ_e + 1/R)·σ²/2`,
charging the edge's and the rim's full space curvatures (here
`κ_e + 1/R ≈ 2`). What sets the edge's offset across the rim within the
sphere is the difference of their geodesic curvatures on the face, about
`h/ρ ≈ 0.02` here, so the margin over-charges by two orders. The
refusal is `JunctionEndUnread { Window }`: loud and conservative, never
a wrong body.

## Done when

The margin charges a bound on the geodesic-curvature difference on the
edge's own face (with its own soundness argument for the surface's bend
between the two), and the pose above passes the side check, its row
flipped to a held side.
