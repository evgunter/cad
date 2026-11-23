---
id: an-interval-cone-cut-refuses-at-its-caps-two-arcs-and-beside-its-rulings
kind: issue
title: At Interval a cut cone refuses where it is embedded at f64: its plane caps' two arcs of one conic are never one value, and a cone cell a rounding past a ruling is kept where the face's extent reaches past the ruling by its bracket
status: open
opened: 2026-11-23
priority: P1
cost: M
---


## Finding

From `embed/cone-image` (2026-11-23). A frustum split by a tilted plane
(`cone_cut` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`) is
`Embedded` at `f64` through `topo::clearance::embedded` in every
placement tried (as built, turned, a hundred units out, mirrored across
a generic plane, on and off the dyadic grid). The same body built at
`Interval` refuses in every one of them, `Uncertified { Departure }` or
`Budget(Depth)`. Two exits, traced on the measuring branch
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

**Cause of 2, not isolated.** The face's extent now holds the section
arc's enclosure (`row_of`, `crates/topo/src/clearance.rs`), whose ends
are brackets as wide as the stored row's `u0` (3.5e-14 on the traced
body), where the ruling's line is a point; a cell overlapping the
extent by more than `chart_slack` is not dropped. That is the likeliest
cause and was not separated from the next one (the cover's reading of
the shortened ruling at `Interval`). The cylinder analogue takes the
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
