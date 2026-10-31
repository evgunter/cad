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
(`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`) at ε 1e-9, 1e-6
and 1e-12 all read `Undecided`.

## Closed

`one_arc_pair` renames an undecided walk or end reading after the
fact, where the carrier's shape accounts for it. Only `Undecided` is
renamed: no answer becomes a refusal and no refusal an answer.

- `OneArcRefusal::AlongSide { chart }`: the carrier's image on that
  chart lies along a side of its domain (`four::along_side`: every
  control point's `u`, or every one's `v`, within the end box of one
  side, `four::end_reach`, `ε/8` metres at that axis's chart speed).
  This places the image, not the locus. An exact `IsoArc` polyline lies
  on the side exactly; a fitted boundary-row image 1e-17 to 2e-16 off
  it. Takes precedence over the next.
- `OneArcRefusal::EndOnSides { sides }`: the failure is an end's, its
  `reaches_end` reading or a box walk that stopped in a piece meeting
  that end's box, and that end lies on that many domain sides of the
  two charts (`four::sides_at`), that end's count alone.

A box missing the domain, a failed link and the chain's own
first/last guard stay unnamed.

The single-chart twin (`certify::probe_tube_chart`) stays `Undecided`
with a comment saying why: past its side arm (PR 4012) an `Undecided`
may be the arm's own in-band reading, which is indeterminate. SHELL's
moved fitted cap, the one chart-tube `Undecided` along a side, keeps
its pin; the ssiedge row records the measurement.

Witnesses, each at ε default, 1e-6 and 1e-12:
- `four::tests::a_carrier_along_a_charts_side_refuses_by_name`: told
  nothing the walk stays `Undecided`, told `Some(0)` it reads
  `AlongSide(0)`, and a proved arc told the same stays `Ok`.
- `four::tests::an_end_at_a_charts_corner_refuses_by_its_sides`: the
  corner row's probe, `EndOnSides(2)`.
- `four::tests::an_end_refusal_counts_that_ends_sides_alone`: over one
  box, a start on two sides and an end on three read the start's two.
- `four::tests::an_image_off_a_side_by_twice_the_end_box_is_not_along_it`.
- `four::tests::an_end_on_a_domain_side_is_reached_by_the_root_there`
  (unchanged): an end on one side still proves.
- `spline_pair_arc_rim`: the three boundary-row rows re-pinned on
  `AlongSide` with the chart named; `a_rim_ending_on_both_charts_sides_refuses_by_them`
  (the √ε span) reads `EndOnSides { sides: 2 }`; the interior control of
  `an_exact_charts_tube_is_about_the_rim` still certifies.
- `ssi_limb3_one_arc::a_flush_side_on_a_conical_or_twisted_wall_certifies`
  (unchanged).
- `a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim` (lily) pins
  `AlongSide { .. }` on the rim's certification in both orders.

Mutations, each red: either naming removed (its rows read `Undecided`
again); the failure attributed to the other end without the failing
piece's check (`EndOnSides(3)`); the end box loosened to `ε` (the image
`2·ε/8` off reads along).
