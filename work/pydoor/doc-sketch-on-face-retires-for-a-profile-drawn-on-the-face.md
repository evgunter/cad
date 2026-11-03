---
id: doc-sketch-on-face-retires-for-a-profile-drawn-on-the-face
kind: unit
title: Doc.sketch_on_face retires: a sketch on a face is a profile drawn on the face's frame
status: open
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face]
parent: on-the-picked-face-makes-a-profile-and-extrudes-it-in-one-checkbox
---


Cut by the VSEAM orchestrator (2026-11-02) from
`work/vseam/on-the-picked-face-makes-a-profile-and-extrudes-it-in-one-checkbox`.
That row's dated "ruled" section is the spec (Ev, `[ev]` PR 6298).

Blocked on `work/doctail/a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face`,
which provides the face frame, the drawn-on-face door and the signed
depth.

## The build

- **Retire `Doc.sketch_on_face` and `SketchedOnFace`** from pncad
  (`crates/pncad/src/document.rs` re-exports them) and from pncad-py
  (`py/doc.rs`, `pncad.pyi`). Their tests go with them:
  `test_face_frame.py`, the binding census, and the ty fixtures.
- **Expose the replacements to Python**: the face-frame pose (face, cell,
  optional spin), drawing a profile on it, and a signed `extrude`. Then
  a boss is profile-on-face, extrude +d, union, and a pocket is the
  same sketch with extrude −d and subtract. Port
  `test_face_frame.py`'s cases to that spelling, so that what they pin
  survives.
- Do not delete editor-core's fused act here: the viewer still calls it
  until `work/vseam/on-the-picked-face-draws-the-profile-there` lands.

- 2026-11-03 — moved from LIB to PYDOOR by `git mv` when LIB closed, id and body unchanged (LIB had three P1 rows left behind 124 closed ones; its library-door rows went to PYDOOR, opened for them, and its tag, stub, census and projection rows to BIND; Ev, in chat).
