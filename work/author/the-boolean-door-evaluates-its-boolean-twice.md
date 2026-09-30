---
id: the-boolean-door-evaluates-its-boolean-twice
kind: issue
title: The boolean door evaluates the boolean it commits twice, once on the frame's thread
status: open
opened: 2026-09-30
priority: P3
cost: M
refs: [addboolean-doc-names-a-vocabulary-that-does-not-exist]
---


Filed by AUTH-9 as the residue of its own design call.

## What

`DocSession::add_boolean` (`crates/viewer/src/session.rs`) judges the
boolean before recording it: it evaluates the staged document through
`session::probe::evaluate_with`, outside the seam, so that a boolean
refusing an undeclared contact can be refused at the door with its
offer instead of committed. Every boolean the tool commits is
therefore evaluated twice: once there, synchronously on the thread
that performs the frame's ops and with a `CancelToken` nothing can
set, and once more by the seam after `record_run` submits the
document.

The memo keeps the first run to the cone of what the run added when
the landed evaluation is current. When it is not (an edit still
evaluating), the judge also recomputes everything that edit changed,
still synchronously.

## Why it was left

The judge has to see the boolean's own result before anything is
recorded, because a declaration cannot be attached to a committed
node, so some evaluation has to precede the commit. The cheaper
shapes are (1) hand the judged `Evaluation` to the seam as the landed
run for the recorded document, so the seam's run is skipped, or (2)
move the judge into the seam as a request whose answer the session
records or refuses. Both touch `evalseam`'s landing contract, which
is not this unit's ground, and neither changes what the author sees
on the ordinary block-and-boss scale, where the boolean is
milliseconds.
