---
id: a-rigid-map-can-still-refuse-a-sound-approx-face-at-its-edges-or-meters
kind: issue
title: a rigid map can still refuse a sound Approx face: a meter that goes in-band in the new frame, or an edge that rode the fit net a re-fit replaced
status: open
opened: 2026-09-28
priority: P3
cost: M
design: true
---


## Found (PR 3332's review, ENCL rigid-map headroom lane, 2026-09-28)

**Inferred from the code, not reproduced.** PR 3332 made
`topo::transform`'s `map_approx` (`crates/topo/src/transform.rs:438`)
answer a LIMB refusal of a sound face's image by re-fitting the mapped
description (`lane.mint`, `transform.rs:471`). Two refusals of a sound
face survive it.

### 1. Meters

`certify_offset_at` re-runs the door meters on the MAPPED base before
any limb (`crates/geom-brep/src/offset_fit.rs:1203`, `meter_patch`,
`crates/geom-brep/src/offset_meters.rs:740`). The regularity floor and
the curvature headroom are assembled from ambient-frame control-hull
boxes, so they move under a rotation the way `hull_sup` does. A face
whose meter reading sits just outside the band where it was minted can
land in-band after a rotation and refuse
`ApproxRecertify { Meter }`. The re-fit cannot answer it: the fit door
runs the same meters on the same mapped base (`offset_fit.rs:1004`).
`map_approx` passes meter refusals on verbatim for that reason.

### 2. Edges that ride the replaced fit net

A re-fit replaces the fit net. The body's edges are then mapped
control-point-wise from their OLD carriers (`transform.rs:658`,
`map_carrier`) and re-certified against the new surface
(`transform.rs:709`, `EdgeCurve::certify_via`), and the pcurve caches
are re-minted against it (`transform.rs:735`). An edge whose carrier or
chart image was cut from the old fit's bits — the iso rows
`replace_face` builds from `approx.fit()`
(`crates/topo/src/replace_face.rs:1632`, `iso_boundary_row`), a
`Chart` image on the fit — describes the old net, not the new one, and
can refuse `TransformError::Certify { edge }` or `Pcurve`.

### Latent

No body the tree can move today carries a curved `Approx` face with
edges. The one fixture that does move, `sweep`'s `box_with_approx_cap`
(`crates/sweep/tests/common/approx.rs:247`), offsets a planar patch, so
its fit is exact and its image never refuses. Lofted bodies with
`Approx` walls refuse the transform earlier, at their NURBS seam
carriers.

## What is open (design)

- Meters: whether a rigid map should re-classify a sound face's meters
  at all, or carry the mint's verdict. Carrying it is the witness-style
  answer; re-deriving it is O5's posture.
- Edges: whether a re-fit should re-derive the face's boundary edges
  from the new net (as `replace_face` does at mint), or whether the
  map should prefer a re-fit that keeps the boundary rows. Either is a
  change to what the transform door promises about edge identity.
