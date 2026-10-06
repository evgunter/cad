---
id: face-boundary-walks-spell-the-linked-prelude-by-hand
kind: issue
title: Face boundary walks spell the linked member prelude by hand
status: dispatched
opened: 2026-10-06
priority: P3
cost: E
---

## What

(Found by PR 4078's review, style Q1.) A face's boundary is walked
through links by one prelude: `Body::face_loops_linked` →
`loop_walk(first).closed("loop", first)` → `proven` member →
`linked` edge, with an isolated-vertex loop's point beside it. PR 4078
added `Body::face_boundary_linked` (`live.rs`), which yields each
member as `BoundaryMember::Isolated(point)` or
`BoundaryMember::Edge { ek, edge }`, and moved the four sites in files
it touched onto it (`census.rs` `boundary_axial`, `boundary_reach`;
`boolean/boxes.rs` `face_box`'s axial window, `boundary_hull`). A walk
that spells the hops by hand can drop one, and no witness sees the
dropped hop unless a mutation is aimed at it.

The other sites that spell the prelude (or a prefix of it) by hand.
Line numbers ride along and may be stale:

- `boolean/ops.rs` `face_boundary_meets` (`:1219`) and
  `faces_by_vertex` (`:1529`)
- `boolean/carrier_cross.rs` `boundary_crossing` (`:114`)
- `boolean/finish.rs` `face_cycles` (`:820`)
- `boolean/boxes.rs` `face_window_steps` (`:1133`): zips each cycle
  with `pcurves::lifted_images`, so it needs the cycle whole
- `boolean/reduce.rs` `boundary_meets_circle_only_at` (`:2919`)
- `boolean/rim_wedge.rs` `face_boundary_arcs` (`:875`)
- `boolean/surface_group.rs` `unmated_boundary` (`:151`)
- `pcurves.rs` (`:2225`)
- `merge_faces.rs` `outermost_survivor` (`:2046`) and
  `boundary_points` (`:2419`)
- `offset_nappe.rs` `corner_stations` (`:189`)
- `offset_together.rs` the solids walk (`:840`)

Some read the member half-edge or its start rather than the edge; the
iterator grows those fields when the first such site moves onto it.

**What the sweep could not match.** It looked for
`closed("loop"` and `face_loops_linked(`. A face walk that spells the
loop list as `once(face.outer).chain(face.rings…)` (about 30 sites:
`validate.rs`, `euler.rs`, `seqgen.rs`, `splitting/finish.rs`,
`chart_region.rs`, `boolean/contain.rs`, `boolean/shell_witness.rs`,
…) reads its hops typed or absent, not as links; those are the
torn-hop rows' and the families row's to convert, and each lands on
this iterator when it does.

## Done when

Every face boundary walk that resolves its hops as links goes through
`Body::face_boundary_linked` (or a sibling yielding the fields it
needs), and no site spells the four hops by hand.
