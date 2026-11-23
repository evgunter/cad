---
id: a-cone-and-a-spline-cylinder-keep-the-rows-a-cylinder-face-no-longer-lays
kind: issue
title: A near-cylinder cone and a spline-stored cylinder keep the lattice rows a cylinder face no longer lays
status: open
opened: 2026-11-23
priority: P3
---


The chart lane lays no rows on a cylinder face since PR 7526 (`mesh/src/chart_table.rs`, `AnalyticRow::lattice_rows`, the cylinder arm), which took the notched and holed pipe walls from 1.2 to 223 times the rectangle lane's triangles to 1.0–1.5 times. Two faces that are a cylinder in all but name keep the density that unit removed:

- **A cone of half-angle near zero** keeps its graded rows (`lattice_rows`, the cone arm). The unit kept them on one probe that took rows off cones and saw cones refuse (PR 7526's stage-1 lane-wide run: `reach_probes::a_cone_face_bounded_by_a_section_ellipse_meshes_on_the_cone_row`, `a_cone_face_through_its_apex_meshes_*`, the funnels, `a_cone_wall_holding_a_ring_meshes_on_the_cone_row`, CertificateExceeded in the development with no correction on the cone row). No row pins why a cone needs its rows, nor how a near-cylinder cone compares.
- **A cylinder stored as a spline** (a lofted circle's wall) takes the NURBS lane's per-cell lattice (`mesh/src/trimmed.rs`, `per_cell_candidates`), its rows set by the spline's Hessian budget, not by the chart.

**Expected to close with** the lane-wide correction, `a-refused-certificate-adds-a-point-and-no-face-sizes-below-delta` (`tesslane/certificate-decides`): with the correction on every row, a cone's rows can be measured off the same way the cylinder's were.

**Measurement owed:** the cone's rows off with the correction on, over the `mesh` suite and `sweep`'s meshing modules at three ε: refusals, correction rounds, and triangles against the rows, per face family (near-cylinder cones of half-angle 0.01° to 5° named apart); and the lofted circle's wall against the analytic cylinder of the same radius and height at the same δ.
