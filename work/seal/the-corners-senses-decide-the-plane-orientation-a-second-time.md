---
id: the-corners-senses-decide-the-plane-orientation-a-second-time
kind: issue
title: topo: plane_eq::senses decides bool_plane_orient a second time on a sign-only decide
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by PR 6634's review (SEAL, `carrier-readings-keep-the-margins-they-decided-on`).

## What

`plane_eq::senses` (`crates/topo/src/boolean/plane_eq.rs`) decides `bool_plane_orient` (the normals' cosine, levered at the corner's arm) with a sign-only `decide`. Its callers are the corner sites:
- `vtxfac.rs`, twice;
- `recl.rs`, twice: once for the `unglued_corner` arm and once again for the `Escalated` arm of the same `carrier_eq_reading` match.

On a pair whose plane ladder already ran its orientation rung, this is a second decision of the same row on the same data, each one logged.

## Shape

The ladder's rung reports the orientation it decided (it already rides in `CarrierVerdict::One::relation` and `CarrierEqError::Undecided::relation`). A corner reads it from there, or decides it once per corner and reuses it. Step 4 (`a-corner-reads-only-its-own-sectors`) rewrites these readers and should take this.
