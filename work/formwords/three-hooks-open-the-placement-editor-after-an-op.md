---
id: three-hooks-open-the-placement-editor-after-an-op
kind: issue
title: Three hooks open the placement editor after an op: Tools::answered, held_target and datum_made
status: open
opened: 2026-11-23
priority: P4
cost: C
---

Three `Tools` hooks open or settle the placement editor after the
session answers an op, each keyed by a different op shape:

- `Tools::answered` (`crates/viewer/src/tools.rs:833`): a landed
  `SessionOp::Place` opens the editor on the placement it made, and
  the editor's own `SetBundle` landing or refused settles its draft.
- `Tools::held_target` (`crates/viewer/src/tools.rs:897`): a door's
  `Refusal::TargetHeld` opens the editor on the copy with the target
  set.
- `Tools::datum_made` (`crates/viewer/src/tools.rs:869`, PR 7535): a
  committed `SessionOp::AddDatum` opens the editor on the datum with
  one row of the kind `Tools::awaiting_datum` holds.

`crates/viewer/src/app.rs` calls the first and third back to back after
each committed op. One door, "the session answered this op", that
dispatches on the op would leave one place to add the next such hook.
Out of scope for 7535, which only added the third.
