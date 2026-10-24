---
id: a-boolean-over-an-assemble-cites-none-of-its-meetings
kind: issue
title: A boolean reading a ⊔ cites none of the ⊔'s meetings, since the ⊔'s value carries no record of its census
status: open
opened: 2026-10-23
priority: P1
cost: M
refs: [assemble-is-an-operator-and-the-product-is-a-slot, a-placement-carries-its-sources-at-rest-result, each-space-runs-its-census-between-arrivals, a-vertex-piercing-two-declared-coincident-faces-reads-as-one]
---

Found by D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`), building test 25 (`docs/INTENT-STAGE3-SPEC.md` §11 row 25, §5′.3 "∪ over a ⊔").

What D′3 built: a `⊔`'s result is an ordinary operand. `NodeErrorKind::ProductOperand` (FORK-2b's refusal) is retired, so `(p ⊔ q) − t` cuts both pieces, and `(bolt ⊔ plate) ∪ bead` keeps the plate a piece of its own when the bead touches only the bolt. The kernel's union keeps an operand's solids apart, so the bolt–plate meeting is not decided again (`crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`, `a_cut_through_an_assemble_keeps_its_pieces` and `a_seat_unioned_with_a_bead_keeps_its_pieces`; `crates/editor-core/tests/docm6_seam_declarations.rs`, `a_boolean_over_a_product_keeps_its_parts`).

What it did not build: two halves of the spec's row.

- **The record is not carried.** The census at a `⊔` runs where the `⊔` is gathered (`crates/editor-core/src/product.rs`, `assemble_gathers` and `gather`), at the product and at check time, not in the `⊔`'s wire (`crates/editor-core/src/eval/wire.rs:510`). So the `⊔`'s value holds no contact record (`NodeValue::contacts` is empty), and a union reading it has nothing to cite: "keeps the bolt–plate seat as the `⊔`'s record, cited" does not hold. The same is true of a placement of a `⊔`. The product census sees the copy as one arrival and re-finds the touch inside it, which `a-placement-carries-its-sources-at-rest-result` ends.
- **A bridged seat refuses untyped for the case.** A bead touching both bolt and plate makes one solid touching itself where they met. The union refuses with the kernel's own boolean refusal (`NodeErrorKind::Boolean(VertexReadTwice { .. })` on the row's scene), not with a refusal that names the bolt–plate pair as a self-touch.

Shape of a fix: run `record_at_rest` over the `⊔`'s pieces in its wire (`product.rs`'s `gather` is the machinery), so the value carries its records. Then have a boolean carry an operand's records through its lineage (D1 `[topology.records]`), and type the self-touch refusal from the pair the records name. This touches `a-boolean-reading-two-bodies-of-one-arrival-re-decides-their-contact` (stage 4, `design: true`), whose question PR 5494's rule answers.

## Re-scoped (D′3's fix pass, 2026-10-24)

The bridged seat's half moved to the kernel:
`work/cleave/a-vertex-piercing-two-declared-coincident-faces-reads-as-one.md`.
The kernel reads the bolt's foot and the plate's top apart, so the
union refuses `VertexReadTwice` before any self-touch could be read.
The row pins that refusal under a name that says what it becomes
(`a_seat_unioned_with_a_bead_keeps_its_pieces_and_a_bridge_is_read_twice`).

`−` and `∩` now distribute over a `⊔` operand
(`crates/editor-core/src/eval/wire/distribute.rs`). Each piece's
result keeps that piece's own records, so a cut through the seat keeps
both pieces (`a_cut_through_a_seat_cuts_each_piece_and_keeps_both`,
`a_meet_with_a_seat_meets_each_piece`).

What is left is the first half: the `⊔`'s value carries no record of
its census, so nothing above it can cite one. D′3's fix pass builds
that census at the `⊔`'s door, with the verdict on the value, after
ATREST stage 4 I's PR 5637. This item closes there.

