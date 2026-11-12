---
id: a-chart-region-reading-decides-two-faces-of-one-curved-record-abutting-along-an-arc-apart
kind: issue
title: A chart-region reading that decides two faces of one curved record abutting along a boundary arc apart
status: closed
opened: 2026-11-03
closed: 2026-11-04
---


A boolean leaves each glued one-carrier pair's faces on one surface
record (`topo::Body::unify_surfaces`, DESIGN `[coincidence.door]`), so
the census's one-key conformal arm (`census.rs` `sweep_conformal_patches`,
through `one_record_overlap`) reads two opposed faces of one cylinder
record that abut along their rim: a boss's wall and its hole's wall. The
region door (`chart_region.rs`, `chart_overlap`) refuses the pair
`TouchingBoundary`, or `SeamBranch` where the record's seam crosses the
rim (which one depends on the pose). The faces' own fallback reading
(`census.rs` `face_interiors_meet`) answered `true` for it, so the pair
was `CensusUnsupported`.

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

## Closed

2026-11-04, on branch `onedoor/one-surface-record`: the premise was wrong.
The fallback's `true` was not a lens or crossing artefact. The reading
took every pair as facing the same way and read the opposed face's cone
as its complement. It now takes the two faces' facing (`census.rs`
`Facing`) and reverses the second cone where they oppose. It then decides
the abutting walls apart, so no chart-region capability is wanted for
them, and both closed-conic rows drop `abutting_walls`.
