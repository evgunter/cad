---
id: the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start
kind: issue
title: The census's collinear edge-edge lane reads two edges' line offset at the second edge's start, so its overlap verdict depends on arena order: a body whose edges part by 21 bands reads an undeclared overlap
status: open
opened: 2026-10-04
priority: P0
cost: M
refs: [boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm]
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
`(4, 1)`: review r1's `shallow200 nt e0 a3 d1e-7`. Prism ∪ cube and
prism ∖ cube build at the exact volume, with tier 2 and the
certificate. Pinned by `sweep` `join_pierce_runs_sweep.rs`
`a_near_tangent_two_run_pierce_builds_with_no_edge_pair_in_band`.
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

The edges' nearby ends are 2.13e-7 apart, 21 bands. Every edge pair of
the body parts by more than 2.0e-7 (segment distance, in the pinned
row). **With the roles swapped**, the offset is read at the short edge's
start: 2.13e-7, not collinear, no finding. So the verdict depends on
which edge comes first in arena order.

**The converse.** When the long edge's start lies off the short edge's
line by more than the band, but the long edge passes within the band
along the short edge's span, the lane reads "not collinear" and misses
a real overlap. Arithmetic witness: a short edge of length 1e-3, and a
long edge of length 2 at 1e-7 rad to it, crossing its span. Parallel
reads 1e-10, so Zero. The offset at the long edge's far start reads
2e-7, so the lane says not collinear, though the edges run within 1e-10
of each other along the short one. I did not build a closed body that
holds such a pair, so whether a construction can reach it is unmeasured.

## The shape to give

Read the offset where the edges are near each other: both of the
shorter edge's endpoints against the longer edge's line, levered as the
parallel test is. The verdict is then symmetric in the pair. Rerun the
pinned row with tier 3′ asserted. The near-tangent battery of PR 4026's
review (`r1_pierce_probes`, `R1_NT_D`) is the corpus to diff.
