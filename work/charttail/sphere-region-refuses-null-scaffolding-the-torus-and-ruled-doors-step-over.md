---
id: sphere-region-refuses-null-scaffolding-the-torus-and-ruled-doors-step-over
kind: issue
title: The sphere region's gather refuses a face holding null scaffolding, which the torus and ruled gathers step over
status: open
priority: P1
cost: E
opened: 2026-10-24
---

## What

The three region doors now gather their boundary through one function,
`boundary` in `crates/topo/src/boolean/region.rs`. They still differ on one
rule, which the gather carries as its `Scaffolding` argument:

- `torus_face_region` and `ruled_face_region` pass `Scaffolding::Skipped`.
  They step over null scaffolding: an edge with no certified curve whose
  two ends are one point.
- `sphere_face_region` (`crates/topo/src/boolean/sphere_region.rs`) passes
  `Scaffolding::Refused`. A face holding null scaffolding has no region,
  as if the edge were an uncertified gap.

Before the gather was shared, each door had its own copy. The sphere copy
never had the null-scaffolding skip, while the torus and ruled copies did
(PR 5664's `ruled_face_region`, and the torus door before it).

## Why it matters

A sphere face holding null scaffolding refuses where a torus or ruled face
in the same state is read:

- `point_in_solid` answers `PointInSolidError::PartialSphereFace`
  (`solid_contain.rs`, `face_geo_arms`'s sphere arm).
- The pierce arm answers `CurvedPlacement::Trim(None)` (`contain.rs`,
  `sphere_face_containment`).

The refactor that shared the gather kept the difference rather than pick
one, because picking would have moved the sphere door's answers.

## Owed

Decide whether the sphere door steps over null scaffolding too:

- If it does, pass `Scaffolding::Skipped` and pin the answer with a row, a
  sphere face holding a null edge (the ruled rows' Euler-built wall with a
  strut and a null edge is the template). Then retire `Scaffolding`.
- If there is a reason it must not, say so at `sphere_face_region`.


## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: one region gather, two rules for null scaffolding: a special case that should be handled uniformly, P1 by `work/README.md` Priority.
- Moved to CHARTTAIL (`work/charttail/`) by `git mv`, id and body unchanged: which holds CHART's rows since CHART folded in today: chart readings and the region doors. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
