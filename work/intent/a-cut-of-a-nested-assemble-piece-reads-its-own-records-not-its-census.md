---
id: a-cut-of-a-nested-assemble-piece-reads-its-own-records-not-its-census
kind: issue
title: A boolean distributed over a ⊔ cuts each nested ⊔ piece with that piece's own records, not the touches its census kept
status: open
opened: 2026-10-29
priority: P3
cost: M
refs: [a-mirror-of-an-assemble-reports-its-findings-twice]
---


Found by PR 5999's fix pass, folding a boolean operand's records into
its carry (`Carry::kept`, `crates/editor-core/src/eval/carry.rs`). This
is read from the code; no row witnesses it yet.

## What

A boolean reads a whole operand's records as that operand's door kept
them: `distribute::Operand::read` and the union/intersect `MemberRead`
take `value.carry.kept(doc, tol)`. That is the census-kept record set,
including the touches the `⊔`'s census recorded between its pieces.

A cut or meet distributed over the operand's pieces is different.
`distribute::Operand::units` (`crates/editor-core/src/eval/wire/distribute.rs`)
takes each piece's carry as the piece holds it. A piece that is itself a
`⊔` therefore brings only its members' records, never the touches its own
census recorded between them. `units` takes no document, and the census
needs one to run. So a meeting inside a nested `⊔` piece reaches that
piece's boolean unrecorded, where the same `⊔` read whole reaches its
boolean recorded.

## Done when

A row: `((bolt ⊔ plate) ⊔ c) − t` must decide the seat the same way
`(bolt ⊔ plate) − t` does, with `t` clear of the seat. The likely fix
is for `units` to take the document and use `Carry::kept` per piece.
That would make a cut run a nested census it does not run today, so the
row should state that cost.
