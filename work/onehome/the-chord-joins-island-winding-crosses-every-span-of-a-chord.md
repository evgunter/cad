---
id: the-chord-joins-island-winding-crosses-every-span-of-a-chord
kind: issue
title: The chord join's island winding takes about 1.9 s of a 5 s unequal-radius cylinder boolean in release, in the curve-pair walk's whole-carrier foot and an unkept acceleration bound on 630-pole chords
status: open
opened: 2026-11-10
priority: P1
cost: M
refs: [a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses]
---

## What

Filed by SPIRICARM's cylinder-pair cost lane (branch
`spiricarm/cylinder-pair-cost`, 2026-11-10), measuring the P1 row
`a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses`.
Re-measured by ONEHOME's chord-join-cost lane on main `77cf43f7b`
(2026-11-14).

On the unequal-radius pose (unit cylinder against a `r = 0.6` cylinder
across it, axes meeting; `the_fenced_poses_keep_their_own_outcomes` in
`sweep/tests/verbs_germarms2.rs`; `cargo run -p sweep --release
--example cylinder_pair_cost_probe unequal`), an operation takes
4.9–5.5 s in release. The chord join reads an island's winding twice
per operation (`topo::boolean::join` `IslandClosing::Carried` →
`chord_join::path_island_winding_closed_by`), 1.05–1.22 s and
0.75–0.82 s: about 1.9 s of each. The skew pose reads none.

**Where it goes** (timing probes, not committed). 1.84 s of the 1.9 is
40 calls of `geom_brep::curve_pair::meet` from `ring_path.rs`
`spline_crossings`: a path's ruling or circle-arc piece against the
island's spline arc, a chord of 630 or 631 poles, which the certified
walk reads (`curve_pair/walk.rs` `meet`):

| Where | Time |
|---|---|
| `Read::of`: `NurbsCurve3::accel_upper_bound`, about 33 ms on the chord, computed afresh at every meeting (the speed bounds beside it are memoised) | 0.66 s |
| the walk's stop: `Curve3::foot` on the spline, 1110 stops, each sampling all 630 knot spans before polishing | 0.85 s |
| the walk's ends and witnesses: the same whole-carrier foot | 0.28 s |
| the rest of the walk and evaluation | 0.05 s |

The row's first reading (2.6–3.2 s of a 15–17 s operation, `perf` in
`ring_path::path_parity` → `spline_crossings` under
`geom_core::ladder::climb_agreeing`) predates the work that brought the
operation to 5 s.

## Why it matters

A third of each unequal-radius cylinder boolean, on ordinary geometry
(the P1 row's measured cost).

## Re-banded P1 (2026-11-11)
By SPIRICARM's orchestrator, under the blocker rule (`work/README.md`, Priority): `spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses` (P1, about 10 s per ordinary cylinder boolean in release) is parked on this row and ONEHOME's/PCERT's sibling, which are the remainder of its cost after PCERT's chord hulls (re-measured there).
