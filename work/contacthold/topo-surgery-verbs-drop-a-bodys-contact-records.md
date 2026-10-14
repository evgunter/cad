---
id: topo-surgery-verbs-drop-a-bodys-contact-records
kind: issue
title: No non-boolean topo op carries ContactRecords in or out, so a fillet, shell or replace_face of a touching boolean result loses its declarations and refuses at rest downstream
status: closed
opened: 2026-10-02
priority: P1
cost: M
refs: [transform-and-pattern-drop-a-values-contact-records, 3856]
branch: contacthold/surgery-carries-records
pr: 4694
closed: 2026-10-11
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

## Built (2026-10-11)

Measured first. A corner-kiss union run through `transform_rigid`,
`shell`, `split` (the side that keeps the kiss), `replace_face_offset`
or `merge_coplanar_faces` was refused by `validate_pseudomanifold` with
no records, `UndeclaredContact` at the kiss vertex pair.

`ContactRecords::carried_through` maps each record's cells through an
op's lineage and re-cites it `Backing::Carried { input: 0, record }`.
It refuses a cell the op removed without naming a replacement
(`CarryRefusal::Unmapped`), and a pair that comes out incident
(`CarryRefusal::Pair`). Each verb's outcome carries through it:

- `OffsetOutcome::carry_contacts`, for all five offset doors;
- `MergeCoplanarOutcome::carry_contacts`;
- `Shelled::carry_contacts`;
- `SplitResult::carry_contacts`, which returns one list per side, reads
  a vertex the cut copied as each side's copy, and refuses `Parted`;
- `Blended::carry_contacts`, for fillet and chamfer;
- `transform_rigid`, which keeps its keys, through `carried_from(0)`.

The merge's lineage has one home, `MergedGroup::substitutions`, which
both the carry and the boolean's descendant map read. A record whose
two cells map to one cell is consumed, as in the boolean.
`RekeyRefusal` was folded into `CarryRefusal`.

Residue:
`surgery-carry-keeps-a-divided-cells-record-on-the-piece-that-kept-its-key`,
and the design question
`a-blend-or-shell-that-consumes-a-recorded-cell-refuses-the-record`.

## Closed 2026-10-11 (PR 4694)

Topo's surgery verbs carry records through `ContactRecords::carried_through` and per-verb `carry_contacts`, refusing typed. The residue is filed: `surgery-carry-keeps-a-divided-cells-record-on-the-piece-that-kept-its-key` (now including the edge-kiss split) and `a-blend-or-shell-that-consumes-a-recorded-cell-refuses-the-record` (design).
