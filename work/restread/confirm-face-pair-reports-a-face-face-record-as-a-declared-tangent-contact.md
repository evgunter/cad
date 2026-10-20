---
id: confirm-face-pair-reports-a-face-face-record-as-a-declared-tangent-contact
kind: issue
title: confirm_face_pair reports a contradicted face–face record as a DeclaredContact { Tangent } it never was
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by both designers of the pinch-crescent fork (`[ev]` branch
`contacthold/ev-pinch-crescent`), off its question.

## What

`census::confirm_face_pair` (`crates/topo/src/census.rs:7870`) reads a
face–face record on two carriers through
`boolean::tangent_pair_at_rest` and `tangent_table_at_rest` and, where
either refuses
`Contradicted`, pushes `ValidationError::ContactContradicted` with
`declaration: DeclaredContact { a, b, class: ContactClass::Tangent }`
(`census.rs:7929`–`7940`). No mate declared that pair: the
record is a face–face record (C3) minted where an op's glue door
decided a tangency, and since PR 4775 that door's tangency claims no
material class (C4). The error names a declaration that never was, so a caller
reading the error to find what to fix is pointed at a mate.

## Owed

A contradicted face–face record reports as itself: the record (its two
faces, its granularity) and the witness, not a synthesised
`DeclaredContact`. Every reader of `ContactContradicted` that keys on
`declaration.class` checked when the shape changes.
