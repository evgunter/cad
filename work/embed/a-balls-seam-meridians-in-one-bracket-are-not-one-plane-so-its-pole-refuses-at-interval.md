---
id: a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval
kind: issue
title: A ball whose centre is a bracket stores its two seam meridians in one bracket, which is not one value, so no plane lists both and its pole refuses Departure at Interval
status: open
opened: 2026-11-13
priority: P2
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
