---
id: kept-boundary-undecided-has-no-live-fixture
kind: issue
title: MergeCoplanarError::KeptBoundaryUndecided has no live fixture: the merge's neighbour scan refuses an in-band planar dihedral first
status: open
opened: 2026-10-09
priority: P3
cost: E
---


`MergeCoplanarError::KeptBoundaryUndecided` is raised where a kept
face's boundary edge, after an absorption, has a dihedral the
describer cannot decide (`crates/topo/src/merge_faces.rs`
`of_kept_boundary`). Its live fixture was
`wall_beside_a_leaning_neighbour` (`crates/topo/src/merge_faces_kept_rows.rs`):
a neighbour leaning off the absorbed face by an in-band angle.

Since stage 4 PR E the merge reads every adjacent pair with the
ladder, declared or not (`Body::faces_continue`), so that lean
refuses at the merge's own scan
(`MergeDecision::Neighbours(PlaneRung::Parallel)`,
`an_in_band_lean_between_neighbours_refuses_at_the_scan`) before any
face is absorbed. The kept-boundary refusal's wording is still pinned
on synthesized values (`boolean::ops`'
`a_kept_boundary_refusal_comes_back_as_the_booleans_own`), but no body
reaches it. A curved neighbour near-tangent to a kept planar face —
whose carrier pair the ladder decides distinct while the describer's
dihedral at the edge stays in band — is the likely fixture.
