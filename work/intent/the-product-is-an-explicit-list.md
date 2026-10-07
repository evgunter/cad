---
id: the-product-is-an-explicit-list
kind: issue
title: D10 stage 2 PR C: the product is an explicit list of Body variables; A10's sink rule, coverage, ancestor-freedom and root maintenance retire
status: parked
opened: 2026-10-07
priority: P0
cost: M
design: true
blocked_on: [operands-are-reads]
refs: [a-measured-part-is-not-a-product-root, a-failed-requirement-refuses-the-whole-product]
---

INTENT stage 2, PR C. Spec: `docs/INTENT-STAGE2-SPEC.md` §4.

`Doc::roots` becomes `Doc::product: Vec<VarId>` of `Body` variables. `roots.rs`
(coverage, ancestor-freedom, sink maintenance) is deleted, and the gather reads only
the list. This closes `a-measured-part-is-not-a-product-root` (the cut plate is its
product) and `a-failed-requirement-refuses-the-whole-product` (a failing check reads
no listed body).

`design: true`: FORK-2 (how the list is kept under edits, replacing A10's maintenance
clause) is Ev's, and ASSEMBLY A10's replacement text waits on it.
