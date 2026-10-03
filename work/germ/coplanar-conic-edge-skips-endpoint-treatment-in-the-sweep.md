---
id: coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep
kind: issue
title: A conic edge lying in a plane face's plane gets no endpoint treatment in the sweep, so premise S holds for it only through its neighbours
status: closed
opened: 2026-09-28
priority: P1
cost: E
closed: 2026-09-28
branch: germ/coplanar-conic-endpoints
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
---

## What

`reduce.rs` `sweep_direction`, the conic × plane lane: when
`conic_plane_crossing_roots` returns `Ok(None)`, the lane `continue`s
with no endpoint treatment. `Ok(None)` has two causes, and one of them
is not "never meets": the PARALLEL-frame gate (`classify.rs`, the
`split_conic_plane_parallel` `Zero` arm) returns it for a conic whose
plane is parallel to the face's plane, which includes a conic lying IN
it. The code comment ("a conic that definitely never meets the plane …
Zero endpoints are impossible here") is false for that case.

A conic edge lying in `G`'s plane and running inside `G` is then neither
recorded, nor certified a miss, nor refused, at this arm — premise S of
the section certificate fails per arm. What evidences such a contact
today is the neighbours:

- a line edge of `G` crossing the conic is a pierce of `F`'s carrier
  landing on `F`'s boundary (`OnEdge`, recorded), or a sphere frontier
  refusal;
- the conic's endpoints are endpoints of other edges (a meridian seam,
  a line edge), whose endpoint rows record them;
- a planar face beside the conic and coplanar with `G` is refused
  `UndeclaredCoincidence`.

Measured (section-certificate fix pass, 2026-09-28): a cylinder with
its cap coplanar with a box's top, the box's boundary crossing the rim,
refuses `UndeclaredCoincidence`; a donut revolved with its seam
parallels at the equators, a box top in the equator plane holding an
arc of the outer equator, refuses `CurvedSectorSideUnsupported` under
∪ (and at the ∖/∩ roster). No wrong answer found. A vertex whose every
incident edge is a conic in `G`'s plane (a valence-2 vertex between two
coplanar arcs) would have none of those neighbours; no constructor in
the tree is known to build one.

## Fix

In the `Ok(None)` arm, give the endpoints the line lane's
`(Zero, Zero)` posture: each endpoint on `G`'s plane through
`vertex_on_face`. Then correct the comment, and the section
certificate's premise-S audit line for the lane.

## Closed — the parallel frame is told apart (germ/coplanar-conic-endpoints)

`conic_plane_crossing_roots` returns `ConicPlaneMeet`: `Miss` for the
definitely-never-meets graze arm, `Parallel { offset }` for the
parallel-frame gate, `Roots(..)` otherwise. `sweep_direction` decides
the offset on the band (`bool_conic_face_plane_offset`): off the plane
is a certified miss; in it, both endpoints go through `vertex_on_face`
(the line lane's `(Zero, Zero)` posture); undecided refuses
`Escalated`. The split lane takes both new arms as it took `Ok(None)`.
The comment and the section certificate's premise-S line say so.

Rows: `crates/topo/src/boolean/coplanar_conic_rows.rs` runs the sweep
on a half cylinder-wall sheet whose top rim is split at `(0, 1, 1)` (a
valence-2 vertex between two arcs, built by `split_edge`) against a
brick whose top holds it — the vertex is now recorded on the top face;
off the plane (by 0.1 and by twice the escalation threshold) nothing
is recorded; in the ambiguity band the sweep refuses naming the
offset. Mutants: the arm back to `continue` reddens the in-plane and
undecided rows; the offset decision dropped reddens the off-plane row
(at the near offset, where `contfp` places the off-plane point `In`)
and the undecided row.

The one-dimensional coincidence after the endpoints are recorded,
measured by `crates/sweep/tests/germ_coplanar_conic.rs` (every op
refuses or answers its closed form, `point_in_solid` agreeing): the
cylinder cap refuses `UndeclaredCoincidence`; the equator donut
refuses `CurvedSectorSideUnsupported` under ∪ and `CurvedPairUnsupported`
at the ∖/∩ roster; a tube whose outer wall meets itself in a circle in
the box top refuses at the join (`split_arc_window` for an arc,
`UnpairedLooseEnds` for the whole circle — evidence added to
`an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`).
Every one refused identically with the arm reverted; none answers, so
no refusal was added at the sweep.

**Answers that changed.** The in-tree constructor this item did not
know of is the revolved ball poled along an axis lying in a plane face:
its seam meridian and both poles lie in the face. The poles are now
recorded, the reduction takes the crossings path, and the join refuses
the tilted plane×sphere section:

- `cube(1) ∖ ball(0.3)` poled along `y` at `(0.5, 0.5, 1)` answered its
  closed form through the no-crossings re-cut and now refuses
  `Join(SectionInvariant)`. The row that used it
  (`r2_rim_corpus_probes.rs` `r2_a_boolean_made_rims_arcs_store_one_circle`,
  about boolean-made rims) takes the `z`-poled pip instead; the pose is
  held in `germ_coplanar_conic.rs` and filed as
  `sphere-seam-in-a-plane-face-loses-the-fallback-recut-to-the-tilted-section-refusal`
  on reach's slate.
- PR 9c's die-pips smoke shape refused `FallbackExtentUnsupported`
  (tangency) and now refuses `Join(SectionInvariant)`; re-pinned as
  `m5_pr9c_sphere_doors.rs`
  `the_die_pips_shape_stops_typed_at_its_section_roles`.
