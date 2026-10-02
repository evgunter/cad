---
id: sphere-seam-in-a-plane-face-loses-the-fallback-recut-to-the-tilted-section-refusal
kind: issue
title: A sphere whose seam meridian lies in a plane face now takes the crossings path and refuses the tilted plane×sphere section, where the no-crossings re-cut answered
status: open
opened: 2026-09-28
priority: P2
cost: M
refs: [coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep]
---


## What

A revolved ball's seam meridian and both its poles lie in a plane face
of the other operand when the ball is poled along an axis lying in that
plane: `cube(1) ∖ ball` with the ball `r = 0.3` poled along `y` and
centred at `(0.5, 0.5, 1)`, on the cube's top face.

Before `coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep`, the
sweep's conic × plane lane skipped a conic lying in the face's plane, so
this pair had no events: the reduction found no crossings,
`boolean/ops.rs`'s sphere extent scan saw the ball escape through the top
face, re-charted it about that normal, and the re-entered op answered
`1 − (2/3)π·0.3³` exactly (valid body, `point_in_solid` right in the
dimple, below it and above it). The sweep now records both poles on the
top face, so the reduction takes the crossings path, the re-cut never
runs, and the join refuses `Join(SectionInvariant)` in
`chord_join.rs` — "plane×sphere section tilted against the sphere
chart's polar axis". Every op of the pair refuses there.

The events are right: the poles lie in the face, and premise S of the
section certificate needs them recorded. What is lost is the answer the
fallback's re-chart gave, which the crossings path has no counterpart
for.

Held, refuse-or-answer-correctly, by
`crates/sweep/tests/germ_coplanar_conic.rs`
`every_op_refuses_or_answers_its_closed_form` ("y-poled pip on the
cube's top face"); `m5_pr9c_sphere_doors.rs`
`the_die_pips_shape_stops_typed_at_its_pierce_ring` pins PR 9c's
smoke shape, the same class, at the join's refusal.

## What the taker owes

The crossings path re-charts a sphere group whose section with a plane
face is tilted against its polar axis (as the fallback's re-cut does),
or the join's tilted-section arm learns the section; either way the
y-poled pip answers its closed form.

## Evidence (2026-10-02, `reach/tilted-sphere-pair`)

The join's tilted-section arm now selects the arc (the run-side rule,
`chord_join::select_arc_by_run_side`), and the y-poled pip and the
die-pips shape both move one step on: every op refuses
`Join(SectionArcSide { case: NoCertifiedRun })`. The run the chord is
handed carries only null scaffolding — the poles land on the face as a
pierce ring, the routine source `ArcWindowCase::NoChartedRun` already
names for the window rule — so what this row now waits on is the ring's
own chord lane (`tang/pierce-ring-has-no-join-arm`) or the re-chart.
