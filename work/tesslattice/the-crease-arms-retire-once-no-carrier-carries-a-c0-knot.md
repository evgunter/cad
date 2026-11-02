---
id: the-crease-arms-retire-once-no-carrier-carries-a-c0-knot
kind: issue
title: every reader's crease arm retires once no face or edge carrier carries a C⁰ knot at rest
status: parked
opened: 2026-10-28
priority: P1
cost: M
refs: [loft-of-three-or-more-sections-at-degree-one-is-unmeshable]
blocked_on: [tier-3-refuses-a-carrier-with-an-interior-c0-knot]
---


## Ruling

Ev's ruling on [ev] PR 5913 (2026-10-28, Decision 2: "S, then"), now DESIGN [topology]: every face chart and edge carrier is C¹ by its knot structure strictly inside its cell; a construction whose carrier is not puts an edge or vertex there; import splits at its door; no reader carries a crease arm.

## Owed

Once the tier-3 rule holds, nothing at rest carries a C⁰ knot. Each crease arm is then unreachable; delete it, do not generalise it. The designers' inventory:
- `geom-brep` `patch_bound`: the C¹ gate in `check_direction`, `patch_cells_across_creases`, and `PatchBoundError::{Degree1Crease, Crease, DerivedKnots}`. The patch bound becomes one per-span door, `patch_cells`.
- `mesh::trimmed`: `Creases`, and its two-direction refusal.
- `mesh::chords`: the arc-rim count rounding, `sub_arc_joint`'s mesh uses, and the polyline and kink refusals.
- `NurbsCellGrid::cert`: the crease answer of +∞, which becomes an assertion.
- `topo` boolean: `NormalAtSplineCrease`, `sector_face`'s crease arm, and the curve×face crease cuts.
- `offset_fit`: its C¹ gate.
- `geom_core`: `C1Break`'s two names for one fact.
- splice: the rows `a-seam-on-a-creased-spline-chart-reads-no-arm`, `a-creased-spline-gets-no-second-order-clearing` and `a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease` become unreachable. Close them, naming this row.

Each crate's part may be its own unit. This row spans ENCL, NURBS, SPLICE, topo's boolean programs, and TESS's successors; announce the seam on each.

Filed by the TESS orchestrator at the ruling.
