---
id: two-arcs-of-one-ellipse-stated-a-quarter-turn-apart-are-not-one-conic
kind: issue
title: one_stated_conic reads two arcs of one ellipse as two conics where their frames are stated a quarter turn apart with the semi-axes exchanged, so a plane cap a cone's split mints refuses at its two-arc corner
status: open
opened: 2026-11-23
priority: P1
cost: E
---


## Finding

From `embed/cone-image` (2026-11-23).

`continues` (`crates/topo/src/injective.rs`) reads a corner of two arcs
as [`Reading::Conic`] only where `one_conic` reads their carriers as one
conic. At `f64` that is `one_stated_conic`: the two stored ellipses one
value, their frames one up to the signs of their axes. Two arcs of one
ellipse stated with `u_ref` a QUARTER turn apart and `semi_u`, `semi_v`
exchanged are the same ellipse and read as two.

A cone's split mints them so. The frustum of radii 0.7 and 0.31 over a
height of 0.9 about `y`, split by the plane through `(0, 0.86, 0)` whose
normal is `y` tilted 0.3 toward the azimuth 0.7, the side below
(`placed_cut(OFF_GRID, (0.86, 0.3, 0.7), false, AsBuilt)` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`): the cut's
plane cap holds two section arcs meeting at a vertex on a ruling, and
the trace prints, for that corner,
`corner=true alone=false continues=false one_conic=false`, the two
stated carriers

- `Ellipse { origin (0.0342, 0.8462, 0.0288), axis (−0.2260, −0.9553, −0.1904), u_ref (0.6442, 0, −0.7648), semi_u 0.33031, semi_v 0.34891 }`
- the same origin and axis, `u_ref` square to that one in the plane,
  `semi_u 0.34891, semi_v 0.33031`.

The cap is `Structural` at that corner (`beside_curve`'s unread corner,
690 exits at one vertex) and the body refuses
`Uncertified { a: 3v1, b: 8v1, shared: Edge(12v3), arm: Departure }` in
every placement (as built, turned, a hundred out, mirrored). The cone
side of the pair is presented (`Of(zero = Curve)`, `corner = true`), and
the two halves of the wall asked alone hold.

The same cut tilted toward a ruling (`psi = 0`) holds: there the split
states both arcs in one frame. A subtract-built cut (a frustum less a
half-space tilted about `x`, running out through the cap) prints the
same pair of frames at `psi = 0` and refuses the same way.

**Not checked:** whether any cylinder cut mints such a pair (the rod
less a tilted slab through its cap holds at both scalars).

## What a fix needs

`one_stated_conic` (and `one_conic`'s lifted arm, which already admits
`u_ref` "equal or opposed") reading an ellipse up to a quarter turn of
its frame with its semi-axes exchanged. `continues` reads the two arcs'
directions at the vertex off each arc's own parametrization, never
mapping one onto the other, so nothing downstream assumes one frame.

## Pin

`self_intersection_proves_injectivity::a_cap_cut_under_a_generic_tilt_refuses_on_its_cut_cap_alone`.
