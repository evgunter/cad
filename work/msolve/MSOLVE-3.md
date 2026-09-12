---
id: MSOLVE-3
kind: unit
title: MateFault::PatternRule carries the evaluation layer's typed refusal; the DanglingHead catch-all closes
status: closed
opened: 2026-09-05
refs: [mate-dangling-head-is-a-catch-all-that-reports-a-false-cause]
branch: msolve/3-placer-refused
closed: 2026-09-06
pr: 2081
---


## Ruling (MSOLVE orchestrator, 2026-09-05)

The proposal in
`mate-dangling-head-is-a-catch-all-that-reports-a-false-cause` is
ruled IN as S-MATE's successor: one variant carrying the evaluation
layer's typed refusal verbatim replaces `derived_offset`'s `_ =>
dangling()` arm, and the catch-all closes so no future `NodeErrorKind`
becomes a dangling head silently. The genuine `DanglingHead` causes —
an index at or beyond the count, a head outside the vocabulary — keep
their variant. Sequenced after MSOLVE-1 because the arm it replaces is
rewritten there (the offset now folds over a chain and a transform's
slots can refuse the same way a pattern's do). Spec when MSOLVE-1
merges; the issue is parked on this unit.

## Spec (2026-09-05)

`docs/MSOLVE-3-SPEC.md`. The variant is `MateFault::PlacerRefused`
(the chain has transforms now, not only patterns); slot evaluation in
the solve moves onto `eval_slots`, the evaluation's own door, so the
carried refusal is byte-identical to the node's. Rides the placement
constructor finding (`placement-frame-constructor-refuses-on-the-
frame-not-the-axis`) as a rider. Dispatches after MSOLVE-2 (both edit
`derived_offset`).

## Closed (2026-09-06, PR 2081)

Landed: `MateFault::PlacerRefused { mate, side, placer, error }`
carrying the evaluation layer's own `NodeErrorKind` verbatim (an `Arc`
newtype, `NodeRefusal`), replacing every arm of `derived_offset` that
relabeled a typed refusal as a dangling head; slot evaluation in the
solve through `eval_slots` and the wiring's own slot readers;
`DanglingHead` keeps its two genuine causes (the walk's stop, an index
at or beyond the count); the viewer's row and the Python projection
carry the cause; the rider — `Frame::rotate_then_translate` decides
its axis through the direction door under `PLACEMENT_AXIS_ROLE` and
`SetPlacement` refuses in the axis's voice, constructed only at that
site. Measured: the carried kinds are byte-identical to the placer's
own on eight review probes; the rider reaches no viewer call site.
Reviews: correctness APPROVE-WITH-FIXES (the datum's refusal was
attributed to the pattern; a blanket `From<NodeErrorKind>` for the
edit error reopened the catch-all one door over; no Python row), style
no MAJOR. Residue with files: the four-workspace build hazard and the
`names/{flush,select}.rs` `map_err(|_| ..)` discards (`work/issues/`).
