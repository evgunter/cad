---
id: main-red-a-cell-off-a-face-decides-no-torus-cell-at-1e-12
kind: issue
title: main red: sweep cell_region_bodies::a_cell_off_a_face… is red at ε 1e-12 (the torus decides no cell), with no open row
status: open
opened: 2026-11-26
priority: P2
---


Filed by the LATHE orchestrator. LATHE's one-band lane measured this while running the full slow set over its stack (PR 7414) against main.

The `sweep::all cell_region_bodies::a_cell_off_a_face…` row (`crates/sweep/tests/cell_region_bodies.rs`) is red at `CAD_TOLERANCE_EPS=1e-12` on main `5e04ac6561`: the torus decides no cell. It is green at the default ε and at 1e-6. LATHE's head reds it the same way. No open row names it. EMBEDWALK last touched the file (d24d492bec, 18d98b52b2).
