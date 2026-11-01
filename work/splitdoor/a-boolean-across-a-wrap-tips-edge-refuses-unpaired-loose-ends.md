---
id: a-boolean-across-a-wrap-tips-edge-refuses-unpaired-loose-ends
kind: issue
title: A boolean whose brick crosses a wrap-tip hemisphere's wrap edge refuses UnpairedLooseEnds in every op, where the same brick across a cylinder's seam builds
status: open
opened: 2026-10-31
---


Found by PR 6097's review (F5) and probed in its fix pass. The body is a
hemisphere whose two pole meridians were made one wrap edge from the
equator to the pole (`kef_describing`, then `join_edges`), finished
through `topo::test_support::finished`.

A brick across the wrap edge, `x ∈ (−2, 2)`, `y ∈ (0.5, 2)`,
`z ∈ (−2, 0)`, refuses `subtract` with
`Join(BooleanJoin(UnpairedLooseEnds { count: 2 }))`. The review saw
`union` and `intersect` refuse the same way.

For comparison:
- a brick across a plain extruded cylinder's seam (also a wrap edge,
  but one with no tip), `x ∈ (0.5, 2)`, `y ∈ (−0.5, 0.5)`,
  `z ∈ (0.5, 1.5)`, builds;
- the same brick clear of the seam builds.

So this is not a seam-crossing defect. It belongs to the wrap tip: the
section crosses an edge whose one end is a valence-1 vertex at the pole,
and the join is left with two ends it cannot pair.

No door mints a tip yet, so nothing is live. It becomes live with the
first door that does (`import-mints-the-wrap-tip-at-a-chart-singularity`,
CARVE's revolve). The probe is `probe_tip_operand_booleans` on
`restcurve/review-lantern-6097` (`crates/sweep/tests/review_probe_6097.rs`).

Signed: (RESTCURVE implementer)
