---
id: a-zip-listed-chord-between-merged-faces-along-a-rim-has-no-end-to-end-witness
kind: issue
title: A zip-listed seam edge between two merged faces that lies within an operand rim is read only by unit rows: no document reaches it
status: open
opened: 2026-10-20
priority: P3
cost: M
---


## Finding

`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_edges`: a
zip-listed seam edge whose two faces are both merged and which
`chord_kind` reads as a piece of an operand rim (`merged_chord_rim`
holding, or `one_operand_chord`'s `Rim`) goes to `zip_rims` and is
named as that rim's piece. Only the unit rows that call
`merged_chord_rim` directly (`emit_topo`'s `every_order` rows) read
this path.

Measured on `wire/pair-seam-name-order` (PR 5469): a probe that
recorded each test reaching a `zip_rims` push fired in no test of the
full editor-core suite (3062 run, slow set included). A likely reason:
an edge flush with an operand rim is carried as that rim's piece rather
than zip-listed. Nothing says the zip never lists such an edge, though,
so the path stands, unwitnessed end to end.

Wanted: a document that reaches it through `chord_kind`, or the reason
none can (which would make the arm row 4).

Signed (WIRE lane `wire-seamorder`).

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
