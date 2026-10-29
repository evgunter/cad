---
id: the-door-proves-a-constructed-tangency
kind: unit
title: INTENT stage 6 PR D: the door proves the tangency row a trace-tangent construction makes
status: parked
opened: 2026-10-29
priority: P0
cost: M
design: true
blocked_on: [a-sketch-continues-tangent-to-a-surfaces-trace]
---


INTENT stage 6, PR D. Spec: `docs/INTENT-STAGE6-SPEC.md` §5.

The `SurfaceTangent` row between a face C's step drew and the face it traced is `Structural`. Today the boolean's tangency records a sampled margin (`DecisionSite::TangentWitness`, `topo/src/boolean/mod.rs` `tangent_row`), and no rung proves a `SurfaceTangent` row.

`design: true` for **FORK-S6-4** (spec §7): a margin the symbolic tier can read, a first-rung statement over the sweep, or nothing new. Under the third, D folds into C and closes with it. Cost M, H if the fork takes a new rung. Must keep the slice-tangency caveat: a sphere's small-circle trace extruded normal to its plane cuts the sphere and is proven tangent by nothing.
