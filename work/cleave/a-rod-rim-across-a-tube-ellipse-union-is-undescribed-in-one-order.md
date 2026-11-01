---
id: a-rod-rim-across-a-tube-ellipse-union-is-undescribed-in-one-order
kind: issue
title: A rod's rim across a tube's ellipse rim, one carrier: t ∪ r refuses EdgeUndescribed NoConventionalLane where r ∪ t builds
status: closed
opened: 2026-10-12
priority: P2
cost: M
branch: cleave/rod-rim-union-describe
closed: 2026-10-14
---



Found by PR 4782's fix pass (`cleave/one-carrier-reads-ellipses`),
which pins it as
`sweep/tests/a_ruling_lying_on_a_wall.rs::a_rim_across_a_walls_ellipse_builds_every_op_but_one_union_order`.

## What

The scene:
- `t` is the slanted tube: radius 1, `z` from `0.5 + 0.2·x` to 2.
- `r` is a coaxial rod of the same radius, `z ∈ [0.5, 2.5]`, turned
  0.4 and 1.3 rad about `z`.

Their walls are one carrier, and the rod's lower rim crosses the
tube's ellipse rim at `x = 0`. Valid input, with closed forms: the
overlap is `1.5π − 2/15`, `t ∖ r = 2/15` and `r ∖ t = 0.5π + 2/15`.

Measured at ε 1e-9, 1e-6 and 1e-12, both turns:
- `r ∪ t`, both `∩` orders and both `∖` orders build at their closed
  forms, at tiers 3 and 3′.
- `t ∪ r` refuses `EdgeUndescribed { failure: NoConventionalLane,
  cause: None, fitted: false }`.

On main these ops refused earlier, at the interior question, which
read no ellipse on the boundary.

## Why

Not measured. The refusal is `boolean::ops`' `describe_edges`: an
edge of the union in this member order has no conventional
description lane. The other order describes the same result. The
first step is to find which edge it is (a split piece of the ellipse
rim or of the circle rim) and which lane each order reads.

## Built (branch `cleave/rod-rim-union-describe`)

Measured on main (`1b2ae6356d`): the edge `t ∪ r` refuses is the
ellipse rim's arc over `x > 0`, an `Intersection {tube wall, tube
bottom}` left between the tube's wall and the rod's wall piece under
it. In this scene a union keeps the first member's wall whole, so
`t ∪ r` leaves the ellipse's arc (and the tube's top circle) across a
kept wall, and `r ∪ t` the rod's lower circle instead. Both reach
`describe_edges`' smooth arm as stale curved edges. That arm described
a stale circle by an arc pushforward and refused every other curved
kind `NoConventionalLane`, so only the order that left an ellipse there
refused.

The stale arm now restates any carrier kind as a derived image in the
plus face's chart, on the unchanged carrier, carrying the edge's
authority record as it was (`boolean/ops.rs` `describe_edges`). The
circle and line lanes it replaces each stamped a made-up `Declared`
pushforward on an edge nothing declared. `EdgeDescribeFailure`'s
`NoConventionalLane`, `Arc` and `Line` collapse to `Chart`.

The pin is renamed
`a_rim_across_a_walls_ellipse_builds_every_op_in_both_orders`: six
turns of the rod, all six ops build at tiers 3 and 3′ at their closed
forms, at ε 1e-9, 1e-6 and 1e-12 (run locally).

Fix pass (review of PR 4856):
- **One home for the restatement.**
  `geom_brep::EdgeCurve::restated_in_chart` now builds the chart image
  and carries the authority. The stale arm uses it, and so do both of
  `splitting::finish`'s conventional restatements.
- **The pin asserts the description.** It checks that every edge lying
  across a wall rests in one of its faces' charts, and that each curved
  one is derived. The turns `π/2`, `π` and `3π/2` are added.
- **New pin:**
  `a_half_rods_rim_across_a_rods_wall_moves_with_its_flat_face`.
  - The half-rod-first union reaches the stale-line path.
  - In the rod-first union, the flat face's offset of ±0.05 builds at
    its closed form.
  - Under the old made-up `Declared` lanes, that offset refuses
    `CarrierLaneUnsupported`.
  - Run locally at ε 1e-9, 1e-6 and 1e-12.
