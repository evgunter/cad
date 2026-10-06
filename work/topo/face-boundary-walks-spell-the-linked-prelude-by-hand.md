---
id: face-boundary-walks-spell-the-linked-prelude-by-hand
kind: issue
title: Face boundary walks spell the linked member prelude by hand
status: open
opened: 2026-10-06
priority: P3
cost: E
branch: topo/face-boundary-walks-one-home
refs: [4078]
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

## Where it stands

The iterator is `Body::face_boundary_linked` (`live.rs`): each member
is `BoundaryMember::Isolated { vertex, point }` or
`BoundaryMember::Edge { he, half, ek, edge }`, and
`Body::face_boundary_by_loop` yields the same members a loop at a time
(for a reader that needs a cycle whole), built on
`Body::loop_members_linked`, the one spelling of a loop's member hops.

Moved onto it: `boolean/ops.rs` `face_boundary_meets`,
`faces_by_vertex`, `has_lone_vertex` (onto `face_loops_linked`);
`boolean/carrier_cross.rs` `boundary_crossing`; `boolean/finish.rs`
`pinch_site` and `section_boundary` (its `face_cycles` is gone);
`boolean/boxes.rs` `face_window_steps` (a loop at a time);
`boolean/reduce.rs` `boundary_meets_circle_only_at`;
`boolean/rim_wedge.rs` `face_boundary_arcs`; `boolean/surface_group.rs`
`unmated_boundary`; `merge_faces.rs` `outermost_survivor` (rings only,
through `loop_members_linked`) and `boundary_points`; `offset_nappe.rs`
`corner_stations`. From the `once(outer).chain(rings)` sweep:
`boolean/rest.rs` `face_witnesses`, `halves_at`; `shell.rs`
`duplicate_in_loop`; `splitting/rules.rs` `face_extent`; `movefac.rs`'s
loop hop (onto `face_loops_linked`; its walk stays, see below).

**Remaining** (the row stays open for them):

- `pcurves.rs` `face_loop_walks`: another lane is editing the file.
- `offset_together.rs` the solids walk: PR 4080 rewrites the file.

**The sweep's other hits, by disposition.** Typed or absent reads, the
torn-hop rows' to convert, each landing here when it does:
`chart_region.rs` `face_boundary_points`; `splitting/finish.rs` (four
walks); `separation.rs` `of`; `boolean/shell_witness.rs` `face_loops`;
`boolean/discard.rs` `discard_row`; `boolean/solid_contain.rs` (three);
`boolean/mod.rs` `face_boundary_points`; `boolean/ops.rs`
`boundary_edges`; `boolean/sphere_region.rs` `sphere_face_region`;
`boolean/contain.rs` `face_loops`; `props.rs` (two); `euler.rs`
`find_half_edge`. By design not links: the
validator (`validate.rs`, five), `coherence.rs` (reports the unexamined
loop), `census.rs` `face_loops`/`face_cycles` (its callers name what an
unwalkable loop means), `fixtures.rs` (counts dead loops). Loop key
lists with no walk: `euler.rs` `:4229` (rewires a site's loops),
`seqgen.rs` (three, test generator, `expect`s). Test code:
`iso.rs`, `null.rs`, `tier3_tests.rs`, `review_m0_pr7.rs`,
`review_m1_pr4.rs` (three), `boolean/contain.rs` `:1065`. Not
topology: `chart_region.rs` UV polygons (three). HOLD (D10, PR 3990):
`boolean/rest.rs` `patch_faces`, whose own logic reads the declared
REST surfaces. Single-loop walks, which the iterator's shape does not
take: `splitting/containment.rs` `cycle_steps`, `splitting/classify.rs`,
`shell.rs` (four), `boolean/contain.rs` `:981`, `chord_join.rs`,
`euler.rs` `:2837`, `boolean/rest.rs` (six outer-only walks behind its
`loop_boundary`/`cycle` helpers). `movefac.rs`'s walk: validator-shaped
asserts on the loop record run between the loop hop and the walk, and
on the whole cycle after it, so the walk stays hand-spelled.
