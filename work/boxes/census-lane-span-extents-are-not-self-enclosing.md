---
id: census-lane-span-extents-are-not-self-enclosing
kind: issue
title: The census lane's Span extents do not enclose their locus at f64 by themselves; their soundness rests on what each reader compares them against, which nothing guards
status: open
opened: 2026-10-18
priority: P3
cost: M
---

Found by the review of PR 5308 (`restread/one-edge-box-map`, MINOR 2).

`boolean::boxes::Span` (`crates/topo/src/boolean/boxes.rs`, the type
docs' rounding paragraph) does its arithmetic with no directed rounding.
On the `f64` bracket lane, `Aabb::padded`'s outward ulp plus
`sweep_pad` covers that error. The census lane (`census::face_reach_in`,
`census::edge_reach_in`) adds no pad. So at `f64` its boxes can fall a
few ulps of their scale short of the locus. These are the face extents
(`slab_extent`, `cone_frustum_extent`, `ball_extent`, `torus_extent`,
`torus_window_extent`, `torus_rect_extent`, `conic_extent`) and
`arc_extent`.

None of them takes the root of a cancelling difference, so the shortfall
stays at a few ulps of scale. The spiric's extent did take such a root,
and PR 5308 moved it into `geom::spiric_period_box`, which charges for
its own rounding and encloses by itself.

Before PR 5308 the docs said "the census lane's scalar carries its own
enclosure", which is false at `f64`. That PR's first pass replaced the
sentence with a roster of the census readers' margins, and its review
found the roster incomplete: it missed `separating::apart` (`reduce.rs`,
`ops.rs`), `join.rs`'s `face_reach` and `reduce::gate_maximal_faces`. A
roster is unguarded however complete it is, so the paragraph now states
the gap instead and points here.

**Final state**: every census-lane extent encloses its locus at every
scalar by itself. Either charge each extent's rounding at its scale, as
`spiric_period_box` does, or charge the census entry (`edge_rule_box`,
`face_reach_in`) once by a stated bound on the extents' error. The
claim then needs no reader roster.
