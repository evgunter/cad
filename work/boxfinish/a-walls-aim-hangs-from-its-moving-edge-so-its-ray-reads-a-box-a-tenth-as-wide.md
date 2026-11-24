---
id: a-walls-aim-hangs-from-its-moving-edge-so-its-ray-reads-a-box-a-tenth-as-wide
kind: issue
title: A wall's aim hangs a fixed step under its moving top edge, so its ray reads only a box a tenth of the height wide
status: open
opened: 2026-11-15
priority: P1
cost: M
---


Found by the ray-readings unit
(`an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice`,
branch `boxfinish/ray-readings`), measuring which reading stops which
aimed ray on the extrude door's slab at depth `1 ± w` (`Sym<Interval>`).

**What stops a wall's ray.** `side_reader::loop_aim` steps from a point
`m` of the face's first edge in walk order. On the door's slab and on
the fixture brick a wall's first edge is its TOP edge, so the wall's aim
stands a fixed step (`0.19` on the unit slab) under an edge that rides
the boxed depth `h`. Every member's ray passes that edge `0.18964` away
and crosses the cap's plane at advance `−1.628`, at every depth (read
at `f64`, depths 0.75, 1 and 1.25). Over the box the walk reads those
between separately stored copies of `h`: the aim's `z` (from the first
edge's stored line), the cap's plane origin, the other top edges'
lines, and the upright edges' far ends. Each difference of two of them
is `h − h`, which is not zero at `Interval`.

**What the unit took, and what is left.** The unit re-posed the loops'
turn and the aim's step, which carry the caps' rays; neither moves a
wall's ray, which reads to `± 0.05` on the door's slab before and
after. Three readings stop it, none of which can be re-posed from what
the walk holds by arithmetic:

- the advance to the cap's plane over the wall, and so the turn of
  that cap about the crossing (`side_at_infinity_winding`, the first
  stop, at `± 0.06`). Reading that advance from the aim's own edge
  lifts the walls to `± 0.09` (measured, not shipped):
  `the-aimed-faces-advance-is-zero-only-through-its-first-edges-incidence`;
- the ray's gap from its own top edge and from the top edges of the
  other walls (`side_at_infinity_edge`): the aim's `z` against an
  edge's line;
- the span test of an upright edge beside the aim
  (`side_at_infinity_edge_span`): how far the closest point lies under
  the edge's top end, `h` (the aim's) against `h` (the edge's length).

The caps' rays carry the slab instead: both read at `± 0.25`, the
bottom cap's to `± 0.46`.

**What would lift it.** Either of:

- the symbolic tier valuing a form that is constant in the parameters
  (`(h − s) − h` is `−s`): `Sym` proves a zero form today and values
  nothing else (`crates/geom-core/src/sym.rs`);
- a wall's aim stepped from an edge that does not move. That changes
  where aims land at `f64`, which is SIDETIES's ground
  (`work/sideties/`).

**Who it holds.** No red row: a prism reads by its caps. It bounds a
body none of whose planar faces is square to the moving direction, and
it is why the walls' rays are not asked for in
`side_reader::boxed_brick_rows::a_lined_boxed_brick_reads_by_its_caps_own_rays`
and in `sweep`'s
`an_aimed_ray_over_boxed_door_bodies::a_door_slab_reads_its_side_by_its_caps_rays_over_a_wide_box`.

**The test that says it is closed.** Those two rows read all six aims
at `± 0.25`.
