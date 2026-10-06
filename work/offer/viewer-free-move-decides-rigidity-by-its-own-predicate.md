---
id: viewer-free-move-decides-rigidity-by-its-own-predicate
kind: issue
title: viewer: the free-move probe admits a frame by its own rigidity test (RIGID_SLACK), not the kernel's topo::check_rigid
status: open
opened: 2026-09-30
priority: P3
cost: E
---


Found by the EDIT placement unit's fix pass (PR 3497), which made every
document door that admits a frame ask one rigidity predicate
(`Frame::admission_fault`, over `topo::check_rigid` at the linear band —
the predicate the evaluation's `TransformError::NotRigid` applies).

## What

The free-move probe admits a preview frame by its own test,
`crates/viewer/src/display.rs` `is_rigid`: columns unit and orthogonal and
the determinant one, each within `RIGID_SLACK` (`1e-9`, absolute). Its doc
argues this is a display bound nothing downstream decides geometry on, and
today that holds.

It stops holding at P3 of `docs/doc-ledger/edit-placement-spec.md`: "place where
shown" is one edit whose frame comes from the probe. That frame then meets
the edit door's predicate, which decides against the run's band (ε, K·ε)
rather than a fixed slack, so a frame the probe shows can be one the door
refuses as `non_rigid_placement`, and the two answers are about one frame.

## Repair shape

Admit a preview by the document's predicate, so a shown frame is a
placeable one; keep `DisplayFault::NonRigidFrame` as the probe's sentence.
Do it with P3, which is where the frame first crosses.
