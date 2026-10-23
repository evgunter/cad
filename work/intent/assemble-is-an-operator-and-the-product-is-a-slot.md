---
id: assemble-is-an-operator-and-the-product-is-a-slot
kind: unit
title: D′3: Assemble (⊔) is the fourth operator, with the census at its door; one product slot; a placement copies one source body; an instance reads its part's product; picks, so RoleSeg::Placed/InPart and CopyKey retire
status: review
opened: 2026-10-22
priority: P0
cost: H
parent: a-placement-pins-by-written-values-without-a-mate
branch: intent/dprime3-assemble-product-slot
pr: 5636
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

## Handoff

Lane: D′3 implementer, branch `intent/dprime3-assemble-product-slot`.

**Split decision: one PR.** The spec's seam (one source and picks
first, then `⊔`) leaves no spelling for a list placement in PR 1: a
list `Place` under a mated bundle cannot become several placements
(the mate's copy side reads one copy), and inline of a part with
several world placements writes exactly that list. One source needs
`⊔` to be written at all, so the two halves land together.

**Plan, in commit order** (each compile-driven):
1. Names: `RoleSeg::Placed` and `InPart` retire; a placement's and an
   instance's copy carry their source's names verbatim (the copy's
   body row stays its own).
2. `Place { source, target, constraints }`, one copy.
3. `Node::Assemble { members }` (`⊔`): graft pieces, names keyed by
   read as a union's members are, records carried; the census at its
   door (`product_in`'s machinery over its members as arrivals).
4. `Doc.product` slot, `DocEdit::SetProduct`, wire + content key;
   `product_recorded` reads the slot; gather retires.
5. `InstantiatePart` defines one body: its part's product.
6. Picks (`VarDef::Pick`); `CopyKey`, `BodyRead.key`, `SlotValue::Copy`,
   `Copies`/`KeyedCopy`, `copy_of` retire.
7. Inline lands the copied product body; split/flatten follow.
8. pncad, pncad-py, viewer, tour, wild; stored documents regenerated.

**Progress:**
- Steps 1–8 landed. Editor-core: `⊔`, the slot, one source, the instance's one body, picks; `CopyKey`, `RoleSeg::Placed`/`InPart`, `Copies`/`KeyedCopy`, `SlotValue::Copy` and `NodeErrorKind::ProductOperand` are gone. The census runs at each `⊔`, where it is gathered: at the product, and for every `⊔` the slot does not read (`product::assemble_gathers`).
- A `⊔` reading one body twice gathers its names as a tie (`names::repeated_member_view`), not a panic.
- Acceptance rows for tests 19, 25 and 26: `crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`.
- pncad, pncad-py (bindings, stubs, Python suite), viewer, tour and the guide are migrated. A test document says its product (`placing.world_product`, `fixture::world_product`).
- Filed: `a-name-through-a-parts-assemble-moves-with-every-earlier-part-edit`, `a-boolean-over-an-assemble-cites-none-of-its-meetings`, `an-unread-body-is-no-finding`, `the-viewer-authors-no-pick`, `the-viewer-sets-no-product`, `the-pick-cross-check-cannot-tell-two-copies-of-one-body-apart`.
- Absorbed items closed. `the-world-still-speaks-of-roots` was read and is left open with a note: the rename is a naming choice.
