---
id: min-separation-tightening-crosses-the-drive
kind: issue
title: min_separation cannot consult the chart boundary: its funnel rows cross the drive and refuse every leaf
status: open
opened: 2026-09-13
refs: [clearance-window-tightening-needs-chart-boundary]
---


## What

TRIM-3 PR-2's spec (§3(b)) puts the chart-boundary drop at the head of
BOTH subdivisions: the clearance sweep's and `min_separation`'s. The
sweep's half landed. `min_separation`'s did not, and the measurement
is why.

`min_separation` is called from inside an EVALUATION — by the
`min_clearance` measure primitive
(`crates/editor-core/src/measure.rs`, the `MinClearanceLane` interval
impl) — not from a door a consumer calls after a drive. The drive
certifies a leaf by comparing the predicate census at the box's
corners, and `RefusalReason::FlipCrossing` names both flips and
DIVERGENCES (a predicate decided a different number of times). The
description's drops are funnel decisions, and how many of them a query
takes is a function of how far the subdivision ran, which is a
function of the parameter box. So every leaf crosses a divergence.

**Measured** (PR-2's lane, `cargo test -p editor-core --features
interval --test all -- m10_6_`): with the tightening wired into
`min_separation`, six rows lose their certified leaf —

- `m10_6_r1_probes_interval::a_planted_violated_reads_violated_over_a_certified_leaf`
- `m10_6_r1_probes_interval::the_at_least_arm_that_read_the_carriers_end_now_refuses_by_name`
- `m10_6_r1_probes_interval::the_bracket_walk_through_the_public_doors`
  (`0 certified, 1 refused (flip_crossing)`, 99.73 % of the mass)
- `m10_6_min_clearance_interval::the_drive_certifies_and_the_assertion_holds_over_the_certified_leaves`
- `m10_6_min_clearance_interval::a_stackup_over_a_min_clearance_forfeits_its_advisory_columns_and_still_gates`
- `m10_6_ci_rows_interval::every_registered_assertion_holds_over_the_certified_leaves_within_budget`
- `r2_m10_6_probes_interval::a_tolerance_study_end_to_end_through_the_public_doors`

Causation is the two-run differential: the rows are green with
`min_separation`'s band `None` and red with it `Some`, nothing else
changed. The MECHANISM above is read off `render_reason`'s divergence
arm and is not itself separately measured — a lane that takes this
item should print the flipped predicate names first.

`min_separation`'s own doc already says what this is a consequence of:
*"Nothing here decides."* That sentence was a design position, and
this is the price of leaving it.

## What it costs

The `min_clearance` bracket's `hi` is still the WINDOW's. On the M10-6
notch fixture the bracket is `[0.100…, 0.100…]` against a true face
separation of `0.269`, unchanged by PR-2
(`m10_6_r1_probes_interval::the_notch_bracket_is_the_windows_not_the_faces`),
and `Certified::LowerBoundOnly` goes on refusing the two unsound arms
for a second reason on top of
`work/trim/exact-region-cells-for-lower-bound-only.md`'s.

## Fix shape

Either the census learns that a subdivision's internal drops are not
model predicates (a decision site that records no row, which is a
`geom-core` question and an E-level one — nothing in the funnel is
silent today), or `min_clearance` moves out of the evaluation so its
engine may decide like every other door. The second is SHELL-3's
neighbourhood; the first is the smaller change and the larger ruling.

## Home

TRIM filed it; the ground is M10's (`clearance.rs`, `measure.rs`) and
the ruling is Ev's.
