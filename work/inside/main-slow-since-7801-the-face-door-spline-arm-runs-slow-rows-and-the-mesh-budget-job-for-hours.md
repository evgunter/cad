---
id: main-slow-since-7801-the-face-door-spline-arm-runs-slow-rows-and-the-mesh-budget-job-for-hours
kind: issue
title: Main slow since PR 7801 (the face door's spline arm): slow-set sweep rows run several times longer and the PR gate's mesh-budget job runs for hours, holding every PR's gate ok
status: open
opened: 2026-11-26
priority: P0
cost: M
refs: [7801]
---


Filed by HELDRECORDS's orchestrator, which bisected the first-parent merges on main. The slowdown was found by HELDRECORDS's `heldrecords/minted-door` lane on PR 7808, whose own copy of this finding, raised to P0, is on its branch.

## Bisect

The witness is `sweep::all cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall`. Each commit was run on its own in the default `cargo nextest` dev profile, from one target directory, on a 4-core box, with a 900 s cut-off:

| first-parent commit | merge | time |
|---|---|---|
| `ffc8c37d4c` | | 378 s |
| `cf9aa2d062d` | PR 7807 (after 7806) | 383 s |
| `2fc64636e46` | PR 7811 (after 7810) | 374 s |
| `1bb20a9fce1` | **PR 7801** `inside/face-door-spline-arm` | **> 900 s, killed** |
| `7ce370250a8` | PR 7816 | > 2100 s, killed |

So PR 7801 is the merge that slowed it. Its head message: "The face door reads spline arcs: curve_face::cross answers the ring-composite arm at T: Decide". That merge was not bisected further into its own commits.

## What it holds

- **Slow-set sweep rows run 10–20× longer on the minted-door lane's release runs.** Examples:
  - the row above;
  - `germ_plane_torus_frame::a_general_pose_builds_to_its_volume`;
  - `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`;
  - `a_closed_conic_in_the_partners_face::a_tube_through_a_ball_across_its_wrap_edge_builds_every_op`.

  These rows read their material through `point_in_solid` at hundreds of probes.
- **The PR gate's `mesh budget meter + certificate falsifier` job** (`ci.yml`, the step `mesh --features budget`) took about 8 min before this merge. Since, it runs for hours: PR 7808's runs went over 40 min, one was cancelled at 2 h, and PR 7658 stalls in the same step. The job sets no `timeout-minutes`, so it waits out GitHub's 6 h default. **No PR's `gate ok` can conclude until this is fixed.**

## Done when

The face door's spline arm costs what it did before on these rows and in the mesh-budget job, or the cost is measured, bounded and accepted. The mesh-budget job also gets a `timeout-minutes`, so that a hang fails fast rather than holding every PR for 6 h.
