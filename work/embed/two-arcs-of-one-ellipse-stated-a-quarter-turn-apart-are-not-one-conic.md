---
id: two-arcs-of-one-ellipse-stated-a-quarter-turn-apart-are-not-one-conic
kind: issue
title: one_stated_conic reads two arcs of one section ellipse as two conics where they are stated a half turn apart (every plane cap a cone's split mints, since the anchored ellipse) or a quarter turn apart with the semi-axes exchanged, so the cap refuses at its two-arc corner and no cut cone is embedded
status: open
opened: 2026-11-23
priority: P0
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

## 2026-11-23 — on the landing branch with main's anchored ellipse, EVERY cut cone refuses here (`embed/cone-image`)

Measured on `embed/cone-image` merged with `embed/wave3-land`
`8d34076a9e` (which holds main `50c8275810` and PR 7203's conic
accessors). `one_stated_conic`'s ellipse arm is now
`e.same_content(o)` and one `v`; before that merge it was
`one_stated_plane` on the two frames (each axis equal or opposed) and
equal semi-axes.

The split of the frustum of radii 1 and 0.5 over a height of 1 by the
plane through `(0, 0.5, 0)` tilted 0.25 toward `+x`
(`placed_cut(ON_GRID, (0.5, 0.25, 0.0), true, AsBuilt)`) states its two
section arcs

- `Ellipse { at: Frame { origin (−0.66509, 0.66982, 0), axis (0.24740, 0.96891, 0), u_ref (−0, 0, 1) }, held: Arc { phase: (0, −1) }, semi_u 0.75619, semi_v 0.78689 }`
- the same `at.origin`, axis and semi-axes, `u_ref (0, −0, −1)`, `phase (−0, 1)`.

Their derived `frame()`s have one origin `(0.09734, 0.47515, 0)`, one
axis, `u_ref` opposed: the same ellipse restated a half turn on.
`same_content` is `false`, so `continues` is `false`, the cut's cap is
`Structural` at the vertex where the arcs meet, and the body refuses
`Uncertified { a: 5v1, b: 8v3, shared: Edge(11v1), arm: Departure }`,
the half-cone `5v1` against the cap `8v3` across a section arc.

On the base before that merge (`ef332af96`) the same bodies were
`Embedded` at `f64` in every placement, and 12 of the 24 cone/cone
census tests passed with the gate asking. On the merged tree every cut
cone probed refuses on a plane cap (15 placements of five bodies, the
needle and the near-flat cone), with the two halves of the wall holding
when asked alone. The 12 census tests were not re-run through the gate
on the merged tree.

**The question this leaves** is what one stated conic is for two
anchored arcs of one section: by content they differ, by derived frame
they are one up to a half turn, and the derived centre is rounded. It
changes what `continues` certifies, so it is a ruling, not a repair.

## Pin

`self_intersection_proves_injectivity::a_cut_cones_wall_holds_in_every_placement_and_its_body_refuses_on_a_cap`
and `…::a_needle_and_a_near_flat_cone_cut_read_as_any_cone`
(`refuses_on_a_cap_and_its_wall_holds`).

**Re-filed where the minting door lives** (the EMBED orchestrator's ruling, same day): the half-turn case is ONEROUND's, `work/oneround/two-arcs-of-one-cone-section-are-stated-as-two-anchored-ellipses-so-no-reading-knows-them-one-conic.md` (P0). `one_stated_conic` comparing by content is right and stays; the defect is that one section is stated twice. The census re-run on the merged tree: 1 of 24 pass. This row keeps the pre-merge quarter-turn sighting.
