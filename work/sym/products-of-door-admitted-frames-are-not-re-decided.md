---
id: products-of-door-admitted-frames-are-not-re-decided
kind: issue
title: products of door-admitted rigid motions are not decided again, so a chain of admitted literal frames is rigid only within about one band a factor
status: open
opened: 2026-10-13
priority: P3
cost: M
refs: [SYM-17]
---

## What

Ev's ruling on SYM-17 (item 6 of `docs/SYM-17-SPEC.md`'s Phase 2):
products of admitted frames are not decided again. That stays. What it
costs, measured by both of SYM-17's reviewers on `a89f7a54eb`:

- `RigidMotion::check_rigid` admits a map whose seven rigidity margins
  decide `Zero`, so an admitted factor may be off rigid by up to the
  band's zero, a dimensionless ε.
- A product of `k` admitted factors is off by about `k` of them, and
  past the band's escalation the door would refuse the product it is
  never asked about.
  - R1's `r1_an_admitted_defect_compounds_under_compose`
    (`sym/17-review-r1` `4debacac6`,
    `crates/topo/tests/review_sym17_r1_probes.rs`): twenty squarings of
    an admitted 4e-10 stretch are a 4.2e-4 stretch, refused
    `NotRigid(col0_unit)` by the door, and `transform_rigid` moves a
    unit cube to an x extent of 1.000420.
  - R2's `r2_products_of_admitted_maps_drift_past_the_band`
    (`sym/17-review-r2` `e692e0b34`, `rigid_motion.rs`'s `r2_probes`):
    the 64-fold product of an admitted 4e-10 axis scale has
    `|c0|² − 1 = 5.1e-8`, refused `NotRigid(col0_unit)`.
- A document reaches it through a chain of literal steps or nested
  group frames (`placement.rs`'s `chain_motion` → `Frame::rigid_motion`
  → `Motion::compose`). On `main` before SYM-17 the composite was
  decided once, at `transform_rigid`.

The bound is now stated on the type (`RigidMotion`'s module docs) and
pinned by `rigid_motion.rs`'s
`a_product_of_admitted_factors_drifts_by_a_band_a_factor`.

## The lever

Designer B's, on SYM-17's design fork: **admission's standard, not a
second decision.** Leave products undecided, and tighten what the door
admits a factor at, so that the product of a chain as long as a
document builds stays within the band the body door's consumers assume.
How long a chain is, and so how tight, is the question.

