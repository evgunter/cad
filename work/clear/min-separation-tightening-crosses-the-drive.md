---
id: min-separation-tightening-crosses-the-drive
kind: issue
title: min_separation cannot mint a chart boundary: the walk's rows are recorded on one lane only and every drive leaf refuses
status: closed
opened: 2026-09-13
priority: P1
cost: M
branch: clear/tighten
pr: 5998
closed: 2026-10-29
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
`work/charttail/exact-region-cells-for-lower-bound-only.md`'s.

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

**2026-09-28:** `r2_m10_6_probes_interval::a_tolerance_study_end_to_end_through_the_public_doors` was deleted in the 2026-09-28 CI-latency cut.

## Re-homed (2026-10-24, CHART's split)

Moved from CHART to CLEAR by `git mv`, with its id and body unchanged.
The reason is ground: every site the fix shape names is the clearance
engine's (`editor-core/src/clearance.rs`, CLEAR's and SHELL's) or the
measure's (`editor-core/src/measure.rs`, FLUXHOLD's), and nothing in
`chart_bound.rs` is wrong on its own terms. CHART decided nothing here:
the open question is still whether a lane split may be invisible to the
census, and the body says that ruling is Ev's.

Re-priced `cost: M` and `design: true` from legacy `D`. Weigh it after
INTENT's `an-operation-computes-in-a-frame-of-its-reads` lands: that
unit changes how an operation (a measure among them) evaluates, which is
where the lane split lives. (CHART orchestrator)

## Note (CHART, 2026-10-24, `clearance-cells-are-decided-in-model-space`)

The description this row's walk rows came from is gone: `chart_bound.rs`
and `pcurves::chart_boundary` left the tree, and clearance decides a cell
off its face's edges in model space (`topo::cell_region`: `Inside`,
`Outside` or `Undecided`) and places a station with the face's point door
(`topo::cell_region::station_region`). So the rows a tightened
`min_separation` would record are now the `cell_region_*` gap rows and the point doors' own (`bool_contact_*`, the
region readings'), not the `pcurve_loop_*` walk rows measured above. The
lane split is untouched: the `f64` lane still answers `None` and records
none of them, so any decision inside `min_separation` still crosses the
drive. `min_separation` keeps `band: None`.

What lands here once the split closes is the measure's half of the
design: `hi` reads only cells `cell_region` decides `Inside` and
stations `station_region` places `In`, `OnEdge` or `OnVertex`, which
bound the FACES' separation from above, so `window_hi` becomes a bound
on the faces and `Certified::LowerBoundOnly` can retire. The clearance
sweep already admits only such stations as a `Violated` witness.

## Decided (CLEAR orchestrator, 2026-10-28)

Two designers were dispatched after INTENT's E merged. They answered
independently and reached the same final state in their first
reports. No ratified text changes, and no ruling is needed: the
question this row put to Ev dissolves.

- **The problem is not a lane split.** `min_separation` is a
  branch-and-bound: how many cells it examines depends on the width of
  its enclosures. A point build and a box build would record different
  counts even if both ran the search, and a point scalar cannot carry
  the bracket. So "an `f64` lane that walks" cannot close the
  divergence, and `f64` stays `None`.
- **The engine's readings go unlogged.** The engine's region readings
  (`window_of` with a band, `cell_region`, `station_region`) choose only
  which cells and stations feed the bracket, and every outcome leaves a
  true bracket. That is the shape of `[verdicts.consumers]`' lane
  choice, which records nothing. The tree already has a door for it:
  `k_stats::check_unlogged` (ledger rows F18 and on, in
  `docs/predicate-dimension-audit.md`).
  - Generalise it to a scoped form: a closure run under its own frame.
    Its verdicts and escalations are discarded, its K samples keep
    their predicate names, and a new ledger row states the argument.
    `check_unlogged` becomes the one-margin case of the scoped door.
  - The engine's door opens that scope around its readings. The
    receipt may count what the scope decided, as a receipt line, not a
    census row.
  - No node's census gains a row: `m10_6_certifying_keys.txt` must not
    move.
- **Then `min_separation` decides cells, as the sweep does.**
  - Definite `Outside` cells drop out of `lo`. `lo` stays the least
    `separation_lo` over every pair still standing, `Undecided` cells
    included.
  - `hi` reads only `Inside`–`Inside` cell pairs, or station pairs that
    the point doors place `In`, `OnEdge` or `OnVertex`. So `[lo, hi]`
    encloses the faces' separation.
  - An absent `hi` reads `Unevaluated`, and the box bisects under the
    existing rule.
- **What retires:** `Certified::LowerBoundOnly`,
  `UnevaluatedReason::WindowSuperset`, `WINDOW_TIGHTENING` (whose
  user-facing pointer names a file that no longer exists),
  `MinSeparation::window_hi`'s caveat, and `clearance.rs`'s header
  paragraph saying that the sweep's windows and the measure's differ.
  Whether `Certified::Neither` still earns its place is re-checked
  then.
- **Pins:**
  - the notch fixture's bracket contains 0.269, at more than one
    budget;
  - `the_notch_bracket_is_the_windows_not_the_faces` flips;
  - the seven M10-6 rows the earlier tightening broke stay green.
- **First probe:** the two-run differential (`band: None` against
  `Some(band)` under the scope). It also measures whether in-band
  escalations would have bisected every leaf (unverified, likely).

Sequencing: after `SHELL-3`, on the moved door and the generic
`cell_region` it delivers. It is independent of the symbolic unit.
Rejected: having the drive skip the measure node's rows where the
witness answered `MeasureUnavailable`, and running the measure outside
the evaluation. Both are argued in fork-log row 206.

## Amended at landing (CLEAR orchestrator, 2026-10-29)

Two points of the decision above did not hold as written. Both were
ruled on at TIGHTEN (#5998):

- **An absent `hi` does not make the drive bisect.** The drive bisects
  only on an escalation or a failed node, and an unbounded value would
  break a value invariant on every lane. The measure answers
  `MeasureUnavailable { NoUpperBound }`, the assertion reads
  `Unevaluated`, and the leaf certifies without bisecting. A missing
  `hi` comes from the cell budget, which a smaller parameter box barely
  changes. It never occurred in the first probe (0 of 35 calls).
- **`hi` is a true upper end up to the band.** `OnEdge` and `OnVertex`
  stations still count, as decided. They are in-band decisions of the
  point door, so such a station can sit up to the band outside its
  face. An `AtLeast c` over faces within the band above `c` can
  therefore read `Violated`, which is the kernel's ε model and not a
  defect. Every claim site and ledger row F28 say so.

## Closed

TIGHTEN (#5998), with this commit last.

- `k_stats::unlogged` is the scoped unlogged door. `check_unlogged` is
  its one-margin case, and its ledger row is F28.
- `min_separation` reads its regions inside that scope. It drops
  definite-`Outside` pairs from `lo` and reads `hi` only off placed
  stations.
- On the notch, the bracket now contains the faces' 0.2693:
  `[0.1601, 0.3881]` at 512 pairs.
- `Certified` retired whole, along with `WindowSuperset`,
  `WINDOW_TIGHTENING` and `window_hi`.
- No census moved.
- It costs about 2.4× on single-drive rows and about 1.25× on the
  suites; the cost is stated in `min_separation`'s `# Cost`.
- The single sequential review (byte 64) at `e27260a2` returned
  APPROVE-WITH-FIXES with no MAJOR. The orchestrator read and accepted
  the fix pass at `535a6988`.
