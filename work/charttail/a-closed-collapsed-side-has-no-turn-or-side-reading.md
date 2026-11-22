---
id: a-closed-collapsed-side-has-no-turn-or-side-reading
kind: issue
title: a spline net's collapsed side along a channel the net closes (a NURBS cone's apex) has no turn or side reading, so a joint its marks do not decide refuses ClosedSide
status: open
opened: 2026-11-22
priority: P2
cost: H
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint]
---


Filed by CHARTTAIL's `charttail/collapsed-column` lane, from its
review's finding (PR 7325).

A spline net's collapsed side is a chart singularity (DESIGN
`[extensibility]`). Where the side runs along a channel the net closes,
as a NURBS cone's apex row does along its closed `u`, a joint at the
point is a turn about it, as at an analytic cone's apex. On a spline
chart no reading states that turn, or the face's side of the point.

`crates/topo/src/pcurves/decision.rs` `decide_joint` keeps the chart-gap
decision wherever the marks decide it. Across the seam, the
`spline_cone()` apex joint (1, 0) → (0, 0) reads a period over, as on
main. A joint whose gap no mark decides, such as half a turn
(0.75, 0) → (0.25, 0), refuses `SingularMiss::ClosedSide`. On main it
read `Discontinuity`, a decided refusal of a face corner that may be
legal. Pinned by
`pcurves/collapsed_side_rows.rs`
`a_collapsed_side_along_a_closed_channel_keeps_what_its_gap_decides`.

## Fix

Give a spline net's closed collapsed side the turn an analytic pole has
(`singular_turn`): the angle about the point between the two edges,
read on the face's side. That needs a direction about the point on a
spline chart, which the analytic charts read off their axis.
