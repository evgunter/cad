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
plane built from either lists both only where `coplanar`
(`crates/topo/src/injective.rs`) reads them as one plane: a point-scalar
body's stored frames one up to the signs of their axes, or the lifted
frames as one value, every component a point (`one_plane`). A bracket is
not one value. Two edges
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

## The f64 half read off the stored frames (EMBED, lane one-plane, 2026-11-22)

**Traced** (`embed/one-plane-measure`: main `2da45c59f` with PR 7115's
branch, a print of `planes_through`'s conic frames), the unit ball on three
turned sketch planes, f64:
- **after** PR 7131: the two meridians' stored frames are one origin, one
  `u`, and `v`, `w` exactly negated, bit for bit. Their lifts keep the
  origin a point, and each axis is the enclosure of its frame's polar
  factor, one or two ulps wide per component (frame 1's `w.x`
  `[-0.39473979817379984, -0.3947397981737997]`). `one_value` reads no
  point, so `one_plane` is false.
- **before** (PR 7115's branch at `e718644b9`, on `dc7059c445`): the
  lifted axes are the stored bits as points
  (`[-0.3947397981737998, -0.3947397981737998]`), and `one_plane` holds.
- The plain-frame ball's axes are exact (`(0, 0, ∓1)`), and stay points.

So the cause is the polar read meeting `one_plane`'s point comparison. It
is not a defect on ONEHOME's side: the polar factor of a frame that is not
exactly rigid is not an `f64`, and a point would not enclose it.

**What landed** (`embed/one-plane`):
- `planes_through` lists two conics in one plane where `coplanar` reads
  it: their lifts as one value (`one_plane`, unchanged), or, on a
  point-scalar body, their stored frames (`Rim::stated`, PR 7131's) one
  origin and each axis `u`, `v`, `w` the other's or its negation
  (`one_stated_plane`). The polar factor of `M·D`, `D` a diagonal of
  signs, is `M`'s times `D`, so the two lifts enclose one origin and one
  axis line exactly.
- `one_stated_conic` reads the same comparison. It compared `w` alone,
  which the polar read leaves short: the polar factor's axis leans on
  `u` and `v` wherever the stored axes are not exactly rigid, so two
  circles stored with one `w` and two `u` are two circles a rounding
  apart. `one_conic` now takes the stored reading as one more way to be
  one conic, beside the lifts' own (it returned the stored answer
  alone).

**Measured** (receipts are discharged / refused / splits):
- The three turned balls, f64, at the default ε, 1e-6 and 1e-12, with
  PR 7115: 257 / 0 / 254, 295 / 0 / 292, 293 / 0 / 290, against
  642 / 12 / 651, 589 / 8 / 594, 611 / 10 / 618 refusing `Departure`.
  On main without PR 7115 (this lane's body row): the first plane is
  embedded at 257 / 0 / 254, and the third plane's turn placed a hundred
  units off the origin at 291 / 0 / 288; the second plane runs out of
  pairs (`Budget(Pairs)`, 46,158 / 19,378 / 65,533), and holds with
  PR 7115's reading of the pole (above), so it is not pinned here.
- Their `Interval` twins, the frames minted at `Interval` (no stored
  frame), refuse `Departure` on `Edge(1v1)` on every plane (about 780 to
  820 splits). The plain-frame `Interval` ball holds (250 / 0 / 247).
- The snowman at `Interval` is unchanged: it holds no stored frame, so
  this reading does not reach it.

**What is left on this row** is the `Interval` half, and it is wider than
the snowman: every ball built at `Interval` in a turned frame. No stored
fact reaches the engine there. Weighed:
- the sphere chart's seam: a plane from the face's own frame vanishes on
  that face's `u = 0` and `u = π` for every member, but the two half-bands
  store two sphere frames, so the same "two brackets alike" moves to the
  faces;
- `carrier_eq`: decides at the band, a concession this reader does not
  have;
- `RigidMotion`: a value, with no record of being minted from another.
So the `Interval` half still needs the identity carried (one `Sym` node
through `boundary_of`) or the one-band build (LATHE's row). Should the
one-band build land, a ball has one meridian, `planes_through` sees one
conic at the pole, and `coplanar` has nothing to compare.
