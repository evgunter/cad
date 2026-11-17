---
id: face-geo-restates-carrier-desc-curved-arms
kind: issue
title: solid_contain's FaceGeo restates CarrierDesc's curved arms field by field
status: parked
opened: 2026-11-13
priority: P1
cost: H
design: true
blocked_on: [analytic-surfaces-spell-their-frame-field-by-field]
---


## Finding

`crates/topo/src/solid_contain.rs` `enum FaceGeo` is the
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

## Moved to ONECOPY (2026-11-17)

Moved from ONEDOOR by `git mv`, id and body unchanged, when ONEDOOR went
`ready` (Ev, in chat). The fold this row owes, a named description per
curved kind in `geom_brep::CarrierDesc`, is the geom-brep side of the
cause ONECOPY's `analytic-surfaces-spell-their-frame-field-by-field`
settled for `geom::Surface` (struct-like variants are not types, so every
reader memoizes the match in a struct of its own; its "Decided" section,
cause 1). Parked on that row's stage 3: per-kind `CarrierDesc` types
built before `geom`'s kind structs land would be reshaped by them, and
whether they hold `geom`'s structs or mirror them is a question for
that stage's design. `design: true` stands. (GREEN, at Ev's request)
