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

The rank cannot be re-read where the two orders agree: that needs the
geometry, and a remap does not have it. Until the fix below lands, the
name is carried and silently stops resolving.

## What settles it

**Split and inline: the maps keep name order.** ASSEMBLY.md's A4
*Acceptance* requires split-then-evaluate to equal unsplit evaluation
"at structural and name-resolution identity", and inline-of-split to
return the document "up to minted ids and order". A reference that
stops resolving across a split breaks that, and so would refusing a
split that is valid. The pair owns its maps, so it makes them keep the
order of every line a run can name. Then no ranked run's least line
moves, and nothing needs re-ranking.

**The public door: a typed refusal.** `refactor::remap_name` takes a
caller's own maps, which nothing can steer. Under fail-loud, a rank
that cannot be re-read is refused, not carried silently.
`SegRewrite` gets a typed hook for an `Unrankable`. By default it
carries the name, so `emit_union`'s two citing rewriters keep their
behaviour. `Remapping` answers the hook with an error beside
`Unmapped`, and `remap_name` returns it.

## Where the pair's maps lose name order (as of `b41d2fe38f`)

- **Node ids:** kept. `refactor::carry` inserts the carried nodes in
  document order, which is id order. `gauges_first` moves a gauge ahead
  of the nodes on it, but a gauge mints no edge.
- **Step ids:** not kept. `carry` zips the old program's step ids with
  the new ones in program order, and the insert door mints them in
  program order. A `SetProgram` keeps an old step's id at any position
  and mints a new step's id after every older one
  (`edit::settle_step_ids`), so a program can hold its ids out of
  order (`a1, c3, b2`), and re-minting then swaps `b` and `c`. Two run
  lines minted by one node and told apart by their steps can therefore
  swap. This is a reading of the code; it has not been reproduced end
  to end.
- **Read ids (`ReadMap`):** mapped port by port onto the outputs each
  carried insert mints, so they follow node order.

## Next

1. Make the split's and the inline's step map keep id order, so the
   part's step ids are minted in the order the old ids sort, not the
   program's. Pin it with a split of a `SetProgram`-reordered profile
   whose ranked run would otherwise swap lines.
2. Give `SegRewrite` / `canonical::rewritten` the typed channel, and
   have `remap_name` refuse with it. Re-pin
   `refactor::remap_a_ranked_run` as the refusal.
3. If a reach to (1) turns up a reorder that does not come from step
   ids, the pair should refuse it loudly rather than carry it.
