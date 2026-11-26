---
id: mesh-budgets-slow-rows-read-two-band-revolves
kind: issue
title: mesh's slow rows (the mesh-budget job's cargo test) still read two-band revolves, and three of them changed outcome
status: closed
opened: 2026-11-23
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
cost: M
closed: 2026-11-26
pr: 7556
---


Found by the LATHE one-band lane (2026-11-23, PR 7414), on its branch
`lathe/full-revolve-one-band`.

## What

CI's `mesh-budget` job runs `cargo test -p mesh --features budget`, which
runs every mesh row, the slow set's included, which the nextest `ci`
profile filters out. Eleven of them fail on the branch (303 passed). Run
locally with that command. The rows are slow-set rows
(`.config/nextest.toml`), not budget-only ones: the nightly runs them,
and so does a PR's slow step wherever the diff spells one or edits its
file (`scripts/slow_set.py`, `selected`).

**Outcome changes** (not counts; each needs a reading before a re-pin):

- `reach_probes::a_needle_funnel_cut_at_its_apex_refuses_typed`: the
  needle funnel cut at `h = 1e-3`, above, `δ = 1e-3` now refuses
  `FoldedTriangle` where it meshed (`None`).
- `reach_probes::a_cone_face_met_on_its_axis_meshes_at_every_delta`: the
  union no longer builds: `ResultInvalid` with `VolumeUncomputable`,
  `ConeLoop { what: "props_cone_loop_vertex" }` on the one-face cone.
- `quadric_pair_sections::a_ball_and_a_rod_against_a_donut_build_on_periodic_loops`:
  ball ∪ donut refuses `FallbackExtentUnsupported` (a closed loop interior
  to both faces): likely
  `a-section-loop-inside-a-one-band-curved-face-refuses-the-interior-loop-guard`'s
  class, not checked.

**Re-baselines the one band owes** (counts or fixtures encoding two
π-bands): `reach_probes::a_cone_face_through_its_apex_meshes_at_every_scale_and_pose`
(nappes `[-1, 1]` where it pinned two faces on each);
`reach_probes::a_sphere_face_bounded_by_a_spline_section_refuses_in_the_chord_pass`
(its fixture no longer bounds a sphere face by the projected section);
`split_rulings::a_split_side_adds_one_triangle_per_point` (cells 330 → 285)
and `every_close_split_of_a_draft_cones_rulings_meshes` (560 → 504);
`winding_rows::the_cut_frustum_meshes_with_thin_triangles_resolved`
(226 → 225); `lift_probes` × 3 (the per-face tables: faces, seamed faces,
rings, loops through a singular row).

## Ground

`crates/mesh/tests/{reach_probes,split_rulings,winding_rows,lift_probes,quadric_pair_sections}.rs`,
and wherever the two outcome changes lead (the cone loop's props read,
the trimmed lane's fold check). Tesslane's territory.
