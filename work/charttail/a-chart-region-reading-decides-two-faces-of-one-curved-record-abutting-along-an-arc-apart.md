---
id: a-chart-region-reading-decides-two-faces-of-one-curved-record-abutting-along-an-arc-apart
kind: issue
title: A chart-region reading that decides two faces of one curved record abutting along a boundary arc apart
status: open
opened: 2026-11-03
---


A boolean leaves each glued one-carrier pair's faces on one surface
record (`topo::Body::unify_surfaces`, DESIGN `[coincidence.door]`), so
the census's one-key conformal arm (`census.rs` `sweep_conformal_patches`,
through `one_record_overlap`) reads two opposed faces of one cylinder
record that abut along their rim: a boss's wall and its hole's wall. The
region door (`chart_region.rs`, `chart_overlap`) refuses the pair
`TouchingBoundary`. The faces' own fallback reading
(`census.rs` `face_interiors_meet`) answers `true` for it, which is not a
certificate (it reads two crossings' cones as shared interior). So the
pair is `CensusUnsupported`, and no lane decides it apart.

Witnesses, each pinned to admit exactly that refusal through
`abutting_walls`:
- `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
  `a_boss_over_a_hole_by_pose` (the ∪ rows that read `Reads::Unrecorded`
  or `Reads::Undecided`);
- the same file's
  `a_hole_sharing_the_rim_along_an_arc_then_leaving_it_tangent`.

Wanted: a reading in the shared chart that decides two such faces apart
where they share only a boundary arc. With it, the two pins drop
`abutting_walls`.
