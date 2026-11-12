---
id: the-chord-joins-island-winding-crosses-every-span-of-a-chord
kind: issue
title: The chord join's island winding takes 2.6–3.2 s per unequal-radius cylinder boolean in release, crossing every span of 625-span chords
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

On the unequal-radius pose (unit cylinder against a `r = 0.6` cylinder
across it, axes meeting; `the_fenced_poses_keep_their_own_outcomes` in
`sweep/tests/verbs_germarms2.rs`; `cargo run -p sweep --release
--example cylinder_pair_cost_probe unequal`), the chord join reads an
island's winding twice per operation, and the two reads take 2.6 s (∪),
2.9 s (∩) and 3.2 s (∖) of a 15–17 s operation in release (timing
spans, wall). The skew pose reads none.

`perf` places it: `topo::boolean::join::RoleLane::resolve` →
`ring_run_ccw` → `chord_join::path_island_winding_closed_by` →
`ring_path::path_parity` → `spline_crossings`, under
`geom_core::ladder::climb_agreeing` (15% of all CPU samples). The paths
are spline chords of 625–628 cubic spans, and the crossing count is read
over every span.

## Why it matters

A fifth of each unequal-radius cylinder boolean, on ordinary geometry
(the P1 row's measured cost).

## Re-banded P1 (2026-11-11)
By SPIRICARM's orchestrator, under the blocker rule (`work/README.md`, Priority): `spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses` (P1, about 10 s per ordinary cylinder boolean in release) is parked on this row and ONEHOME's/PCERT's sibling, which are the remainder of its cost after PCERT's chord hulls (re-measured there).
