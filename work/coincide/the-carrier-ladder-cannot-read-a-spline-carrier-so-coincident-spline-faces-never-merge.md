---
id: the-carrier-ladder-cannot-read-a-spline-carrier-so-coincident-spline-faces-never-merge
kind: issue
title: the carrier-pair ladder reads analytic carriers only, so two coincident spline faces (a degree-1 loft's coplanar slabs) are never merged into a maximal face
status: open
opened: 2026-10-28
priority: P2
cost: M
refs: [loft-of-three-or-more-sections-at-degree-one-is-unmeshable]
---


## What

The boolean output stage and the finishing join merge two adjacent faces on a Zero verdict between their carriers (`topo::boolean::carrier_pair_relation`). That ladder has a description only for analytic carriers. A face on a NURBS surface reads `PairUnread::OutsideInventory` and is never compared, so it is never merged.

DESIGN [topology]'s "maximal faces" therefore holds only for analytic carriers.

## Witness

A loft at v-degree 1 through three or more translated copies of a polygon. The loft cuts its walls at each section (PR 5923, building Decision 1 of `[ev]` PR 5913), and the slabs of one wall lie on one plane. Each slab is a separate degree-1 NURBS face, and no join merges them. `crates/sweep/tests/degree1_loft_slabs.rs` `the_output_stage_merge_keeps_every_slab_wall` pins today's answer as this gap.

A two-section degree-1 loft's flat wall is the same case with one face: a plane that no carrier reading recognises as a plane.

## Ruling

Ev, on PR 5913 (2026-10-28, 1b), asked how this case differs from the coplanar faces the kernel does merge. The answer is that it does not: the construction cuts at each section, and the ordinary join should merge where the carriers coincide. What is missing is a carrier description, a canonical form, for spline carriers that the ladder can compare. Recognising a planar or coincident net is exact for a degree-1 net through coplanar control points.

## Owed

The ladder reads spline carriers: at least a degree-1 net whose control points are coplanar, and ideally two spline carriers that are one surface. The join then merges the slabs. Flip the pinned row, and re-baseline the slab names that merge.

Filed by the TESS orchestrator.
