---
id: joining-a-spline-carrier-is-unbuilt
kind: issue
title: A curved join on a spline carrier refuses JoinCarrierUnsupported: the kept edge cannot be run on along a carrier with no period or parameter inverse
status: open
opened: 2026-10-07
priority: P1
cost: H
---

## The finding

The curved join restates the kept edge over both edges' span on its own
carrier (`topo::boolean::edge_join::joined_spec`): a closed join over
the carrier's period, which only `Circle`, `Ellipse` and `Spiric`
carriers have, and an open join through the killed edge's far end,
recovered by `Curve3::param_near`, which has no inverse on a `Nurbs`
carrier. Where either is missing the join refuses
`BooleanError::JoinCarrierUnsupported` (pncad-py tag
`join_carrier_unsupported`) and the boolean fails, where before the
curved join it left the vertex standing. The ruling allows the refusal
(a vertex the build cannot join refuses typed); no `ci` row reaches
it, since no operand carries a spline edge between two faces past the
boolean's operand gate (`BooleanError::CurvedEdgeUnsupported`).

## What it needs

A spline carrier's run-on: its knot span extended (or the two edges'
spline images concatenated) and a parameter foot on it, so the joined
edge is one spline with a certified interval.
