---
id: the-metric-clearance-dials-door-outlives-the-assertion
kind: issue
title: clearance_at_dials still takes a metric ClearanceBound at its dials, a second spelling beside the clearance assertion
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

What it left: `clearance_at_dials` / `clearance_over_at_dials`
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

## Update (2026-11-02, PR 6292's fix pass)

The door is fenced but not gone. It is renamed
(`clearance_at_dials`, `clearance_over_at_dials`), its module header
says it is the engine's, and it has one constructor,
`ClearanceQuery::at_dials(bound, tol)`.

**Two drivers of one sweep** (the style review's S-m2). The assertion
(`eval/wire.rs`, `wire_clearance`) and this door (`clearance.rs`,
`Replayed::at`) each resolve seats, read bodies into a frame, build
windows, run `Sweep` and verify pairs, and agree only because they
are kept in step by hand. The correctness review measured where they
differ:

- the node log (`unlogged("F30")` is the wire's alone);
- the `RecordedContact` check (the door's, for the strict question
  only);
- the budgets (the door's dials against the wire's defaults);
- `Pruning::Facets` (the door's alone);
- the witness rebuild (once per candidate here, once up front there).

`ClearanceRefusal::of_bound` is now the one bound check both call.
The rest is this row's: one driver with the dials as its parameters,
or the door gone.

**A class of rows this row omitted** (S-m12). Besides the M10-5
engine rows at budgets and oracles, some rows read the `Selection`
door's own refusals: a profile node, a node that does not exist, an
edge, an empty or multi-face scope, two spaces, a body index the
node lacks, a standing that did not build. They live in
`clearance_refusals_say_what_they_found.rs`, `m10_5_*`,
`p2_gauge_poses_and_doors.rs` and `node_standing.rs`, and ask through
`fixture::dials_clearance`. An assertion refuses each at insert,
with its own document refusal (`SlotVarKind`, `SpaceMismatch`,
`UnresolvedInput`), or as a node error at evaluation. So when the
door goes, these rows move to those refusals, with
`SelectionRefusal`'s words carried over where the document's are
thinner. They do not move to the sweep.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.
