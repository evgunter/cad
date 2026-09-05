---
id: SEAT-8
kind: unit
title: split migrates onto the Verb substrate — the two-sided out-type
status: closed
opened: 2026-09-05
branch: seat/splitverb
refs: [SEAT-7, 1910]
pr: 1950
closed: 2026-09-05
---


Spec: `docs/SEAT-8-SPEC.md` (deleted at merge per `docs/DOC-LEDGER.md`).
`Node::Split` onto `Verb<T>`: `Verb::Split { plane }` with the body as
the one operand, a per-door out-type carrying TWO `Body | Empty` sides
under one record (the unit's structural decision), `VerbRecord::Split`,
tag 7 pinned, an explicit empty flow row (a section plane has no stored
scalar field), the one-index-space stamping across both halves as a
red-first row, and an `Empty`-side corpus document so that token can
red. Block SEAT-B3's first slot; the block's byte was drawn at this
dispatch and stays private until the block closes.

## Closed

PR 1950 (2026-09-05). Delivered as specified with one argued choice
(`SplitOut` over a record-with-body door) and five disclosed
deviations; the dual found zero MAJORs; the fix pass consolidated the
record-unwrap rule and the provenance digest into one home each. The
empty-side document stays in-suite; registering it in the corpus is
unscheduled and Ev's call (both review arms). The spec is recoverable
at `git show 57dc0fe3a:docs/SEAT-8-SPEC.md`.
