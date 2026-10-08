---
id: split-refusal-detector-names-a-shared-parameter-coincidence
kind: issue
title: Split's undeclared-pinch refusal could name, as a detector finding, that the coincidence holds for every value of a shared parameter, so the user declares it with the reason in hand
status: parked
opened: 2026-10-03
priority: P3
cost: M
blocked_on: [intent-stage4-is-built]
---


From PR 3960 (Ev, 2026-10-03). Once split refuses an undeclared pinch, a
detector in the shape of `editor-core/src/names/flush.rs` could report that
the tip's coordinate and the plane are the same `Expr`, in one frame, for
every parameter value. It would offer the Split node's declaration with
that reason attached.

Ev's line: a shared parameter never declares on its own. The user still
makes the declare call, under #286's no-fusion rule.

Unmeasured: carrying the comparison through the sketch frame and the
placement transforms. Waits for the split declaration seat to be built.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: this is the unproven-coincidence lint's job (report with the edit that makes it one construction); the split refusal and declaration it hangs on retire at stage 4. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)
