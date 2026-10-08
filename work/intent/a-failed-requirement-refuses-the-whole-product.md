---
id: a-failed-requirement-refuses-the-whole-product
kind: issue
title: A report-only Measure or Assertion root that fails refuses the whole product gather; under D10's explicit product list a check must never gate the product
status: closed
opened: 2026-10-04
priority: P0
cost: M
refs: [a-measured-part-is-not-a-product-root]
closed: 2026-10-08
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

## Stage 2 slicing (2026-10-07)

Re-parked on `the-product-is-an-explicit-list` (INTENT stage 2 PR C,
`docs/INTENT-STAGE2-SPEC.md` §4, test 8): the gather reads only listed `Body`
variables, so a failing check cannot reach it. Stage 5's assertions do not
change that acceptance.

## Closed

By INTENT stage 2 unit C (`the-product-is-an-explicit-list`, branch
`intent/s2-c-world`): the gather reads only the placements, and a
measure or an assertion is no placement, so a failing one reports and
the product builds: `crates/editor-core/tests/intent_s2_c_world.rs`,
`a_failing_measure_and_its_assertion_gate_no_placement`.
