---
id: point-in-solid-refuses-an-l-shaped-torus-face
kind: issue
title: point_in_solid refuses a body whose torus wall is L-shaped in the chart, which the plane x torus boolean now mints
status: closed
opened: 2026-10-12
priority: P1
cost: M
rides_with: torus-face-bounded-by-an-oblique-circle-refuses-point-classification
closed: 2026-10-18
---


Found by GERM's U-T3 (`germ/plane-torus-chord-circles`, the plane × torus
chord for the meridian and concentric circles), 2026-10-12.

## Measured

`crates/sweep/tests/contact11_torus_chart_l.rs`
`a_notched_half_donut_answers_its_volume`: the half donut (`R = 2`,
`r = 1/2`, half a turn about `y`) less the bar `[0, 3] × [0, 1] × [−3, 3]`
now builds. Its volume is `3π²/8` and it is valid at tier 3, but both its
torus walls are L-shaped in the chart (a parallel cut by the bar's floor,
a meridian by its side). `topo::point_in_solid` on the result refuses
`PartialTorusFace { face }` at every point tried: the notch
`(√2, 0.25, −√2)`, under it `(√2, −0.25, −√2)`, the other quarter's top
and a point past the half turn.

The refusal is raised by `boolean/solid_contain.rs`
`torus_chart_trim` (about :2206), whose two classes are the full-revolve
band and the partial-revolve rectangle; the chart-box guard CONTACT-11
added (`chart_polygon_box`) keeps an L face from being trimmed by its
hull, which is right. The row pins the refusal.

## What the taker owes

A containment reading of an L-shaped (or any chart-polygon) torus face,
or a narrower refusal that leaves the rest of the body answerable. Then
`contact11_torus_chart_l` asserts `Out` in the notch and `In` under it,
which `work/contacthold/notched-half-donut-owes-its-notch-and-volume-when-torus-plane-lands.md`
is parked on.

## More evidence: every torus face an oval bounds (GERM U-T4, 2026-10-12)

Past U-T4's spiric chord the plane × torus boolean mints torus faces
bounded by spiric oval arcs, none a chart rectangle. Measured on
`germ/plane-torus-chord-spiric`
(`crates/sweep/tests/germ_plane_torus_spiric.rs`):

- `point_in_solid` refuses `PartialTorusFace` at every sampled point of
  every body those poses build (S1, M3, P4, the box between two ovals'
  planes, B1, B2 on the donut, the one-face tori and the seamed torus;
  B5, B5′ on the fat tori), so the rows read it nowhere and say so
  (`SKIPPED (… ): point_in_solid: it refuses PartialTorusFace on every
  built body`);
- the result's shell sort reads it too, and refuses
  `Pieces(Probe { source: PartialTorusFace })` (letter `s` in the rows'
  tables): ∩ both orders and T ∖ P of the box between two ovals' planes,
  ∩ both orders of B1 and the tie bar, P ∖ T of B2, and ∩ both orders of
  B5 and B5′ on every fat torus.

When this lands those rows read `point_in_solid` at every sampled point
and the stops move to bodies.

## More evidence (GERM, `germ/one-face-torus-pierce-ring`)

The torus less a box corner refuses `PartialTorusFace` at every one
of 48 sampled points, in the ∪ and `T ∖ P` bodies (∩ and `P ∖ T` read
at every point): the box `x ≥ 0 ∧ y ≥ h` against the one-face torus
with its vertex on the inner equator (`h = −0.1, −0.3, −0.45`), and the
bar `[−3, 3] × [0.2, 1.2] × [0, 3]` against the donut. Both are pinned
in `crates/sweep/tests/germ_plane_torus_chord.rs`
(`holds_to_the_truth_but`, `PARTIAL`), and go red when containment
reads such a wall.

## 2026-10-17 — rides the torus region reading (RESTREAD)

Moved from INSIDE by `git mv`: it and
`torus-face-bounded-by-an-oblique-circle-refuses-point-classification`
are one defect (the torus arm reads only chart rectangles), weighed by
two designers and decided there. One unit closes both. Raised to P1
with its carrier: the shell sort refuses whole ops on it.

## Closed (RESTREAD, `restread/torus-region`, 2026-10-18)

Closed with its carrier,
`torus-face-bounded-by-an-oblique-circle-refuses-point-classification`,
whose Closed section says how. `contact11_torus_chart_l` asserts `Out`
in the notch and `In` under it; the GERM rows above read
`point_in_solid` at every sampled point, and their sort stops are
bodies held to the truth.

