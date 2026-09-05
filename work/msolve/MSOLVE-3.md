---
id: MSOLVE-3
kind: unit
title: MateFault::PatternRule carries the evaluation layer's typed refusal; the DanglingHead catch-all closes
status: open
opened: 2026-09-05
refs: [mate-dangling-head-is-a-catch-all-that-reports-a-false-cause]
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
