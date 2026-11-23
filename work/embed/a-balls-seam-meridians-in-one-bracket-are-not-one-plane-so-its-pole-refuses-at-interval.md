---
id: a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval
kind: issue
title: A ball whose centre is a bracket stores its two seam meridians in one bracket, which is not one value, so no plane lists both and its pole refuses Departure at Interval
status: dispatched
opened: 2026-11-13
priority: P0
cost: M
pr: 7328
branch: embed/one-plane
---


Found by CLEAR's seam lane (`clear/seam-meridians`), closing
`revolve-stores-a-balls-two-seam-meridians-in-two-planes-so-a-corner-keeps-a-band-reading`.

## Finding

A full revolve now stores a ball's angle-π meridian with the angle-0
meridian's centre and reference direction and its axis turned over
(`sweep::revolve::full::build_wire`, `placed_segment_spec_about`). The
plane built from either lists both only where `one_plane`
(`crates/topo/src/injective.rs`) reads the two lifted frames as one
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
(`crates/sweep/tests/self_intersection_proves_injectivity.rs`) pinned the
refusal until 2026-11-21 (below). The unit ball's frame is all points, so its pole holds.

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

**Read off the stored frames, then withdrawn.** A first head of PR 7328
listed two conics in one plane where a point-scalar body stores their
frames one up to the signs of their axes (`one_stated_plane`: one origin,
each of `u`, `v`, `w` the other's or its negation; the polar factor of
`M·D`, `D` a diagonal of signs, is `M`'s times `D`). On main alone it
made the turned balls embedded at f64 (257 / 0 / 254 and, 100 units out,
291 / 0 / 288, discharged / refused / splits), the three tour dice
(78 → 81 of 85) and the f64 cavity row
`shell8_r2_probes::r2_three_solids_on_one_sphere_chart_open_additively`.
PR 7315's plane through a vertex alone (`embed/apex-separator`) reads the
same poles by another route, at f64 and at `Interval`. On this lane's
head merged with that branch and with the at-rest gate's harness
(`embed/one-plane-measure2`), with the stored arm switched off:
- the tour walk: every one of the 85 lines identical, receipts included
  (81 of 85; the lantern is
  `the-lily-lantern-refuses-departure-on-its-cone-torus-pair-in-a-tilted-frame`);
- the f64 cavity switch, 221 tests: every ask's verdict and receipt
  identical, the three-solids row shelling either way;
- the 24 gate-census tests the arm had moved: 1,670 asks, every verdict
  identical; one test's receipts move,
  `intent_e_operation_frame_of_reads::every_corpus_document_reads_alike_in_a_second_computing_frame`,
  71 of its 1,254 ball asks embedded at 255–256 splits with the arm and
  258–260 without.

No verdict depended on it, so `planes_through` reads the lifts alone, as
before, and this row's f64 half closes by PR 7315's reading.

**What landed** (PR 7328): **main's `one_stated_conic` was unsound.** It
read two stored circles as one by their origins, axes `w` (or `-w`) and
radii. The review of PR 7328 measured 135 pairs with one origin and one
`w` bit for bit, radius 1, and `u` one or two ulps off, whose lifted `w`
enclosures are disjoint: two planes for certain, each read as one circle.
`one_stated_conic` now compares the frames by `one_stated_plane`, and
`one_conic` reads the lifts as one value besides. Comparing `w` alone
was sound on two populations: frames one up to signs, which the stored
arm now reads, and exactly rigid frames, whose axis is a point, which
the lifts' arm keeps.

**What is left on this row** is the `Interval` half without PR 7315's
reading: no stored fact reaches the engine at `Interval`. Weighed:
- the sphere chart's seam: a plane from the face's own frame vanishes on
  that face's `u = 0` and `u = π` for every member, but the two half-bands
  store two sphere frames, so "two brackets alike" moves to the faces;
- `carrier_eq`: decides at the band, a concession this reader does not
  have;
- `RigidMotion`: a value, with no record of being minted from another.
Should PR 7315 land, its plane through the pole reads them: on a merge of
the two branches the review of PR 7328 measured a turned ball's
`Interval` twin embedded at 315 / 0 / 312, and the apex-separator lane
the snowman's small ball embedded at `Interval`. Re-measure this row then.

## The snowman holds at Interval (lane `embed/apex-separator`, 2026-11-21)

`departure` now also tries the plane through the pair's one shared
vertex, each face presented against that vertex alone, where shared
edges end there (`at_end_vertex`, `crates/topo/src/injective.rs`). At a
pole that plane is `q` the pole's stored point and `n` from the faces'
headings, so it vanishes at the pole for every member, whatever the
meridians' brackets; each half-band's polar cell is its collapsed side
there. The snowman at `Interval` holds in 579 pairs (f64: 563), the
count this row measured for reading identical brackets as one value,
without that reading. `a_two_ball_union_holds_with_its_poles_read` pins
it holding at both scalars; red with the new arm off (the lane's
mutant M1).

What stays true: two meridians stored in one bracket are still not one
plane, so the meridians' plane still lists its own alone. Whether
anything else reads that plane at a pole is not measured.
