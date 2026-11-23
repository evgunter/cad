---
id: the-trimmed-sphere-arm-triangulates-a-raw-uv-lattice-in-the-chart-metric
kind: issue
title: the trimmed lane's sphere arm lays a lattice sized for a raw-UV Delaunay lemma and triangulates it in the chart metric at mid latitude, where cells reach 10.7:1 near a pole; no defect demonstrated
status: open
opened: 2026-11-23
priority: P4
cost: M
---


Found by the review of `tesslane/one-slit-band` (PR 7498). The measurement below is the reviewer's, not re-run on this branch.

## What

The trimmed lane's sphere arm lays its lattice on `sizing::sphere_grid_steps`' steps (`crates/mesh/src/sizing.rs` `SPHERE_CELL_ASPECT`, line 616), which hold the raw UV aspect at most √2 for a raw-UV Delaunay containment lemma. The arm triangulates that lattice in a different plane: the chart's own metric at mid latitude (`crates/mesh/src/chart_table.rs` `AnalyticRow::trimmed_metric`, line 485), `k = 2^round(log₂(1/cos v_mid))`. At latitude ≥ 78° that is k = 8 to 16, and the lattice's cells enter the CDT at aspect down to 0.093 (10.7:1), against the lemma's 2√2 ≈ 2.83. So the lemma the steps were chosen for does not hold in the plane the arm triangulates in; the certificate is what holds its triangles.

## What is measured (the reviewer's)

- 153 trimmed sphere faces in the `mesh` suite: all near the equator, none outside the lemma's premise.
- 90 oblique-trimmed polar caps (a ball tilted 0.03 / 0.1 / 0.2 rad about z, intersected with the slab y > sin(lat), lat 55° to 85°, δ 3e-2 to 3e-4): all mesh watertight. Sampled worst 0.54 δ under main's metric, 0.82 δ with `cell_metric` swapped in.

No defect is demonstrated, and swapping in the lattice's metric is not the cure (it measures worse).

## What closing it takes

Say which theorem holds the arm's triangles in its own plane. Neighbours: `the-trimmed-sphere-arm-has-no-worst-triangle-theorem` (the arm's missing worst-triangle theorem) and `one-containment-lemma-for-an-oblique-boundary` (the containment lemma every chart lacks for an oblique run). Either supplies the bound, or the arm's plane and its steps are chosen together.
