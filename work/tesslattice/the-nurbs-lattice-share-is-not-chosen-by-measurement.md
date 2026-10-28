---
id: the-nurbs-lattice-share-is-not-chosen-by-measurement
kind: issue
title: the NURBS lane's LATTICE_SHARE = ½ picks a point on its sliver theorem's curve that nothing measured as the coarsest
status: open
priority: P2
cost: M
opened: 2026-10-25
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
did not move; nothing measured that ½ is the cheapest point. The tour's
68 NURBS faces (tess-budget sweep at PR 5717's head) reach a sampled
deviation of at most 0.061 δ, so the lane sits far under δ wherever no
sliver binds.

**What would settle it.** The tess-budget sweep's NURBS rows
(`scripts/tess_budget_sweep.sh`) at shares ½, ¾ and 1, comparing
triangle totals and refusals; the cheapest share that refuses nothing
new is the lane's coarsest step under the rule. A share that moves the
NURBS output re-cuts the baseline and the NURBS render cells.

**Re-banded P2 (PR 5717's review, 2026-10-26).** NURBS is 84% of the
tour's triangles, and its faces reach at most 0.061 δ, so the share is
the largest coarsening left on the table. The reviewer adds two points:
- `split_steps` scales only with the budget, so the cheapest share may
  need no new machinery. A per-band coarsest `b = 4δ/a²`, read off each
  band's own residual aspect `a`, puts that band's sliver at δ without
  touching the schedule's shape.
- `LATTICE_SHARE`'s doc says "every point of that curve puts the worst
  triangle at δ". That holds only at the line. A band whose residual
  aspect is under `√MAX_RESIDUAL_SQ` has its sliver below δ at any
  share, so the curve's other points are not equivalent for it.


## Moved to TESSLATTICE (the NURBS lattice: its share, its rows and cells, and what it costs on swept and lofted walls) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
