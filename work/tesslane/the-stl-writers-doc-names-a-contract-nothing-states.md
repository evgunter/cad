---
id: the-stl-writers-doc-names-a-contract-nothing-states
kind: issue
title: the STL writer's module doc cites a mesh non-degeneracy contract nothing states, and a cone-apex case that no longer exists
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by all three designers of the flat-triangle fork (`docs/DESIGN-FORK-LOG.md` row 269).

`crates/stl/src/lib.rs`'s module docs say the writer refuses a degenerate triangle per "the mesh's non-degeneracy contract". No mesh doc states one (`check_mesh`'s doc says zero-area slivers pass). The refusal itself is of an exactly zero cross product (`stl::facets`, `len == 0.0`), so a triangle whose cross product is rounding-sized is written with a normal that is rounding noise: measured on `sweep`'s `tilted_sphere_pair` union at δ 0.04, cross length about 2e-17, the written normal 13° off the sphere's. The same docs name as a "known live case" a cone's apex fan emitting triangles collinear along a generator, which the cone's development removed (PR 6950).

Owed: the doc cites the promise the mesh makes once `[mesh.winding]` is in force, and loses the stale case. Whether the writer's exact-zero test stays as its own backstop is that unit's to say.
