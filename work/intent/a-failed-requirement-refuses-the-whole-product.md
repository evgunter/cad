---
id: a-failed-requirement-refuses-the-whole-product
kind: issue
title: A report-only Measure or Assertion root that fails refuses the whole product gather; under D10's explicit product list a check must never gate the product
status: parked
opened: 2026-10-04
priority: P0
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
refs: [a-measured-part-is-not-a-product-root]
---


Found in the closed `[ev]` PR #3929's diff, where it was filed but never
reached main. When a report-only `Measure` or `Assertion` root fails
(poisoned measure, unresolved reference), the product gather refuses
the whole document, so a requirement that is meant only to report
blocks every consumer of the product. Under D10 an assertion "checks
and never places" and the product is an explicit list of `Body`
variables, so a failing check has no business reaching the gather.
INTENT stage 2 (the explicit product list) and stage 5 (assertions)
settle it; this row's acceptance is that a failing assertion reports
and the product still builds.
