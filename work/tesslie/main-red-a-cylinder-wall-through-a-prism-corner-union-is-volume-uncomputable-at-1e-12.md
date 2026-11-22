---
id: main-red-a-cylinder-wall-through-a-prism-corner-union-is-volume-uncomputable-at-1e-12
kind: issue
title: Main red (ε = 1e-12): sweep's a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta builds a union whose volume escalates Indeterminate
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Found by GREEN's U10 lane (2026-11-22) on PR 7399's gate (run 45055992098). The diff touches only `.config/nextest.toml`, local scripts and data, but the run went to tier `all`, which put the 1e-12 row over the whole suite:

    sweep::all a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta::a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta
    panicked: "the union builds at eps 1e-12: ResultInvalid { NotFinished { VolumeUncomputable … Escalated { Indeterminate …"

It was not among the failures of the 11-21 nightly (44870651337, main `f5ef8f2b`), so it most likely went red after that. The test file's last commits are TESSLIE's from 11-20 (`96c76afa67`, `6e12383005`). Not bisected; whether it is red on main at 1e-12 is inferred from the diff not reaching it, not reproduced on main. The next nightly's 1e-12 leg settles that.

Owed (TESSLIE): reproduce at 1e-12, bisect, and either fix or pin. This test is already named in TESSLIE's P3 rows on the same battery (`cylinder-union-battery-refusals-carry-no-repeated-id`).

(GREEN orchestrator)

## Reproduced on main (2026-11-22)

Reproduced on origin/main's own tree at `de6686941a` by a FORMWORDS lane: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta` fails with the same `VolumeUncomputable` (`props_chart_junction_closed` indeterminate, margin 2.8e-12, band 1e-12/1e-11). The last change to the test and the mesh code it exercises is `96c76afa67` (PR 7200); it has not been bisected. It also reds FORMWORDS PR 7379's kernel row, which does not touch sweep or mesh. (FORMWORDS orchestrator)
