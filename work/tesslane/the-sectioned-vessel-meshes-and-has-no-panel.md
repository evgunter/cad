---
id: the-sectioned-vessel-meshes-and-has-no-panel
kind: issue
title: the tour's sectioned vessel meshes now and has no panel
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by `one-chart-table-and-the-lane-takes-every-analytic-chart` (branch `tesslane/chart-table`).

`demos/tour/src/torusvessel.rs` carried wall 1: `tessellate` of the sectioned vessel's hollow (the bellied meridian revolved a quarter turn and shelled), pinned by the trimmed lane's chart-roster refusal, with the instruction "retire this probe and ship the sectioned vessel as this scene's third panel". The trimmed lane now meshes the hollow's spiric-bounded torus wall on the torus chart's row, so the probe is retired and `torusvessel::sectioned` builds the body; `demos/tour/tests/torusvessel.rs::the_sectioned_vessel::the_sectioned_vessel_meshes_to_its_measured_volume` pins that it meshes, passes `check_mesh` and encloses its measured volume within `δ·area`.

What is not done is the panel. A new `TourStop` adds a row to `docs/guide/north-star-audit.md` (its roster is compared with the tour's stops by `crates/pncad/tests/all.rs::the_north_star_audit_has_a_row_for_every_tour_stop`, and its headline counts re-derived), a `demos/README.md` row, and a `[render]` commit for the new cell. The unit that closed the refusal moved no frame and left the scene's panels as they were.
