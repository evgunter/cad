---
id: a-rod-rim-across-a-tube-ellipse-union-is-undescribed-in-one-order
kind: issue
title: A rod's rim across a tube's ellipse rim, one carrier: t ∪ r refuses EdgeUndescribed NoConventionalLane where r ∪ t builds
status: review
opened: 2026-10-12
priority: P2
cost: M
branch: cleave/rod-rim-union-describe
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
