---
id: min-separation-tightening-crosses-the-drive
kind: issue
title: min_separation cannot mint a chart boundary: the walk's rows are recorded on one lane only and every drive leaf refuses
status: open
opened: 2026-09-13
refs: [clearance-window-tightening-needs-chart-boundary]
priority: P1
cost: D
---


## What

TRIM-3 PR-2's spec (§3(b)) puts the chart-boundary drop at the head of
BOTH subdivisions: the clearance sweep's and `min_separation`'s. The
sweep's half landed. `min_separation`'s did not, and the measurement
below is why — **not** the reason PR-2 first gave, which the v6 dual
refuted on both arms.

`min_separation` is called from inside an EVALUATION, by the
`min_clearance` measure primitive (`crates/editor-core/src/measure.rs`,
the `MinClearanceLane` interval impl). The drive certifies a leaf by
comparing the predicate census at the box's corners, and
`RefusalReason::FlipCrossing` names both flips and DIVERGENCES (a
predicate decided a different number of times).

**The rows that diverge are the boundary WALK's, not the description's
drops.** With the tightening wired in, `render_reason` prints, on every
refused leaf, only

```
flip_crossing 4:pcurve_loop_closure:count 0->4
              4:pcurve_loop_closure_height:count 0->4
              4:pcurve_loop_continuity:count 0->12
              4:pcurve_loop_pole_joint:count 0->14
```

— `chart_boundary`'s loop walk, on the body node, always `0 -> N`, and
**no `chart_bound_*` drop row appears at all**. The count is `0` on one
side because the `f64` witness build never walks: `MinClearanceLane for
f64` answers `None` (`measure.rs`, the f64 impl), so that lane mints no
description. The interval leaf rebuilds one inside the evaluation and
records `N`. So the divergence is a **lane split** — one build walks,
the other does not — present on every box whatever its width, and *any*
funnel row added anywhere inside `min_separation` crosses it. It is not
"a decision count that is a function of the parameter box", which is
what PR-2's first draft of this file, its code comment and its PR body
all said.

This is also TRIM-3's **STOP 4** arriving unremarked: the spec
pre-registered "a k-lint fires on a `pcurve_loop_*` / `pcurve_chart_*`
row from the new caller" as a stop condition, and these are those rows
from that caller, reaching the drive's census rather than a k-lint.

**Measured** (PR-2's lane, `cargo test -p editor-core --features
interval --test all -- m10_6_ r2_m10_6_`): with the tightening wired
into `min_separation`, seven rows lose their certified leaf —

- `m10_6_r1_probes_interval::a_planted_violated_reads_violated_over_a_certified_leaf`
- `m10_6_r1_probes_interval::the_at_least_arm_that_read_the_carriers_end_now_refuses_by_name`
- `m10_6_r1_probes_interval::the_bracket_walk_through_the_public_doors`
  (`0 certified, 1 refused (flip_crossing)`, 99.73 % of the mass)
- `m10_6_min_clearance_interval::the_drive_certifies_and_the_assertion_holds_over_the_certified_leaves`
- `m10_6_min_clearance_interval::a_stackup_over_a_min_clearance_forfeits_its_advisory_columns_and_still_gates`
- `m10_6_ci_rows_interval::every_registered_assertion_holds_over_the_certified_leaves_within_budget`
- `r2_m10_6_probes_interval::a_tolerance_study_end_to_end_through_the_public_doors`

Causation is the two-run differential: green with `min_separation`'s
band `None`, red with it `Some`, nothing else changed. Both reviewer
arms of the v6 dual reproduced the same seven rows and the same
`render_reason` output independently.

**What was never measured**, and should be said: only the ROOT cut was
ever wired for this door, so the per-cell drops' own contribution to
the census is unknown. They are `chart_bound_gap` and the parity rows,
and none of them appeared — but they could not, since the walk refused
the leaf first.

## What it costs

The `min_clearance` bracket's `hi` is still the WINDOW's. On the M10-6
notch fixture the bracket is `[0.100…, 0.100…]` against a true face
separation of `0.269`, unchanged by PR-2
(`m10_6_r1_probes_interval::the_notch_bracket_is_the_windows_not_the_faces`),
and `Certified::LowerBoundOnly` goes on refusing the two unsound arms —
now for two reasons: this one, and
`work/trim/exact-region-cells-for-lower-bound-only.md`'s.

## Fix shape

The asymmetry is the subject, so the fix has to close it or move the
walk out of the census, and neither is a change to the description:

- **Close the lane split.** `MinClearanceLane for f64` answers `None`
  because `clearance`'s engine is written at `Interval` concretely. A
  f64 lane that walked — or a census that knows a leaf's two builds do
  not run the same lanes — makes the two counts agree. This is the
  honest target and the larger one.
- **Move the mint off the evaluation path.** A description built once
  per body per leaf, outside the measure's own evaluation, records its
  walk once rather than once per corner. Adjacent to SHELL-3's move of
  the engine's body-level half into `topo`.

What is NOT the fix, and what PR-2's first draft of this file aimed at:
teaching the census that a subdivision's internal drops are not model
predicates. Those rows never diverged.

## Home

TRIM filed it; the ground is M10's (`clearance.rs`, `measure.rs`) and
the ruling — whether a lane split may be invisible to the census — is
Ev's.
