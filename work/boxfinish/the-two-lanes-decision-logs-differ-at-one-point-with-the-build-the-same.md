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

