---
id: main-red-the-side-readers-aims-take-a-circle-foot-since-point-walk-floors
kind: issue
title: main red (probe roster): side_reader_aims_ask_nothing's rod aims take a circle foot since PR 6645's circle floor reads an azimuth offset
status: open
opened: 2026-11-09
priority: P2
cost: E
---


Filed by `onecopy/main-reds-k-and-m10`, from the K probe sweep's plain half.

## What

`crates/sweep/tests/side_reader_aims_ask_nothing.rs`,
`the_aims_at_cylinder_walls_take_no_circle_foot` (`--features probe`,
rostered in the sweep's plain half), fails at ε 1e-9: `rod: the aims take
a circle foot`. The row counts samples recorded under `FOOT`
(`azimuth_near_cut_side`, `period_near_mark`, the F31 names) while
`topo::test_support::each_aim` aims at every face of the rod, and requires
none.

## Where it began

Bisected first-parent over main from `76b92357c9` (where the row landed,
green) to `eb5b460c8c`: merge `ce67c0dc9a`, PR 6645
(`onecopy/point-walk-floors`). That PR's circle floor reads `(azimuth
offset from the arc's middle − half span)` through `distance_within`, and
`edge_clearance`'s circle arms read it; its own measurement table lists
`side_reader::loop_aim` among the readers. So the aims now reach an
azimuth read the row says they never take. The PR's body does not name the
row, which runs only under `probe`.

## The question

Either the aims' clearance read of a circle edge is allowed to take the
foot now (the row's claim moves, and it should say what it guards
instead), or the aim path should read the circle's whole-circle clearance
without an azimuth (the row stands, and the read moves).

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo test -p sweep --features probe
--test all -- side_reader_aims_ask_nothing::`.
