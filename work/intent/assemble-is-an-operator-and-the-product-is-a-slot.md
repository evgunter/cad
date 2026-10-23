---
id: assemble-is-an-operator-and-the-product-is-a-slot
kind: unit
title: D′3: Assemble (⊔) is the fourth operator, with the census at its door; one product slot; a placement copies one source body; an instance reads its part's product; picks, so RoleSeg::Placed/InPart and CopyKey retire
status: open
opened: 2026-10-22
priority: P0
cost: H
parent: a-placement-pins-by-written-values-without-a-mate
---



## What

INTENT stage 3, D′3. Spec: `docs/INTENT-STAGE3-SPEC.md` §5′.3,
which governs where this body and it differ. It builds PR 5494's
`[operations.operators]`, `[operations]`' product slot,
`[placement.placement]`'s one source and picks, and `[assertions]`'
census at each `⊔` (rows 189–191):

- `Node::Assemble { members }`, whose door grafts its operands as
  pieces and runs `record_at_rest` over them, one arrival per operand
  at one index value; a meeting inside an operand is carried, never
  re-decided, and an area self-touch refuses typed.
- `Doc.product`, one slot reading a body in the world
  (`DocEdit::SetProduct`); the gather of world placements retires.
- `Place { source, target, constraints }` copies one body; an
  instance copies its part's product body, its decisions carried.
- Picks (`VarDef::Pick`), one piece of a copy; `CopyKey`,
  `RoleSeg::Placed`/`InPart`, `Copies`/`KeyedCopy` retire.
- Inline lands the copied product body; the viewer's forms read
  picks.

Waits on D′2: inline, which D′2 makes one rule, lands a product
body only once this unit has a product slot, and both rewrite
`refactor.rs`'s inline and `node.rs`'s `Place`.

If it is too large for one green PR it splits along one seam only:
one source and picks first, then `⊔`, the product slot and the
instance reading its part's product (spec §5′.3, Risks).

## Done when

The spec's §5′.3 rows, and tests 19, 25 and 26. Product digests
unmoved; names, ids and stored documents move (regenerated); census
rows move only where a `⊔` groups copies differently from today's
gather, each reported.

**Absorbs:** `a-space-names-its-bodies`,
`name-words-budget-grew-under-placed-copy-wrapping`,
`a-placement-copy-is-its-own-table-in-the-product`,
`an-instance-defines-one-body-per-part-placement`,
`the-viewer-authors-no-read-of-one-copy`.
