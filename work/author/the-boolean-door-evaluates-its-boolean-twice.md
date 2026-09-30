---
id: the-boolean-door-evaluates-its-boolean-twice
kind: issue
title: The boolean door evaluates the boolean it commits twice, once on the frame's thread
status: open
opened: 2026-09-30
priority: P2
cost: M
refs: [addboolean-doc-names-a-vocabulary-that-does-not-exist]
---

Filed by AUTH-9 as the residue of its own design call. Re-priced at
the fix pass from P3 to P2, for the second case below.

## What

`DocSession::add_boolean` (`crates/viewer/src/session.rs`) judges the
boolean before recording it. It evaluates the staged document through
`evalseam::evaluate_beside`, outside the seam, so that a boolean
refusing an undeclared contact is refused at the door with its offer
instead of committed. That evaluation runs synchronously, on the
thread that performs the frame's ops, with a `CancelToken` nothing can
set.

Its cost depends on the memo (`DocSession::memo_under`: the landed run,
used only if it resolved through the same seam):

1. **A current landed run.** Only the cone of what the run added is
   computed: the boolean, and the `Declare` when there is one. The
   seam then evaluates the recorded document again, so every committed
   boolean is evaluated twice. "Milliseconds" was measured on one
   scene only, the block-and-boss union. A boolean of curved or
   many-faced operands costs what its evaluation costs, twice, and the
   first time blocks the frame.
2. **Nothing has landed**, e.g. just after an open with the first
   evaluation still running and the operands picked from the tree. The
   memo is `None` and the judge evaluates the WHOLE document
   synchronously. This case is why the row is P2.
3. **A stale landed run** (an edit still evaluating). The judge also
   recomputes everything that edit changed, synchronously.

## The cheap cure, and why it was not taken

`add_duplicate` has the same need and answers it by refusing:
`DuplicateFault::NotLanded` when nothing has landed and
`DuplicateFault::Stale` while `busy()`. Doing the same here would cure
cases 2 and 3 in a few lines. It would also refuse every boolean,
including the many with no contact at all, whenever any evaluation is
running, and the only thing the author could do about that refusal is
wait. The duplicate refuses because it needs the value itself. The
boolean needs it only for this judge. Refusing only when nothing has
landed (case 2) is the narrower version, and it needs a new refusal
arm and its wording. So it is a choice between a refusal and a
blocking pause, which is this row's to make rather than a fix-pass
drive-by.

## The fixes that remove the double run

(1) Hand the judged `Evaluation` to the seam as the landed run for the
recorded document, so the seam's run is skipped. (2) Move the judge
into the seam as a request whose answer the session records or
refuses. Both touch `evalseam`'s landing contract. Either one, with
cancellation, also answers case 2.
