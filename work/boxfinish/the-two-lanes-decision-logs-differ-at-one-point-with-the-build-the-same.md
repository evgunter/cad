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
