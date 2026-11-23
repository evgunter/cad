---
id: an-interval-cone-cut-refuses-at-its-caps-two-arcs-and-beside-its-rulings
kind: issue
title: At Interval the two halves of a cut cone's wall refuse where they hold at f64: a cone cell a rounding past a ruling is kept where the face's extent reaches past the ruling by its bracket; and the plane caps' two arcs of one conic are never one value
status: open
opened: 2026-11-23
priority: P1
cost: M
---


## Finding

From `embed/cone-image` (2026-11-23). A frustum split by a tilted plane
(`cone_cut` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`): the two
halves of its wall, asked alone through `topo::clearance::embedded`,
hold at `f64` in every placement tried (as built, turned, a hundred
units out, mirrored across a generic plane, on and off the dyadic
grid), and the whole body was `Embedded` at `f64` before main's
anchored ellipse (it refuses on a plane cap since; the last section).
The same body built at `Interval` refuses in every one of them,
`Uncertified { Departure }` or `Budget(Depth)`, and its halves alone in
most. Two exits, traced on the measuring branch
(`embed/cone-image-measure`, `CV_TRACE`):

1. **The plane caps.** A cap bounded by two arcs of one conic (the cut's
   cap by two arcs of the section ellipse, the frustum's own caps by two
   half circles) is `Structural` at the vertex where the arcs meet:
   `beside_curve`'s unread corner, `continues = false`,
   `one_conic = false`. An `Interval` body states nothing
   (`Rim::stated` is `None`), and `one_conic`'s lifted arm reads two
   brackets as one conic only where every component is a point. Traced
   on the wall cut off the grid, tilted toward a ruling, as built: caps
   `8v3` (714 and 642 exits at its two vertices), `1v3`, `2v1`; payload
   `5v1`/`8v3` across `11v1`, the cone side presented
   (`Of(zero = Curve)`).
2. **A cone cell a rounding past a ruling.** Traced on the cap cut as
   built: the half-cone `3v1` in the cell `u ∈ (3.13852, π)`,
   `v ∈ (−1.7605, −1.7586)`, beside its ruling `u = π` on the side the
   face is NOT on, reads `Local` at `present_chart`'s
   `sides_are_the_faces` at every depth, and the pair ends
   `Budget(Depth)`. The same exit is taken a turn on, at
   `u ∈ (6.280, 2π)`, on the subtract-built cut under a generic tilt.
   At `f64` those cells are dropped.

With the caps left out, the two halves of the wall asked alone hold at
`Interval` where the cut is as built, turned or a hundred out and tilted
toward a ruling, and refuse (`Uncertified`, `Budget`) mirrored, under a
generic tilt, and where the cut runs out through the cap: 28 placements
probed, all 28 hold at `f64`, 6 at `Interval`.

**Cause of 2, isolated for the generic tilt** (the unit's review,
`embed/cone-image-review-7c3e9a`, its `extent_u_nudged` probe). The
face's extent holds the section arc's enclosure (`row_of`,
`crates/topo/src/clearance.rs`), whose ends are brackets as wide as the
stored row's `u0` (3.5e-14 on the traced body), where the ruling's line
is a point; a cell overlapping the extent by more than `chart_slack` is
not dropped. With the arc's azimuth hull drawn `1e-9` rad inside
itself, at the extent only, the halves `Holds` at `Interval` under the
generic tilt as built and turned: the extent's reach past the ruling by
its bracket is that refusal's whole cause. Mirrored and tilted toward a
ruling the halves still refuse with the hull drawn in, so that
placement has another cause, not traced.

**What should read that cell.** Not the extent: an enclosure of the arc
has to hold the arc, and an arc that ends on the ruling has an
enclosure that straddles it. The cell is beside the vertex the ruling
and the arc share, so it is the corner's to read: the ruling's corner
reading (`present_line_corner`) already decides that the arc leaves the
ruling onto the face's side and that the ruling's other side holds no
face there. A cell past the ruling, in reach of that vertex, would be
read outside by the corner where the extent cannot say so.

The cylinder analogue takes the
same class on unchanged code: the rod less a slab tilted 0.3 about `x`
and spun 0.7 about `z` is `Embedded` at `f64` and
`Uncertified { a: 1v1, b: 4v1, shared: Edge(6v1), arm: Departure }` at
`Interval`, and holds at both with no spin (exit not traced).

Before `embed/cone-image` these bodies refused at both scalars at depth
0, so nothing held at `Interval` that now refuses.

## Pin

`self_intersection_proves_injectivity::a_cut_cones_wall_holds_in_every_placement_and_its_body_refuses_on_a_cap`
pins the bodies `Refused` at `Interval` and the two halves holding at
both scalars where they do.

**2026-11-23, on the merged landing branch:** the bodies refuse at
`f64` too, on a plane cap
(`two-arcs-of-one-ellipse-stated-a-quarter-turn-apart-are-not-one-conic`),
so exit 1 is no longer an `Interval` difference. Exit 2 is: the halves
alone hold at `f64` in every placement and at `Interval` only where the
row pins them.
