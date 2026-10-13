---
id: a-remap-that-changes-a-ranked-runs-least-line-carries-it-uncanonical
kind: issue
title: A remap that makes another line of a ranked EdgeCrossing run least carries the name out of order, so it resolves to nothing
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## What

Found in review of the vertex-run build (branch `emit/vertex-run`).
N2 ranks several vertices with one `EdgeCrossing` run along the run's
least line, and the run is in name order in every name, a pair
boolean's included. A rewrite of a published name whose id map does
not keep name order (`refactor::remap_name` for a split or an inline:
`gauges_first` moves a gauge ahead of the nodes on it) can make another
line least. The rank was measured along the old least line, and no
name says where the vertices lie along the new one, so
`canonical::along_run` answers `RankRule::RefuseRun`, and
`canonical::rewritten` carries the name exactly as the segment rewrite
left it: its run out of order. No emitter mints that spelling, so the
re-mapped reference resolves to nothing where it resolved before.

**Reproduction:** `refactor::remap_a_ranked_run::a_remap_that_swaps_a_ranked_runs_lines_leaves_it_out_of_order`
(a pair boolean's ranked run of two edges minted by nodes 1 and 2,
mapped 1 → 20, 2 → 10).

**Reach:** no document in the suite publishes a ranked run. An
instrumented full editor-core run (every pair-boolean `EdgeCrossing`
group of more than one vertex) found none. Two straight edges cross
once, so a ranked run needs curved edges crossing twice with equal
senses.

## Next

Either re-read the rank where the two orders along the lines agree
(needs the geometry, which a remap does not have), or refuse the
rewrite loudly: `SegRewrite` has no channel for an `Unrankable`, so
`rewritten` would need one. Until then, the name is carried and
silently stops resolving.
