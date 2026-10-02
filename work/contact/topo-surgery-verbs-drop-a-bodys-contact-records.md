---
id: topo-surgery-verbs-drop-a-bodys-contact-records
kind: issue
title: No non-boolean topo op carries ContactRecords in or out, so a fillet, shell or replace_face of a touching boolean result loses its declarations and refuses at rest downstream
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [transform-and-pattern-drop-a-values-contact-records, split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
---


## What

A grep by the further designer on TQUERY's pinch-contacts fork found
no non-boolean kernel op that reads or writes `ContactRecords`:
- fillet, shell, `replace_face`, transform, merge and split all drop
  them.

So any touching boolean result (a declared corner kiss, for one) that
one of those verbs then edits comes back without its declarations, and
the at-rest pseudomanifold census refuses it as `UndeclaredContact`
although nothing about the touch changed. WIRE's
`transform-and-pattern-drop-a-values-contact-records` is the
editor-core face of the same gap.

This applies whichever way TQUERY's pinch question (PR 3813) is
answered, because cross-operand records can never become shared keys.
Measure first: one fillet or `replace_face` on a declared-touching
boolean result, then the at-rest gate.
