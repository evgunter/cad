---
id: census-reads-a-line-edge-in-a-curved-face-only-between-two-planes
kind: issue
title: The census cuts a line edge lying in a curved face at the face's vertices only, leaves its outside touch unrecorded, and reads no spline face
status: open
opened: 2026-10-30
priority: P3
cost: M
---


Found by SPLITDOOR's keel-on-a-ruling triage (branch
`splitdoor/keel-ruling-wedge`), which added the census's curved arm
(`crates/topo/src/census.rs`, `sweep_edge_in_curved_face`,
`curved_overlap_cells`, `edge_in_curved_face`). The arm reads a line
edge lying in a cylinder or cone face of its own solid against the
face's sheets and material direction, and refuses
`FaceThroughEdgeWedge` where the face runs through the edge's wedge
(`crates/sweep/tests/a_keel_on_a_bores_ruling.rs`,
the one-edge row). What it does not read:

1. **A wall tangent to the face** — closed by PR 6061's second fix
   pass: the arm reads every wall, and a tangent one reads its side in
   band and refuses as `FaceThroughEdgeWedgeUndecided`.
2. **A crossing of the face's boundary away from its vertices.** The
   edge is cut at its ends and at the face's vertices on its interior
   only, and each cell is read at its midpoint; pass 4's planar lane
   also cuts where a boundary edge crosses (`boundary_crossings`). A
   curved trim edge crossing the ruling inside both edges leaves a cell
   straddling the boundary, read by its midpoint alone.
3. **A touch from outside.** A wedge clear of the face's material is
   a legal touch that needs an edge-on-face record; the arm leaves it
   to the records and reports no `UnrecordedContact`, where pass 4's
   planar lane does (`ef_overlap_lane`, `ef_bound_backed`).
4. **Spline faces.** A face on a `Nurbs` or `Approx` carrier has no
   implicit residual, so the arm does not read it; the Boolean's rider
   reading (`boolean/slit.rs`, `lies_in`) holds it nothing either.

The fix for each: a reading of the case, or a typed refusal
(`CensusUnsupported`) where none exists, never a silent pass.
