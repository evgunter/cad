---
id: a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval
kind: issue
title: A ball whose centre is a bracket stores its two seam meridians in one bracket, which is not one value, so no plane lists both and its pole refuses Departure at Interval
status: open
opened: 2026-11-13
priority: P0
cost: M
---


Found by CLEAR's seam lane (`clear/seam-meridians`), closing
`revolve-stores-a-balls-two-seam-meridians-in-two-planes-so-a-corner-keeps-a-band-reading`.

## Finding

A full revolve now stores a ball's angle-π meridian with the angle-0
meridian's centre and reference direction and its axis turned over
(`sweep::revolve::full::build_wire`, `placed_segment_spec_about`). The
plane built from either lists both only where `one_plane`
(`crates/topo/src/injective.rs`) reads the two stored frames as one
value: every component a point. A bracket is not one value. Two edges
stored in the same bracket may take different values in two members of
a family; the turning-line family in `injective.rs`
(`a_plane_through_a_turning_line_edge_holds_it_for_every_member`) has
that shape. So the plane lists its own meridian alone.

At `Interval` the snowman's small ball (radius 0.8, centre `y = 1.4`)
stores its centre as `[1.3999999999999997, 1.4000000000000001]`, because
the profile arc's centre rounds. Its pole refuses
`Uncertified { Departure }` on that ball's meridian, and
`a_two_ball_union_holds_with_its_poles_read`
(`crates/sweep/tests/self_intersection_proves_injectivity.rs`) pins the
refusal. The unit ball's frame is all points, so its pole holds.

Measured: if identical brackets are read as one value, the union holds
at `Interval` too, in 579 pairs. That reading is not exact per member,
so it was not taken.

## What a fix needs

One of these:

- The identity reaches the engine as a fact, not as a value. At `Sym`,
  the two centres are one node and the two axes a node and its negation,
  but `boundary_of` (`clearance.rs`) lifts
  each one to a bare `Interval`.
- The two-band build retires.
  `work/lathe/a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote.md`
  (Ev, PR 6014) builds a ball as one band with one meridian. Then the
  second meridian, and with it this row, may go. Re-measure the snowman
  at `Interval` when that lands.

## At `f64` too, in a turned frame, since the carrier frames landed (EMBED, lane pole, PR 7115, 2026-11-21)

Measured by lane pole while merging main into PR 7115.

The unit ball revolved on a sketch plane turned about a generic axis
and moved (three such planes, `turned_plane` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`), f64:
- on main `dc7059c445` with PR 7115's reading of the poles, all three
  held, at 254, 295 and 291 splits;
- on main `94b6b284e`, after the carrier-frames merge (`ec8749552`
  among its commits), all three refuse
  `Uncertified { 1v1, 2v1, Edge(1v1), Departure }`, at
  642 / 12 / 651, 589 / 8 / 594 and 611 / 10 / 618
  (discharged / refused / splits), the same at ε 1e-6 and 1e-12. The
  plain-frame ball holds on both, at 227 splits.

**The exit** (traced on the first plane): the poles are placed and
both half-bands are presented by the corner of a meridian and the
pole's side. `departure` reaches its last `split` with
`planes_through` returning two planes, each listing one meridian
(`on_edges = 1`), where the plain frame gets one plane listing both;
each plane signs one half-band and reads no sign on the other. That is
this row's `one_plane` reading failing on two stored frames that are no
longer one value. Which accessor or constructor of the new frames
breaks the value identity is not traced.

**Not PR 7115's:** `shell8_r2_probes::r2_three_solids_on_one_sphere_chart_open_additively`,
asked through the f64 cavity switch, refuses
`Uncertified { 1v1, 5v1, Edge(1v1), Departure }` at
5,896 / 12 / 5,859 on main `94b6b284e` with main's own `injective.rs`,
and shelled on `dc7059c445`. Its exit is not traced; it is a sphere
pair at the same arm.

Pinned as a refusal by
`a_ball_in_a_turned_frame_refuses_departure_at_its_pole`.

