---
id: SEAT-8
kind: unit
title: split migrates onto the Verb substrate — the two-sided out-type
status: review
opened: 2026-09-05
branch: seat/splitverb
refs: [SEAT-7, 1910]
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
