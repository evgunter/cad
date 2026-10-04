---
id: the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start
kind: issue
title: The census's collinear edge-edge lane reads two edges' line offset at the second edge's start, so its overlap verdict depends on arena order: a body whose edges part by 21 bands reads an undeclared overlap
status: open
opened: 2026-10-04
priority: P0
cost: M
refs: [boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm, two-copies-of-a-pierce-carry-edges-that-run-within-the-band]
---


## What

Found by PR 4026's review r1 (M1) and the fix pass that traced it.

`crates/topo/src/census.rs` `pair_edge_edge` decides "parallel" by
`|ea.dir × eb.dir|` levered at the shorter edge's length. On Zero it
calls `ee_collinear_lane`, which decides "collinear" by the offset of
`eb.p0` from `ea`'s line (`pm_census_ee_line_gap`) and then reads the
projected overlap. The two readings are taken at different places.
Parallelism is read over the shorter edge. The offset is read at
`eb`'s start, which may lie a long way from where the edges are near
each other.

**The witness.** The shallow prism (profile (0,0),(4,0),(4,1),(2,0.6),(0,1),
height 1) with its reflex corner `v = (2, 0.6, 1)` on a cube of side
4, whose face plane lies 1e-7 rad off the corner's edge toward
`(4, 1)`: review r1's `shallow200 nt e0 a3 d1e-7` (the pinned row
scales the tilt with the band, ten bands, so it is this pose at the
default ε). Prism ∪ cube and
prism ∖ cube build at the exact volume, with tier 2 and the
certificate. Pinned by `sweep` `join_pierce_runs_sweep.rs`
`a_near_tangent_two_run_pierce_builds_with_edges_in_band_only_at_its_copies`.
Tier 3′ refuses them `UndeclaredContact { EdgeEdgeOverlap }`:

- `ea` is the prism top edge's last piece, `(4,1,1)` →
  `(3.99675640036, 0.99935128007, 1)`, length 3.3e-3.
- `eb` is a section edge, `v` →
  `(3.99675641195, 0.99935128239, 0.99999978721)`, length 2.036. It
  runs 1.045e-7 rad off `ea`'s line.
- Parallel: 1.045e-7 × 3.3e-3 = 3.5e-10, so Zero.
- Collinear: `eb.p0 = v` lies exactly on `ea`'s line, so the offset is
  0, so Zero.
- The projected overlap is 1.2e-8, which is above the band
  (zero 1e-9, escalate 1e-8), so the census reports an undeclared overlap.

The edges' nearby ends are 2.13e-7 apart, 21 bands, so this pair is
not an overlap. The body does hold edges within the band elsewhere:
the union keeps the pierce's two copies apart at `v`, and the two edges
that leave them run within the band for a stretch. That is a different
pair and a different class, filed as
`two-copies-of-a-pierce-carry-edges-that-run-within-the-band`.
Outside the copies, every edge pair parts by more than the band
(segment distance, in the pinned row). **With the roles swapped**, the offset is read at the short edge's
start: 2.13e-7, not collinear, no finding. So the verdict depends on
which edge comes first in arena order.

**No converse.** This lane cannot miss a real overlap silently (PR
4026's delta review, claim 1). A real collinear overlap has each bound
at a vertex of one edge, lying on the other edge or on its vertex.
Census passes 1 and 2 (`pair_vertex_vertex`, `pair_vertex_edge`) read
each such vertex's own offset, with the same backing rungs as
`ee_bound_backed`, so they report what this lane misses. The review
executed that check over the near-tangent batteries: no body the head
passes at tier 3′ holds an interior in-band edge pair, an unrecorded
in-band vertex pair, or an in-band vertex–edge pair that main does not
also ship.

## The shape to give

Read the offset where the edges are near each other: both of the
shorter edge's endpoints against the longer edge's line, levered as the
parallel test is. The verdict is then symmetric in the pair, and the false positive goes.
That door does not reach
`two-copies-of-a-pierce-carry-edges-that-run-within-the-band`: there the
two edges leave one point on two vertices and diverge, and what passes
them is the parallel reading levered at the full arm. Rerun the pinned
row with tier 3′ asserted. The near-tangent battery of PR 4026's
review (`r1_pierce_probes`, `R1_NT_D`) is the corpus to diff.
