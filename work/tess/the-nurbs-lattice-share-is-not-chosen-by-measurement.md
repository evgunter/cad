---
id: the-nurbs-lattice-share-is-not-chosen-by-measurement
kind: issue
title: the NURBS lane's LATTICE_SHARE = ½ picks a point on its sliver theorem's curve that nothing measured as the coarsest
status: open
priority: P3
cost: M
opened: 2026-10-25
refs: [mesh-sizes-each-lane-by-its-worst-admissible-triangle, S29]
---

Filed by PRED's `mesh-sizes-each-lane-by-its-worst-admissible-triangle`
unit. `crates/mesh/README.md` `[mesh.sizing]` asks each lane for the
coarsest step its theorem grants without extra work.

The NURBS lane's theorem (`mesh::nurbs_cert::LATTICE_SHARE`'s doc) puts
its worst admissible triangle, the off-row sliver of residual aspect
`a`, at `a²·b/4` for a lattice sized on the budget `b`, and a lattice
cell-half at `b/4`. With `b = LATTICE_SHARE·δ` the sliver lands at δ
whenever `MAX_RESIDUAL_SQ = 4/LATTICE_SHARE`, which is how both are now
written. Every share on that curve meets the rule's "worst triangle at
δ"; the share trades a coarser lattice (larger `b`, every band's steps
grow as `√b`) against more bands whose rows `band_schedule` raises
(smaller `a²`). The unit kept `LATTICE_SHARE = ½`, which is the value
the retired `sizing::sizing_target` gave this lane, so the NURBS output
did not move; nothing measured that ½ is the cheapest point.

**What would settle it.** The tess-budget sweep's NURBS rows
(`scripts/tess_budget_sweep.sh`) at shares ½, ¾ and 1, comparing
triangle totals and refusals; the cheapest share that refuses nothing
new is the lane's coarsest step under the rule. A share that moves the
NURBS output re-cuts the baseline and the NURBS render cells.
