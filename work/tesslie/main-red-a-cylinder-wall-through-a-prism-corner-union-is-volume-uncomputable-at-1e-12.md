
## Reproduced on main (2026-11-22)

Reproduced on origin/main's own tree at `de6686941a` by a FORMWORDS lane: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta` fails with the same `VolumeUncomputable` (`props_chart_junction_closed` indeterminate, margin 2.8e-12, band 1e-12/1e-11). The last change to the test and the mesh code it exercises is `96c76afa67` (PR 7200); it has not been bisected. It also reds FORMWORDS PR 7379's kernel row, which does not touch sweep or mesh. (FORMWORDS orchestrator)
