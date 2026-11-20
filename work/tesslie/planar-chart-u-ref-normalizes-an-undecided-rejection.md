---
id: planar-chart-u-ref-normalizes-an-undecided-rejection
kind: issue
title: the planar chart frame's u_ref normalizes an undecided rejection
status: open
opened: 2026-10-01
priority: P2
cost: E
---


## What

`crates/mesh/src/planar.rs` (the planar chart frame, ~:382) takes
`u_ref = far.reject_from(normal).normalize()`, where `far` is the
boundary vertex farthest from the frame origin. The rejection's
length is not decided: a face whose vertices all lie within the band
of the origin, or whose farthest vertex sits along the normal, gives
a residual that is zero (poison through `reject_from`'s 0/0) or in
the band (a definite direction made of rounding).

`f64` only, and the tessellator may refuse such faces earlier; this
filing did not build a fixture, so the reachability is unmeasured.

## Shape

The Gram–Schmidt step, decided: `UnitVec3::new` on the rejection (or
`RigidMotion::from_aim_and_reference` against the normal) under a
tessellation K name, refusing typed. Found by the
`linalg/decided-not-minted` sweep's second pass, which looked for
`reject_from(…).normalize()` after the first pattern
(`- a * a.dot(x)).normalize()`) could not see the helper spelling.

## Moved to TESSWALK (what the walk and the lanes decide: structural premises, and readings taken off undecided numbers) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

Moved from TESSWALK to TESSLIE on 2026-11-20, by `git mv`, when TESSWALK merged into TESSLIE: TESSWALK held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one TESSLIE carries.
