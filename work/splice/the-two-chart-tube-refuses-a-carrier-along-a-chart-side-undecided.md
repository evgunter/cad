---
id: the-two-chart-tube-refuses-a-carrier-along-a-chart-side-undecided
kind: issue
title: The two-chart tube refuses a carrier along a chart's domain side, or one ending on several domain sides, as Undecided rather than by name
status: closed
opened: 2026-10-28
closed: 2026-10-28
priority: P0
cost: M
refs: [a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary, a-pair-walk-end-at-a-domain-corner-refuses-undecided]
---

## What

`ssi/four.rs` `one_arc_pair` refuses `Shortfall::Undecided` for both
shapes of `a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary`:
a carrier lying along a chart's domain side (a boundary row), and a
carrier whose end lies on two or more domain sides at once (the corner
row). Each walks `face_walk` to `EXIT_DEPTH` and the payload,
`TubeNotOneArc { Undecided(ssi_tube_one_arc, MarginDiag(Invalid)) }`,
reads as an escalation: as if more precision might answer it. Neither
shape is a precision question. Under PR 5933's reading a carrier along
a boundary row is a structural selector's curve, which is never asked
limb 3; until that recovery exists, the tube should say what it found.

Measured on main 2026-10-28: the `spline_pair_arc_rim` rows on a
boundary row, both orders, and lily's sweep row
(`a_sheath_on_its_blade_unions_past_the_shared_rim`) at ε 1e-9, 1e-6
and 1e-12 all read `Undecided`.

## Closed

`one_arc_pair` names an undecided walk or end reading where the
carrier's shape says why, and the chart tube does the same past its
side arm. Only `Undecided` is renamed: no answer becomes a refusal and
no refusal an answer.

- `OneArcRefusal::AlongSide { chart }`: the carrier's image on that
  chart lies along a side of its domain, read off the image
  (`one_arc::along_side`: every control point's `u`, or every one's
  `v`, within the end box's half-width of one side, `ε/8` metres at
  that axis's chart speed). An exact `IsoArc` polyline lies on the side
  exactly; a fitted image of a boundary row lies within 1e-17 to 2e-16
  of it (measured on SHELL's moved fitted cap). Takes precedence over
  the next.
- `OneArcRefusal::EndOnSides { sides }`: at an end's box, that end lies
  on that many domain sides of the two charts, each within the same end
  box `reaches_end` reads (`four::sides_at`). Two-chart tube only.

Both are definite (`RefusedArm::SignCertain`), never escalating.

The single-chart twin (`certify::probe_tube_chart`) renames only what
`one_arc` returns `Undecided`, after its side arm (PR 4012) has been
read, so the arm is untouched: the unit rim against `z = 0` along the
cylinder's `v = 0` row still certifies, and so does
`ssi_limb3_one_arc::a_flush_side_on_a_conical_or_twisted_wall_certifies`.
Its one reachable `Undecided` along a side is SHELL's moved fitted cap
(`ssiedge/a-plane-section-along-a-fits-window-edge-proves-no-one-arc`),
now `AlongSide { chart: 0 }`.

Witnesses, each at ε default, 1e-6 and 1e-12 unless said:
- `four::tests::a_carrier_along_a_charts_side_refuses_by_name`: `None`
  stays `Undecided`, `Some(0)` reads `AlongSide(0)`, and a proved arc
  told the same stays `Ok`.
- `four::tests::an_end_at_a_charts_corner_refuses_by_its_sides`: the
  corner row's probe, `EndOnSides(2)`.
- `four::tests::an_end_on_a_domain_side_is_reached_by_the_root_there`
  (unchanged): an end on one side still proves.
- `spline_pair_arc_rim`: the three boundary-row rows re-pinned on
  `AlongSide` with the chart named; `a_rim_ending_on_both_charts_sides_refuses_by_them`
  (the √ε span) reads `EndOnSides { sides: 2 }`; the interior control of
  `an_exact_charts_tube_is_about_the_rim` still certifies.
- `a_sheath_on_its_blade_unions_past_the_shared_rim` (lily) pins
  `AlongSide` on the rim's certification in both orders.
- `r1_lane0_e2e::the_f64_seam_answers_every_public_door` pins
  `AlongSide { chart: 0 }` (default and 1e-12; its 1e-6 red predates
  this row and is unmoved but for the payload, `rungs: 15`).

Mutation: with either naming removed, its rows read `Undecided` again.
