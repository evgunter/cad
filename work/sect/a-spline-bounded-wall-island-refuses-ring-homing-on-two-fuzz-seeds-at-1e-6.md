---
id: a-spline-bounded-wall-island-refuses-ring-homing-on-two-fuzz-seeds-at-1e-6
kind: issue
title: The spline-bounded wall island fuzzer draws two seeds at 1e-6 that refuse RingHomingAmbiguous where the ball decides the bystander
status: open
opened: 2026-11-02
priority: P2
cost: M
---


## Finding (RESTCURVE, PRs 6163 and 6244; routed here by GREEN, 2026-11-02)

The counterexample search
`chord_join::wall_ring_rows::a_spline_bounded_wall_island_re_homes_its_bystanders_as_the_ball_says`
(`crates/topo/src/chord_join/wall_ring_rows.rs:546`, added with the path
re-homing in `c2531dff4`, "sect: a cylinder face re-homes its rings by
the path") fails on main at ε 1e-6 under two seeds. Both fail with
`RingHomingAmbiguous` from the re-homing after the closing chord's `mef`:

| seed | failure |
|---|---|
| `0x9cb9baff9ceb8aeb` | "asked 12, escalated 0: centre `(0.5, 0, 0)`, cuts `[0.3149597220281418]`: `RingHomingAmbiguous { ring: LoopKey(5v1) }`" |
| `0x49504906104860f0` | "asked 12, escalated 0: centre `(0.7, 0.1, -0.2)`, cuts `[0.5519863698563332, 0.8011729840077771]`: `RingHomingAmbiguous { ring: LoopKey(3v1) }`" |

Reproduce:

```
CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0x9cb9baff9ceb8aeb CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo -E 'test(=chord_join::wall_ring_rows::a_spline_bounded_wall_island_re_homes_its_bystanders_as_the_ball_says)'
CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0x49504906104860f0 CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo -E 'test(=chord_join::wall_ring_rows::a_spline_bounded_wall_island_re_homes_its_bystanders_as_the_ball_says)'
```

The first reproduced on main `336465c3` in a clean worktree. The second
reproduced with main's `torus_region.rs` (as on main `71584a65`) swapped
into RESTCURVE's branch. The test draws its seed per run, so it goes red
only on runs that draw a seed like these. The evidence was first recorded on GREEN's
`main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps`
(closed by the PR that filed this row).

It is on SECT by authorship of the search. `scripts/work.py territory`
names no owner for `crates/topo/src/chord_join/wall_ring_rows.rs`, and
SECT wrote the test and the path re-homing it exercises.

## What closing it takes

Find why the re-homing cannot place the ring at these two poses, where
the ball's membership gives the bystander's side. Then either decide it
or refuse with a type the search accepts. By the discipline's fuzzer rule, each seed becomes a
pinned deterministic row beside the fix.
