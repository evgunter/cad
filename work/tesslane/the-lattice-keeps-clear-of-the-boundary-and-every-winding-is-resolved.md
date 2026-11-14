---
id: the-lattice-keeps-clear-of-the-boundary-and-every-winding-is-resolved
kind: unit
title: the trimmed lattice keeps a clearance from the boundary, and tessellate refuses a face whose triangle f64 cannot wind
status: parked
opened: 2026-11-14
priority: P1
cost: H
blocked_on: [6966]
---


Waits on Ev's answer to PR 6966, which puts `crates/mesh/README.md` `[mesh.winding]` and the clearance bullet of `[mesh.sizing]` to them (`docs/DESIGN-FORK-LOG.md` row 269). Specced when the answer is in; what follows is what the three designers agreed an implementer needs.

## What it is

- **The clearance.** On the trimmed lane, a lattice node is laid only where the box `BOUNDARY_CLEARANCE` (1/16) of its own step each way, per chart axis, meets no side or point of the trim polygon and no crease line. Applied once, before the retry loop, to lattice nodes only; ear and refinement points are centroids of a kept triangle and are not filtered. On the spline arm the step is each band's own, and that arm sizes its worst sliver at δ exactly, so it is measured under the rule before the rule is applied there.
- **What retires.** The exact-duplicate drop in `TrimmedFace::insert_points` and the node-on-a-side retry in `constrain_boundary` become unreachable for lattice nodes (the designers differ on whether they become typed refusals or stay as backstops: the unit says which and why). `Creases::cut_candidates` stops moving nodes onto a crease; the snap of boundary points onto a crease stays, and is the same class (`CREASE_MERGE`).
- **The winding.** One definition in `mesh::validate`: a triangle is resolved when its cross product exceeds `cert::rounding_allowance` at its coordinate scale times its longest side. `tessellate` reads it once per triangle where each face's patch joins the mesh (the `Ok(placement)` arm of its placement loop), on every lane and for a memo hit, and refuses the face typed under a new variant. `TrimmedFace::emit`'s `normal·outward > 0.0` reads the same definition and answers outward, folded or unresolved. A public check beside `check_volume` re-derives it; the suites call it in place of `reach_probes`' `MIN_SHAPE`.
- **The STL writer's doc** cites the promise (`the-stl-writers-doc-names-a-contract-nothing-states`).

## Measure first

The designers' numbers are in `work/tesslane/log.md` (2026-11-14, "Design fork, three designers"): the clearance table over 486 faces, the 40.6-million-triangle census, and the two frusta. The unit reproduces them before it builds, and names every golden that moves (D9's trimmed bodies, `docs/tess-budget-data`, the tour's pins) with why its new mesh is right.

## Closes

`a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area` (filed on PR 6960).
