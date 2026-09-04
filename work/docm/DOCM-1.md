---
id: DOCM-1
kind: unit
title: A derived sketch frame: Datum::FaceFrame, the sense beside the pose, and the carrier-kind read (DM1, DM1a, DM1b, DM2)
status: closed
opened: 2026-09-04
closed: 2026-09-04
pr: 1829
branch: docm/1-face-frame
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

## Closed (2026-09-04)

Merged as PR 1829 (three phases: a stop at PP6's f64 fence, ruled
option A and amended on main; the build under the amendment; the v6
dual's union fix pass). Record: the DOCM-1 row in
`docs/MODEL-AB-LOG.md` (ordinal 1802, sample #128) and the MERGED
entry in `work/docm/log.md`. The spec is deleted per the ledger
(`docs/DOC-LEDGER.md`). Residue: the derived frame's placement does
not certify on the symbolic lane under a widened upstream parameter —
diagnosed to the kernel's symbolic budget and filed for M10 as
`derived-frame-placement-freezes-on-the-symbolic-lane`; the Python
surface is `LIB-B-FACE-FRAME`; the chrome is CHROME's two items.
