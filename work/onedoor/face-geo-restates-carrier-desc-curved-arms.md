---
id: face-geo-restates-carrier-desc-curved-arms
kind: issue
title: solid_contain's FaceGeo restates CarrierDesc's curved arms field by field
status: open
opened: 2026-11-13
---


## Finding

`crates/topo/src/boolean/solid_contain.rs` `enum FaceGeo` is the
containment ray's per-face description. Its plane arm holds the
`geom_brep::PlaneDesc` that `face_normal::plane_desc_of` answers. Its
curved arms (`Cylinder`, `Cone`, `Sphere`, `SpherePatch`, `Torus`,
`TorusPatch`) each restate a `geom_brep::CarrierDesc` arm field by field:
the carrier's centre or axis and its radii, plus the face's `sense`
under a second name (`outward` there). Beside those fields they carry
the ray's own state (a face region, a closed group's representative).
The partial arms carry a reach and no sense.

S173 did not fold them. `CarrierDesc`'s curved arms are inline struct
variants, not named per-kind types the way `Plane(PlaneDesc)` now is.
So a `FaceGeo` arm could hold only the whole enum, and every reader would
re-match a kind its constructor (`face_geo_arms`) has just matched. The
fold that would remove the restatement gives `CarrierDesc` a named
description per curved kind in `geom-brep` (`carrier_eq.rs`), which
`FaceGeo`'s arms then hold beside their ray state. That reshapes a
geom-brep type every carrier-ladder arm reads, which is more than a move.
