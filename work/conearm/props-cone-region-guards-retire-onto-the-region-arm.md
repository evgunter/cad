---
id: props-cone-region-guards-retire-onto-the-region-arm
kind: issue
title: props' cone and cylinder region guards retire onto tier 3's region arm
status: open
opened: 2026-10-31
priority: P3
cost: M
refs: [4822, cone-holes-outside-their-face-are-subtracted-silently, validate-tier3-curved-boundary-containment]
---

## What

By Ev's decision on PR 4822, tier 3 owns "a face's loops bound one
region" on every surface kind, and the closed-form lanes read their loops
as such a boundary rather than half-deciding it. Tier 3's region arm now
refuses a curved face's surface cycle off the face's side of another
(`crates/topo/src/one_region.rs`; rows in
`crates/sweep/tests/one_region_rows.rs`, the cone probes P09, P47 and P49
among them).

So props' own region guards follow, per PR 4822's list:

- the cone's `HolesExceedFace` (`require_cone_holes`,
  `crates/geom-brep/src/props/curved.rs`) and the cylinder's
  (`cylinder_holes`, `require_holes_against`);
- the contractibility, vertex-distinct and once-round guards those lanes
  keep on a hole.

Each retires where the arm covers the class it guards: a hole outside its
face, a hole inside another, a hole whose winding contradicts its role.
What the arm does not read (its residue,
`work/restcurve/the-region-arm-is-silent-on-spline-faces-and-off-admits-edges.md`)
keeps its guard until it does.

`cone-holes-outside-their-face-are-subtracted-silently` is answered at
tier 3 by the same arm; its lane-side half is this row. Filed by
RESTCURVE's region-arm lane.
