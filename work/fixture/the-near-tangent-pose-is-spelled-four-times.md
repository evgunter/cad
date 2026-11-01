---
id: the-near-tangent-pose-is-spelled-four-times
kind: issue
title: The near-tangent prism-corner pose is built four times in tests and the probe, and the copies drift
status: open
opened: 2026-10-09
priority: P4
cost: E
---


## What

Found by PR 4415's first review (S3).

The pose "a prism's corner `v` on a cube's face tilted `d` rad off one
of the corner's edges" is built in four places, each with its own
vector helpers:
- `crates/sweep/examples/near_tangent_census_probe.rs` (`basis`,
  `near_tangent`, `cube_poly`);
- `crates/topo/src/boolean/offer_rows.rs` (`basis3`, `clip3`,
  `SliverLump`);
- `crates/topo/tests/sliver_shell_role.rs` (`notch307_meet`);
- `crates/sweep/tests/join_pierce_runs_sweep.rs` (`near_tangent_corners`, `near_tangent_frame`), beside `crates/sweep/tests/common/pinch_cones.rs`.

The copies drift. `offer_rows`' `basis3` did not normalize its axis
where the probe's `basis` does. PR 4415 made it normalize, but nothing
holds the copies to each other.

## The shape to give

Put one pose builder (the frame, the cube, the notch profiles) in a
test-support module both crates reach, and route the four sites
through it.
