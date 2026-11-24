---
id: a-cut-cones-cap-arcs-are-enclosures-at-interval-so-it-refuses-where-f64-holds
kind: issue
title: At Interval a cut cone's two section arcs are held as enclosures, which one_conic's lifted arm never reads as one value, so the body refuses on its cut cap where the f64 body holds
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [two-arcs-of-one-ellipse-stated-a-quarter-turn-apart-are-not-one-conic, a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval, 7605]
---


## Finding

From `embed/conic-symmetry` (2026-11-23), measured on a scratch tree:
`embed/cone-image` `a56c9702c` (PR 7605, on the landing branch) with
this unit's commit `e2a356833` picked onto it, never pushed.

On that tree seven frustum cuts through the wall alone (on and off the
grid, tilted toward a ruling and off it, either side kept), each as
built, turned about a generic axis, a hundred units out and mirrored,
read `Embedded` at `f64` in 28 of 28 asks through
`topo::clearance::embedded`. Built at `Interval` the same 28 refuse:
`Uncertified { Departure }` in 27 and a depth `Budget` in one. For the
two cuts tilted toward a ruling and kept above the plane, as built,
turned and moved out (6 asks), the `Interval` refusal is on the cut cap against a half-cone across a section arc
(`a: 5v1, b: 8v3, shared: Edge(11v1)`), the pair and exit the `f64` body
refuses at with the stored reading off. The others refuse first on
another pair: `2v1, 5v1, Edge(9v1)` (9, the cuts tilted off a ruling),
`5v1, 7v1, Edge(7v1)` (4, mirrored), `1v3, 3v1, Edge(13v1)` (8, the
side below); the kinds of those pairs were not read.

At `f64` the two arcs are one stated conic (`one_stated_conic`: one is
the other's exact restatement). A body held as enclosures stores no
statement (`Rim::stated` is `None`), and `one_conic`'s lifted arm reads
two lifted ellipses as one only where every component of their centres,
axes and semi-axes is a point: an ellipse held on its arc derives its
centre in interval arithmetic, so it is a bracket, and a bracket is not
one value even against itself.

**Not checked:** that the cap's corner is the cause of the `Interval`
refusal on the cut cap, rather than the cone image's own reading at
`Interval`: the payload matches the `f64` body's with the stored arm
off, and no probe told the two apart. The refusals on the other pairs are not this row's and were not
traced.

## What a fix needs

A reading of "one conic" for two enclosures that is exact per member:
the same question
`a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval`
asks of two circles (identical brackets read as one value was measured
there and not taken).

## Pin

None on main: the cone's section image is PR 7605's. On main every
split cone refuses at both scalars
(`self_intersection_proves_injectivity::a_cut_cone_is_embedded_in_every_placement_and_its_wall_holds_alone`).
