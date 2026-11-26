---
id: main-slow-sweep-rows-run-ten-to-twenty-times-longer-since-ffc8c37d4
kind: issue
title: Several slow-set sweep rows run 10-20x longer on main 1ec3a6a6c0 than on ffc8c37d4 (82 s to over 1800 s), cause not bisected
status: open
opened: 2026-11-26
priority: P2
cost: M
---


Found by the `heldrecords/minted-door` lane's per-test evidence runs
(release profile with debug assertions, 4 cores, `nextest --profile
default`).

## What

The same rows, the same machine, the two merge bases the lane compared:

| row (`sweep::all`) | `ffc8c37d4` | `1ec3a6a6c0` |
|---|---|---|
| `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall` | 90 s | > 1800 s, still running |
| `germ_plane_torus_frame::a_general_pose_builds_to_its_volume` | 235 s | > 1260 s, still running |
| `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` | 162 s | > 960 s, still running |
| `a_closed_conic_in_the_partners_face::a_tube_through_a_ball_across_its_wrap_edge_builds_every_op` | 69 s | > 590 s on the base alone; > 1200 s in the full run |

The lane's own diff (PR 7808) runs as slowly on top of `1ec3a6a6c0`, so
the cost came with main.

## Candidates, not bisected

The first-parent merges between the two bases:

- 7782 (`heldrecords/one-list`);
- 7801, 7813 and 7814 (INSIDE's containment arms, which read spline
  faces in the ray test). These rows read their material by
  `point_in_solid` at hundreds of probes.
- 7806, 7807, 7810, 7811, 7812, 7815, 7816 and 7817.

## Done when

The merge that slowed them is named, and either it is made cheap again
or the rows' slow-set cost is re-measured and accepted.
