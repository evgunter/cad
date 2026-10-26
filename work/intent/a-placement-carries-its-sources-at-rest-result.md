---
id: a-placement-carries-its-sources-at-rest-result
kind: unit
title: A placement carries its source body's at-rest result (contact records and census decisions) with its copy, at every placement
status: parked
opened: 2026-10-19
priority: P1
cost: M
refs: [each-space-runs-its-census-between-arrivals, a-part-drops-its-census-findings-at-the-seam]
blocked_on: [assemble-is-an-operator-and-the-product-is-a-slot, an-operation-computes-in-a-frame-of-its-reads]
---

**Sequencing:** `docs/INTENT-STAGE3-SPEC.md` §12 ("Sequencing: the seat and census units"), step 5. Parked on D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`), which makes a placement copy one source body and an instance copy its part's product, and on E's core (`an-operation-computes-in-a-frame-of-its-reads`), since both rewrite the value `wire_place` defines and the carry rides on E's `Framed`. It shares `product.rs`, `eval/wire.rs` `wire_place` and `eval/parts.rs` `PartValue` with both.

D10 `[placement.placement]` (PR 4666, restated by PR 5494): a placement defines one copy of a source body, "which carries every pose of the source as the source carries it, and the source's contact records and census decisions, which stay the source's"; an instance's source is another document's product body, its decisions "carried with it and reported once, as that document's" (`[placement.world]`).

`a-part-drops-its-census-findings-at-the-seam` builds this at the instance seam (`crates/editor-core/src/eval/parts.rs`, `PartValue`). Once the census is each `⊔`'s door, every placement is such a seam. Generalise the carry from instance placements to every placement, so one type crosses every placement. With the census deciding nothing inside an arrival, the parent's re-found in-copy touches stop existing, and the solid-pair match becomes the confirm-and-match arm of `each-space-runs-its-census-between-arrivals` (each `⊔` decides its operands' meetings). `an-escalated-census-finding-names-no-pair` stays a prerequisite of matching an escalated finding.

## Re-scoped (D′3's fix pass, 2026-10-26)

D′3 built the carry for a door's census. The census at a `⊔`'s door
keeps its verdict on the `⊔`'s value, and a placement of the `⊔`, or a
pick of one piece, carries that verdict (`NodeValue::census`, keyed in
the source's body, which a placement moves without re-minting a key).
A census reading the copy as one arrival discards what it finds inside
it (`NodeValue::censused`), so a placed seat made the product is at
rest and its meetings are reported once, at the `⊔`
(`crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`,
`a_placed_seat_made_the_product_is_at_rest`). An instance's copy is
censused the same way, which retired the solid-pair match
(`census_pairs`, `less_carried`).

What is left: the carry rides E's `Framed` (`an-operation-computes-in-a-frame-of-its-reads`),
and a value whose source no door censused (a boolean's result, a
pattern's member) carries no census, so the meetings inside it are
still the census's to find wherever it is read as one arrival.

