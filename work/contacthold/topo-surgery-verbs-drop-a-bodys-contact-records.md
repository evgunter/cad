---
id: topo-surgery-verbs-drop-a-bodys-contact-records
kind: issue
title: No non-boolean topo op carries ContactRecords in or out, so a fillet, shell or replace_face of a touching boolean result loses its declarations and refuses at rest downstream
status: parked
opened: 2026-10-02
priority: P1
cost: M
refs: [transform-and-pattern-drop-a-values-contact-records, 3856]
blocked_on: [contact-records-carry-operand-labels-into-the-at-rest-currency]
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

## Parked on the D10 hold (2026-10-08)

Carrying records through surgery is the remap the one recording door defines. (CONTACT close-out triage; CONTACT's log (`docs/doc-ledger/contact-leaves-the-tracker.md` names the SHA it is read at).)

## Re-parked on B2 (2026-10-10)

Its trigger, stage 4 B (`coincidences-are-recorded-at-one-door`, PR
4354), fired, but B left `ContactRecords` as it was (the S4-B ruling,
option (b)); the record this row is about is rewritten by B2
(`contact-records-cite-their-decision`, live on
`intent/s4-b2-records-cite`), which makes every row cite its
`Coincidence` across ~99 files. Building on the record's shape while
B2 rewrites it would collide and be built twice. Re-read it against
B2's merged record. (CONTACTHOLD orchestrator)

## Read against B2 (2026-10-10): parks on the at-rest record's shape

Still true on main: no fillet, shell, `replace_face`, split or
`transform_rigid` carries records, and the topo ops have no
records-bearing return. B2 makes the carry well-defined: map each record's
cells through the op's lineage, re-cite each survivor as
`Backing::Carried { input, record }`, and refuse any new touch. A
key-stable op (transform) can copy `wire.rs::wire_place_in_world`'s
`contacts.carried_from(0)`. But `record` addresses `ContactRecords::rows()`
order, which the open record-shape fork
(`contact-records-carry-operand-labels-into-the-at-rest-currency`, with
`curve-contact-names-one-face-…`) may renumber. Parked on that fork so
the re-citing isn't built twice. The editor-core twin is WIRE's
`transform-and-pattern-drop-a-values-contact-records`.
(CONTACTHOLD orchestrator)
