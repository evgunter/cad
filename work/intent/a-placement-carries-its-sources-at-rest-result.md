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
