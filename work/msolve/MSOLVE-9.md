---
id: MSOLVE-9
kind: unit
title: A mate frame that names a face of the part and resolves at evaluation through the reach road; A11's inputs sentence revised
status: open
opened: 2026-09-19
branch: msolve/9-from-face
---


Spec: `docs/MSOLVE-9-SPEC.md`. Ev's ruling (F) on `[ev]` PR 2256:
`MateFrame` gains a `FromFace { face, reference }` arm resolved at
evaluation through `face_pose`, on MSOLVE-6's reach road (the cached
part's own product and name table answer it in part coordinates); the
solve runs over resolved frames and its algorithm is unchanged. The
unit revises A11 rule 5's inputs sentence in `ASSEMBLY.md` — the
wording rides this unit's `[ev]` PR and waits for Ev; the unit
dispatches after that merge, after MSOLVE-8 (it rides its frame
witness). LIB's façade and Python half follows by announcement.
