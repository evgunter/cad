---
id: the-two-lanes-decision-logs-differ-at-one-point-with-the-build-the-same
kind: issue
title: The f64 lane's and the box lane's decision logs differ at one point in a quarter of fully decided builds while the build is the same
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by M13 (the build-record measurement, 2026-11-24;
`probe/M13-REPORT.md` on `boxfinish/probe-build-record` at
`447298e0b`), by the parity instrument the designers of
`a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`
asked for: each build's `f64` log against its `Interval` log on a
zero-width box at the nominal, whole logs, checks included.

**2,311 of 8,673 fully decided builds differ in their logs; the record
of the build (standing, named cells, incidence, coincidences) is equal
in every one.** 2,106 part at a step or an end, 203 agree in choices
and part in check sequences, 2 turn a choice (`pcurve_chart_placement`,
the tour's bracket). First differences, `f64`'s entry against the box
lane's:

| `f64` logs | the box lane logs | builds |
|---|---|---|
| `pose_unit_norm` | `session_box_reach` | 1,348 |
| `pose_unit_norm` | `mate_chart_slides_span` | 625 |
| `props_circle_axis_class` (check) | a run the box lane lacks | 195 |
| `ssi_hull_sup` | `pcurve_projected_lever` | 51 |
| `bool_point_in_solid_advance` | `bool_point_in_solid_denom` | 31 |
| `path_leg_length` | `trace_plane_*` | 21 |

None is traced to source. The third is the class
`a-witness-records-curved-premise-checks-the-interval-replay-skips-so-no-curved-leaf-certifies`
repairs (the rounding lane's lifted pass becomes unlogged); the others
have no row.

Under today's certificate (equal whole logs) each is a leaf at its own
witness that cannot certify, reported as a flip naming nothing. What
each should be depends on that row's weighing; whichever gate stands,
a lane that logs a decision the other lane does not make is what the
audit ledger's unlogged door exists for, and each pair above wants its
cause: a pass one lane runs alone, or two lanes deciding one fact by
two predicates. First step: trace the six. (BOXFINISH orchestrator)

## Traced (M13b, 2026-11-24)

`probe/M13b-REPORT.md` on `boxfinish/probe-build-record` at
`3cd35c15fb0`. The sites were read, not run; the labels are inferred
from the read code. No pair is a decision only the box scalar makes
that steers what is built.

- **`pose_unit_norm` against `session_box_reach` (1,348),
  `mate_chart_slides_span` (625), the end of the log (18): one gate.**
  `Chart::charted` (`editor-core`'s `mate/value.rs`) renormalizes the
  carried frame under `POSE_UNIT_NORM` unless the chart is the identity
  or `T::spins_by_frames()`, which is `false` at `f64` and `true` at
  `Interval`. `f64` logs three entries the box lane never makes; the
  box lane's entry at that index is its next one. `session_box_reach`
  is decided by both lanes and steers nothing but a refusal.
- **`ssi_hull_sup` against `pcurve_projected_lever` (51): one lane's
  route, with a different count of rounds.** `limb2` switches on
  `T::WITNESS` (`geom-brep`'s `edge_nurbs.rs`): the exact scalars take
  the enclosure, `f64` an upper bound, and the lever decisions run per
  round until a read clears, which outward rounding moves.
- **`path_leg_length` against the `trace_plane_*` runs (21): the one
  pair whose extra pass builds something.** `ProfileLane::pass_one`
  returns the first pass's form at `f64` and `None` at the box scalar,
  where the profile resolves again with its traces; that resolution is
  the lane's own profile form.
- **`bool_point_in_solid_advance` against `…_denom` (28): no gate
  found.** Both are decided in `ray_crossings` with no scalar test in
  the file; an upstream difference (another ray after a graze, another
  hit set) is inferred, not shown.
- **`pcurve_chart_placement` turning between the lanes (2 builds, the
  tour's bracket).** `Placement::turns` (`geom-brep`'s
  `pcurve_cache.rs`): Positive takes one turn off the azimuth read,
  Negative keeps it. Inferred: the two lanes read the azimuth on
  opposite sides of the chart's cut and the turn undoes it, so both
  build the same pcurve (the records are equal, measured). The two
  margins were not measured. If that is right it is a logged choice
  that chooses a spelling, not a form: the class of
  `a-lane-choice-is-logged-as-a-choice-so-a-turned-one-refuses-a-leaf-the-general-lane-serves`.
- **The third class (a check run only `f64` makes, the plates)** is
  what `a-witness-records-curved-premise-checks-the-interval-replay-skips-so-no-curved-leaf-certifies`
  repairs.

**None is in play on a document the suite drives today:** replayed at
the leaves' own scalar over a zero-width box, 342 of 347 witnesses
equal the `f64` witness in the whole log, and the two plates differ in
check runs only. What is left to do here: find the gate of the
unlabelled pair, and measure the two `pcurve_chart_placement` margins.
(BOXFINISH orchestrator)


## The corpus at one point, no box (PR 7716's review, 2026-11-28)

The reviewer's measurement on PR 7716's head, taken again in its fix
pass with the reviewer's probe on that head merged with main
`3ff862c423`: an `f64` build and an `Interval` build of each document
at its authored point, their verdict vectors (`certifying_vector`)
compared. Of the 34 corpus documents and `cone_on_sphere` and `cup`,
25 are equal and 11 part; no document differs in a `props_*` count.

| document | first parting, `f64` against `Interval` | counts that differ, `f64`:`Interval` |
|---|---|---|
| `boss_union` | `curve_carrier_gap` against `curve_carrier_clear` | `curve_carrier_gap` 68:65 |
| `die_pips` | the same pair | `chart_singularity_in_span` 5:4, `chart_singularity_nearer` 6:5, `curve_carrier_gap` 8:7 |
| `die_tool` | the same pair | `chart_singularity_in_span` 30:24, `chart_singularity_nearer` 76:70, `curve_carrier_gap` 48:42 |
| `die_composed` | the same pair | `chart_singularity_in_span` 50:55, `chart_singularity_nearer` 42:30, `curve_carrier_gap` 33:32, `pcurve_cylinder_chart_axial` 0:36 |
| `die_fillet` | `chart_singularity_nearer` (check, Zero) against `chart_singularity_at_vertex` (check) | `chart_singularity_nearer` 32:23 |
| `die_composed_tour` | the same pair | `chart_singularity_in_span` 435:372, `chart_singularity_nearer` 850:739, `curve_carrier_gap` 358:337 |
| `tube_arc` | `ssi_hull_sup` against `pcurve_projected_lever` | `pcurve_projected_lever` 8:12 |
| `hollow_tube_elbow` | the same pair | `pcurve_projected_lever` 16:24 |
| `cone_on_sphere` | the same pair | `pcurve_projected_lever` 8:10 |
| `axis_projection` | `census_backstop_depth_toward` against `census_backstop_depth_across` | `bool_pair_gap` 326:479, `census_backstop_depth_toward` 6:0, `curve_pair_pose_clear` 53:0, `curve_pair_pose_held_one_piece` 30:0, `curve_pair_pose_parallel` 249:236, `curve_pair_walk_held_end` 177:147, `curve_pair_walk_span_order` 83:46 |
| `revolves_on_one_axis` | `curve_pair_walk_held_end` against `curve_pair_pose_basin` | `curve_pair_pose_held_one_piece` 12:0, `curve_pair_walk_held_end` 42:30, `curve_pair_walk_span_order` 24:12 |

Four of the five pairs are not in the first table above:
`curve_carrier_gap` / `curve_carrier_clear`,
`chart_singularity_nearer` / `chart_singularity_at_vertex`,
`census_backstop_depth_toward` / `census_backstop_depth_across`, and
`curve_pair_walk_held_end` / `curve_pair_pose_basin`. None is traced.
The `ssi_hull_sup` pair is
`limb-2-logs-other-decisions-at-an-enclosure-scalar-than-at-the-witness`
(ISOTAIL).

## The at-rest policy's lanes, read for what each scalar logs (PR 7716's fix pass, 2026-11-28)

Read, not run. In `topo`'s `props.rs` the `f64` and `Interval`
`AtRestPolicy` impls do not build a lane two ways: each of the four
lanes is `Some(its one constructor)` at one scalar and `None` at the
other, so what differs is a whole path present or absent.

- **`offset_fit_lane`: `f64` only** (`props.rs:7652` against `:8000`).
  `f64` logs `offset_normal_floor` (`geom-brep`'s `offset_meters.rs:586`)
  and `offset_curvature_headroom` (`offset_meters.rs:985`) once per rung
  of `meter_patch`'s ladder, per `Approx` face at tier 3's check 1
  (`validate.rs:7696`) and at the offset mint (`replace_face.rs:2461`).
  `Interval` logs nothing and refuses (`ApproxLaneUnsupported`,
  `validate.rs:7705`, `replace_face.rs:2457`). Not read: what
  `fit_offset_at` logs beyond the meters.
- **`section_lane`: `f64` only** (`props.rs:7722` against `:8070`).
  `f64` logs `offset_section_level_row` (`offset_derive.rs:1404`),
  `offset_section_sense` (`:1314`) and the section doors' `ssi_*` rows.
  `Interval` refuses a plane × spline or spline × spline pair
  (`NurbsLaneUnsupported`, `replace_face.rs:3605`); analytic pairs run
  `section_closed` at both scalars.
- **`ssi_lane`: `f64` only** (`props.rs:7729` against `:8077`). `f64`
  logs everything the march and the spline readings decide
  (`ssi_transversality`, `ssi_step_progress`, `ssi_closure_*`,
  `ssi_on_locus`, `ssi_tube_*`, `ssi_chart_domain`, `ssi_pair_*`, and
  the march's own limb 2, `ssi_hull_value` then `ssi_hull_sup`,
  `ssi/certify.rs:1172`, `:1187`). `Interval` refuses before any march
  (`CarryRefusal::NoLane`, `pair_section.rs:2093`; `NoMarch`,
  `spline_implicit.rs:705`). Not read: which march rows sit inside
  unlogged scopes.
- **`clearance_lane`: `Interval` only** (`props.rs:7754` against
  `:8103`). `Interval` logs one `clearance_margin` row, only where the
  sweep refuses `BoxDependent` under an upper bound (`editor-core`'s
  `eval/wire.rs:2733`); the sweep's own readings run under
  `k_stats::unlogged("F41")` (`clearance.rs:1649`). `f64` answers
  `NeedsCertifyingLane` (`wire.rs:2658`) and logs nothing.
- **`leaf` / `take`.** `f64`: `Leaf = AtRestBody`, the full finish runs
  at leaf time under a bracket whose record is held on the body and
  relogged by `take` (`props.rs:7774`–`7796`). `Interval`:
  `Leaf = Replay`, tier 2 only at leaf time, and `take` finishes a
  clone live (`props.rs:8119`–`8131`). So `f64`'s rows at a taker are a
  replay of what it decided at leaf time, and `Interval`'s are decided
  there afresh: the two are equal only where tier 3 itself logs alike
  at both scalars. A tier-3 refusal's record lands on the leaf node at
  `f64` and on a taker (or nowhere) at `Interval`; a moved leaf runs
  `T::leaf` again at `f64` (`finish.rs:659`) and no completion at
  `Interval`.

Outside the four lanes, the two sites the measured partings name:
`limb2`'s switch on the witness kind
(`limb-2-logs-other-decisions-at-an-enclosure-scalar-than-at-the-witness`),
and `pcurve_projected_lever`'s count on a line × cone row, whose parts
are halved on a detached spread reading that takes "spread" where it
is undecided (`pcurve_cache/projected.rs:3165`–`3204`), so a wider
value gives more parts and more rows. That the second is the cause of
a measured parting is inferred.

Not determined by this read: whether a body that reaches any of the
`f64`-only lanes can be driven over a box at all today (each refuses
at `Interval`, so such a leaf fails before its vectors are compared).
