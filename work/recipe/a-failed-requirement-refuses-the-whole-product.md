---
id: a-failed-requirement-refuses-the-whole-product
kind: issue
title: product refuses the whole document when a report-only root (a Measure or Assertion) fails, though a requirement observes the product and never conditions it
status: open
opened: 2026-10-03
priority: P2
cost: M
---


## The finding

Found by the designer pair on `a-measured-part-is-not-a-product-root` (by reading; not yet pinned). `product::product`'s pass 1 reads `Evaluation::usable(root)` for every root, so a measure that fails (an unresolved name, `MeasureClearanceRefused`) poisons its assertion root and the product refuses `ProductError::Root`. "A document whose only addition is an assertion has the same product" (`product.rs`, E10's report-only rule) is then false whenever the requirement cannot be evaluated.

## What would close it

Pass 1 skips the standing of report-only roots (Measure, Assertion): "no partial products" is about material, and a verdict is not material. A failed mate keeps refusing, since it carries placement. First a row that pins today's refusal. Lands after the `[ev]` answer on `a-measured-part-is-not-a-product-root` (which puts the part beside its assertion in the root set).

Filed by the RECIPE orchestrator from the designers' report.
