---
id: co-surface-direction-table-and-spiric-plane-door
kind: unit
title: Build the co-surface arm's direction table (transversality rule, one edge-section reading) and the curve_face::spiric_plane door (section reduction), per C5 as ratified in PR 7301
status: review
opened: 2026-11-21
priority: P1
cost: H
parent: curve-surface-crossing-has-no-kind-pair-table
pr: 7431
branch: germ/co-surface-table
---


Ratified by Ev in PR 7301 (2026-11-21). C5's co-surface sentence and the curve × face paragraph's door list state the design (`crates/geom-brep/README.md`).

## Build
1. **`curve_face::spiric_plane`, by section reduction.** For `Q ∥ P`, the pose is decided by tilt (levered by R + r) and then offset: `Lies` or `Apart`, escalating in band. Otherwise `L = P ∩ Q` is read against the spiric's torus by the built line × torus door, over the stretch of `L` on the oval's side of the axis's projection on `P`. Each root's tube angle is the span's parameter. `route`'s (Spiric, Plane) row becomes built, and its "no producer reaches a boolean with one" note goes. Consumers move onto it: the plane split (`splitting::classify`), the slit's `RIDER_SPIRIC_PLANE`, and the planar walk's ray × oval (`splitting::spiric_arc`).
2. **The co-surface arm's direction table** in `curve_pair::on_surface`, one row per curve-family pair, on the transversality rule. The default reads the edge across the leg's plane. On the torus, a parallel leg against a planar edge, and any leg against a spiric edge, read the leg across the edge's plane. Parallel × Villarceau is fixed and escalates in band. The edge's section (which oval; which of a meridian and its mirror; a Villarceau circle or its mirror) joins the leg's in one `Section` type. `region::spiric_landings`, `Legs::on_edge_plane` and topo's kind switch go. The ring path's `ArcLoop` copy goes when the ring path moves.

Rows: the crest probe (a parallel leg at v = π/2 − d against a meridian and against an oval) builds at every d; the ten plane × torus rows of `the-region-reads-a-spiric-edge-by-the-leg-across-its-plane-not-by-the-spiric-plane-arm` keep building; each table row reds its mutant.

Depends on U3b (PR 7299) merging first.
