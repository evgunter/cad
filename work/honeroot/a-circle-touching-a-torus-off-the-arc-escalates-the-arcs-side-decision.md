---
id: a-circle-touching-a-torus-off-the-arc-escalates-the-arcs-side-decision
kind: issue
title: circle × torus: a touch off the arc escalates the subdivision's side decision (circle_torus_sub_side) for an arc it does not reach
status: open
opened: 2026-11-04
priority: P3
cost: M
refs: [root-slack-meters-roots-outside-the-arc]
---

Found by `splice/teapot-1e6-floor`'s sibling sweep (PR 6519: a span's
walk refusing on a meeting past its end), and re-measured by its review
at every ε row.

## What

`curve_face::circle_torus::circle_torus_roots` reads the circle against
the torus round the whole turn, and its subdivision decides each piece's
side (`circle_torus_sub_side`) wherever the piece lies on the turn. A
touch anywhere on the turn therefore escalates the answer for an arc
`[t0, t1]` that does not come near it. This is the shape of
`root-slack-meters-roots-outside-the-arc`, but through the side
decision rather than the root-slack meter, so it can stay red after that
row closes.

## Measured

The torus is about `z`, with major radius 1 and minor radius 0.25. The
circle has radius 0.5 about `(1.75, 0, 0)` in the plane `y = 0`, with
`u_ref = −x`: it touches the torus's outer equator at `θ = 0`,
`(1.25, 0, 0)`, and stands clear of it everywhere else.
`circle_torus_roots(&ring, 1.0, 2.0, &torus, band)`, an arc one radian
(0.5 m) from the touch, returns `Err(Escalated(Indeterminate {
predicate: "circle_torus_sub_side", .. }))` with margin 1.2e-9 at ε 1e-9,
1.3e-6 at 1e-6 and 1.4e-12 at 1e-12. That is the same refusal as the
arc `[−0.5, 0.5]`, which holds the touch.

## What a fix needs

A piece of the turn that no part of `[t0, t1]` reaches decides nothing
the caller reads. The side decision can be skipped there, or charged
against its distance from the arc, as the line × cone lane does for root
slack (`bool_line_cone_root_slack`).

