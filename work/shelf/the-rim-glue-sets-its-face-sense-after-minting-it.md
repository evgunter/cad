---
id: the-rim-glue-sets-its-face-sense-after-minting-it
kind: issue
title: shell's rim glue mints its face with FaceSurface::Inherit and sets the sense afterwards, where D1 (PR 3480) states the new face's sense at the mint
status: open
opened: 2026-10-11
priority: P3
---


Found by SHELL's exit walk (`docs/SHELL-EXIT-WALK.md`) while checking Ev's rulings against the code.

Ev's D1 ruling (PR 3480; `docs/DESIGN.md`, the `FaceSurface` paragraph) has the mint state the new face's bit: `FaceSurface::New { surface, sense }` and `Shared { key, sense }`. TOPO's seam note on PR 3467 (SHELL log, 2026-09-29) said the rim glue's post-hoc `set_face_sense` was gone.

It is still there. In `crates/topo/src/shell.rs`, the rim glue after `kemr` calls `body.mfkrh(parted.ring, FaceSurface::Inherit)`, which reads the ring as a hole and negates the host's bit. It then calls `body.set_face_sense(made.face, host_sense)` to put the host's bit back. The comment explains why the parted side wants the host's bit. So the face is born with the wrong bit for one step, and the correct one is restored afterwards, where D1 says the mint states it.

Owed: mint the face with the host's sense stated (an `mfkrh` that takes the bit, or an `Inherit` that means "the host's own bit" here), and drop the `set_face_sense`. Or say why this site is an exception D1 allows. Check `mfkrh`'s other callers for the same two-step.
