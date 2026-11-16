---
id: holding-face-normal-absorbs-a-refusal-as-no-holding-face
kind: issue
title: join::holding_face_normal returns Ok(None) for every placement and non-OffSurface normal refusal, so the chord ladder reads a refusal as no holding face
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6648
branch: onecopy/carrier-standing-2
---


Found by the delta review of PR 6576 (`onecopy/boolean-standing`), finding N5. The boolean-standing lane found it too costly to fix in that PR.

`topo::join::holding_face_normal` (join.rs ~5757, ~5773) turns every placement error, and every `NormalAtError` other than `OffSurface`, into `Ok(None)`. The chord ladder (join.rs ~5681-5684) reads `Ok(None)` as Passes. A refusal is therefore absorbed as "no holding face", including:
- `DegenerateTorus`, which is now an Abort (6576);
- `AtConeApex`;
- `ContainError` aborts.

The fix is to return the refusal, so that the ladder classes it through `refusal()`.

The obstacle: an aborting refusal needs a `BooleanError` answer, and `seam_germ_side` has no operand in scope to build `PointInFaceRefused` or `DegenerateTorus` from. Possible ways round it:
- carry the operand into the reader;
- give the ladder an Abort payload that wraps the raw refusal;
- refuse at the caller that holds the operand.

**Pin.** A seam germ whose holding face is a degenerate torus, or a cone face at its apex, must refuse, not pass.
