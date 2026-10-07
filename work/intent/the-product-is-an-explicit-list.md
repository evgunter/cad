---
id: the-product-is-an-explicit-list
kind: issue
title: D10 stage 2 PR C: the product is the world — PlaceInWorld { body, pose } and a derived product; roots.rs, A10's invariants and maintenance, PlacedUnderTwoRoots/N4, D-2's consumer-ward closure and InstanceConsumed retire
status: parked
opened: 2026-10-07
priority: P0
cost: M
blocked_on: [operands-are-reads]
refs: [a-measured-part-is-not-a-product-root, a-failed-requirement-refuses-the-whole-product, error-design-e3-calls-a-measure-a-sink]
---

INTENT stage 2, PR C. Spec: `docs/INTENT-STAGE2-SPEC.md` §4.

The product is the world (spec §4). `PlaceInWorld { body, pose }` places a copy. `roots.rs` (coverage, ancestor-freedom, sink maintenance), `Doc::roots` and `SetRoots` are deleted, and the gather reads only the placements. This closes `a-measured-part-is-not-a-product-root` (the cut plate, placed, is its product) and `a-failed-requirement-refuses-the-whole-product` (a failing check reads no placement).

## FORK-2b ruled (2026-10-07, #4220)

The unit is now "the product is the world" (spec §4). It lands `PlaceInWorld { body, pose }` and a product derived from the placements, not a stored list. Python and the Rust façade place only when told, with one semantics. The viewer's feature gestures re-point the placement. A second identity placement is allowed.

The audit's C-side changes ride here, because each one is stated over placements:
- D-2's consumer-ward closure narrows;
- `InstanceConsumed` retires;
- `PlacedUnderTwoRoots` and N4's once-per-product rule retire.

Membership is checked once, at the migration (one placement per body-denoting root, in root order). The id is kept.
