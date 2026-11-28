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

## 2026-11-27: PR 7801 also gives a wrong material answer (more serious than the slowdown)

HELDRECORDS's orchestrator bisected one more row, in the same way: `sweep::all germ_coplanar_conic::every_op_refuses_or_answers_its_closed_form`.

| first-parent commit | result |
|---|---|
| `9a25655c18` (PR 7799) | PASS |
| `2fc64636e46` (PR 7811, PR 7801's first parent) | PASS |
| `1bb20a9fce1` (**PR 7801**) | **FAIL** |
| `0bd424091e2` (PR 7813) | FAIL |
| `7ce370250a8`, `d23272dea5` | FAIL |

The failure on main, as PR 7808's evidence lane reported it, reads `outer equator arc in the box top: A ∪ B: point_in_solid Point3 { x: 0.0, y: 0.2, z: 2.5 } = Ok(Out)`. That is a **wrong material answer**, not a refusal: containment says Out where the row's closed form says otherwise. This is the worse half of this row. Fix it first, or revert PR 7801 until it can be fixed.

## 2026-11-28: the mesh-budget job now hangs to GitHub's 6 h cut-off

HELDRECORDS's `heldrecords/interior-minted` lane (PR 7832) saw the PR gate's `mesh budget meter + certificate falsifier` job cancelled at 6 h on two heads, a258c91 and f9a6cc7. The test that sticks is `quadric_pair_sections::a_loops_edges_read_simple_knots_at_every_scale` (`quadric_pair_sections.rs:288`).

The lane ran that test locally, in release with the budget feature and a 1500 s cap. It **timed out on main `91f1ee7873` as well as on its head**, so the hang is main's. Evidence is on https://github.com/evgunter/cad/pull/7832#issuecomment-7240767550. It has not been bisected to PR 7801. If it turns out not to be PR 7801's, split it into its own row.
