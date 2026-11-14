---
id: two-boolean-normal-readers-absorb-a-normals-refusal-as-no-normal
kind: issue
title: Two boolean normal readers (reduce::sheets_part, slit::spline_floor) read a normal's refusal as no normal, as holding_face_normal did
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6655
---


Found by the sweep of PR `onecopy/carrier-standing-2`, which fixed the same shape in `join::holding_face_normal` (row `holding-face-normal-absorbs-a-refusal-as-no-holding-face`). Pattern: `Err(_) => (return )?Ok(None)` in `crates/topo/src/boolean`.

Each of these reads a face's outward normal, routes the `Escalated` arm, and turns every other refusal into `Ok(None)`, which its caller reads as "no normal here":
- `boolean/reduce.rs` `sheets_part`, the `normal` closure (`face_outward_normal_at`, `Err(_) => Ok(None)`): `AtConeApex`, `DegenerateTorus`, the spline arms and `OffSurface` all read as no normal.
- `boolean/slit.rs` `spline_floor`:
  - its placement (`Err(_) => return Ok(None)` after the `Escalated` arm);
  - its normal (`spline_outward_normal_at`, `Err(_) => Ok(None)`).

`NormalAtError::refusal` and `ContainError::refusal` state each arm's standing. `OffSurface`, `StaleFace` and `ArcEndClearOfCorners` are aborts that need an answer. The fix is the one `holding_face_normal` took: carry the refusal, and class it at the caller through `refusal()`, naming an abort's answer where the operand is in scope.

Other hits of the pattern, not this shape:
- `solid_contain.rs` `Err(_) => return Ok(None)` (~1512), "the honest remainder: a walk this chart cannot express", a stated remainder;
- `factored_crossing.rs` `Ok(Sign::Zero) | Err(_) => return Ok(None)` (~132), a decide folded into None, the `value-discards-read-a-refusal-as-nothing-found` class. That row is dispatched and does not list it, so it is named here for its owner.
