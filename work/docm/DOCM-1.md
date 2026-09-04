---
id: DOCM-1
kind: unit
title: A derived sketch frame: Datum::FaceFrame, the sense beside the pose, and the carrier-kind read (DM1, DM1a, DM1b, DM2)
status: spec
opened: 2026-09-04
---


## Spec

`docs/DOCM-REFERENCES-DESIGN.md` DM1, DM1a, DM1b, DM2. One unit,
kernel-side only (`crates/topo/src/readback.rs`,
`crates/editor-core/src/names/interrogate.rs`, `node.rs`, `eval/wire.rs`,
the persist wire mirror): `Datum::FaceFrame { at, face, spin }` evaluating
to the frame DM1 states; `face_pose` / `face_frame` returning the face's
orientation sense beside the `Pose`; a carrier-kind read door in
`readback` and its `StableName` twin; rule 1's text tightened to numeric
predicates; the typed non-planar refusal. The chrome that offers it is
CHROME's (`add-profile-mints-no-frame`,
`add-profile-placement-on-picked-face-frame`, both re-homed there
2026-09-04). Python surface is LIB's, filed at merge.
