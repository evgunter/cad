---
id: a-circle-touching-a-torus-off-the-arc-escalates-the-arcs-side-decision
kind: issue
title: circle × torus: a touch off the arc escalates the subdivision's side decision (circle_torus_sub_side) for an arc it does not reach
status: closed
opened: 2026-11-04
priority: P3
cost: M
refs: [root-slack-meters-roots-outside-the-arc]
closed: 2026-11-23
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
slack (`line_quadric_cone_root_slack`).

Moved from HONEROOT to GERM on 2026-11-20, by `git mv`, when HONEROOT merged into GERM: HONEROOT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one GERM carries.

## Closed by U4a (`germ/u4a-local-windows`)

The walk reads past a refused piece and records it as a window
(`half_angle::certified_subdivision_past`); a window that misses the arc
is dropped with the meetings it could hold, where the method's answer is
read (`curve_face::cross::Walked::on_span`), so a refusal off the arc
refuses nothing, through `cross` and the sweep's wrappers alike. Rows:
`curve_face::window_rows::a_touch_off_the_arc_leaves_the_arc_answered`
(the measured case, `[1, 2]` answered at ε 1e-6, 1e-9, 1e-12, `[−0.5, 0.5]`
still refusing) and
`a_first_harmonics_refused_slack_is_local_and_refuses_nothing_off_the_arc`.
