---
id: chorded-walls-reach-the-work-wall-short-of-the-pinned-resolution-at-1e-12
kind: issue
title: Spline-chorded walls reach the measurement's work wall short of the pinned resolution (1024·ε) at ε 1e-12
status: open
opened: 2026-11-06
priority: P2
cost: M
---


Found by the ONECOPY seams lane (PR 6550, `onecopy/seams-1e-12`).

## What

At `CAD_TOLERANCE_EPS=1e-12`, two sweep rows' bodies have spline-chorded walls, measured by the rotational lane under `topo::test_support::pinned_resolution` (1024·ε = 1.02e-9 m of mean boundary displacement). Every run of both stops at `Stop::Wall`, the work wall, short of that resolution:

| row | runs | stop | volume half-width |
|---|---|---|---|
| `parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region`, turned pose about `x` | all six ops | `Wall` | `3.50e-8` – `4.02e-8` |
| `tang_circle_cylinder::a_tilted_rod_through_a_rim_builds_at_its_sliced_volume` | ∖, ∩ | `Wall` | `1.49e-6` |
| same | ∪ | `Wall` | `1.26e-7` |

At ε 1e-9 the same runs meet their resolution. Every enclosure holds its closed form, so this is a cost frontier, not a wrong number. The rows admit it off the witness ε as `Verdict::Unmet`, under measured guards (`TURNED_WALL_GUARD`, `TILTED_ROD_WALL_GUARD`; `sweep/tests/common/differential.rs` `judged_at_resolution`).

## Owed

Find which face or faces stop at the wall at 1e-12, and why: a chord count that grows past the work budget, or a refinement that stops paying. Then:
- either the measurement meets 1024·ε within its budget at 1e-12,
- or the floor is stated where `Settle::Resolution` is documented, so a row can name it rather than measure a guard.

When either lands, lower or drop the two guards.

## Same class, not this row's

A witness-ε test is spelled as a literal across the suites: `tol.eps() != 1e-9` and `(zero - 1e-9).abs() < 1e-12` in `parallel_cylinder_join.rs` and `tang_circle_cylinder.rs`, and the same shape in `nurbs_import.rs` (about :150 and :295) and `sign_walk_plus_v.rs` (about :331). One helper (a `witness()` test on `Tol`, or in `common`) would hold them.

The work-wall cost of the cylinder-pair rows is in `work/spiricarm/a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses.md`. That row is about time, not about the resolution being met.
