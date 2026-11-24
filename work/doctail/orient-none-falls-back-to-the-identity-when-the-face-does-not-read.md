---
id: orient-none-falls-back-to-the-identity-when-the-face-does-not-read
kind: issue
title: draw_on_face with orient=None writes the identity orientation silently when the face does not read, and the refusal that follows names an orientation, not the missing read
status: open
opened: 2026-11-22
priority: P3
cost: E
---

Found by VSEAM's review of PR 7465 (2026-11-22). Filed by the VSEAM orchestrator.

The trigger is `Doc.draw_on_face(..., orient=None)` without `resolver=`, on the bottom face of an instance's copy. `crates/editor-core/src/edit.rs` `chart_on_face` / `nearest_to_face` cannot read the face, and falls back silently to the identity orientation. Evaluation then refuses `face_chart_backwards`, whose recourse names an orientation to write and never mentions that the face could not be read.

The reviewer's probe: the same call with `resolver=ws` stands the boss at z −0.1…0.

This conflicts with fail-loud. `OnFace`'s "nearest" promises an orientation read off the face; when the read fails, the door should say so (for example, `FaceUnread` with the read's own fault) rather than write a value nobody chose. The viewer's on-face form goes through the same door, so it is affected the same way.
