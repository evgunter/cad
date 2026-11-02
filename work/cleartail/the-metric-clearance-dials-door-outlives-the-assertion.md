---
id: the-metric-clearance-dials-door-outlives-the-assertion
kind: issue
title: clearance_with still takes a metric ClearanceBound at its dials, a second spelling beside the clearance assertion
status: open
opened: 2026-11-02
priority: P3
cost: M
---



Found by DOORS' `a-clearance-assertion-is-decided-by-the-sweep`, which
retired `editor_core::clearance::clearance` (the `Selection`-and-`c`
door) and `ClearanceQuery::at_least` into the clearance assertion read
through `drive::assertion_at` (ERROR-DESIGN E3's amendment, Ev on PR
6261: "`editor_core::clearance`'s second spelling retires into it").

What it left: `clearance_with` / `clearance_over`
(`crates/editor-core/src/clearance.rs`) still take a `ClearanceQuery`
whose `bound` field is public, so a caller can still ask
`ClearanceBound::at_least(c)` between two `Selection`s — at the dials a
document assertion has no spelling for (`ClearanceConfig`'s budgets,
`Pruning::Facets`, a `MonotoneOracle`). About sixty engine rows in
`m10_5_clearance_interval.rs`, `m10_5_r1_probes_interval.rs` and
`m10_5_r2_probes_interval.rs` (and `msolve14_run_scalar.rs`) ask the
metric question through it, reading receipts at starved budgets and the
accelerator seam. The unit moved every row that asked the plain
question (`clearance(…, c, …)`) onto the assertion
(`fixture::clearance`), and kept these.

## Repair shape

The strict (self-intersection) question keeps the door; the metric
rows move to `topo::clearance::Sweep` directly (`crates/sweep/tests/`
already asks it there with its own verifier), where budgets are the
sweep's own and the accelerator's facet restriction is a box the test
evaluates over. Then `ClearanceQuery::bound` goes private with
`strictly_positive` its one constructor.

`topo::clearance::min_separation` stays for the same reason: no
evaluation asks it, and the sweep suites
(`crates/sweep/tests/self_intersection_proves_injectivity.rs`,
`clearance_reads_an_iso_bounded_wall.rs`) and two editor-core probe rows
read it as an independent bracket beside the sweep. It leaves when they
stop.
