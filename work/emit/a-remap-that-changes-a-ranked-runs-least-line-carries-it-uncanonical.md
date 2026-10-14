---
id: a-remap-that-changes-a-ranked-runs-least-line-carries-it-uncanonical
kind: issue
title: A remap that makes another line of a ranked EdgeCrossing run least carries the name out of order, so it resolves to nothing
status: open
opened: 2026-10-12
priority: P3
cost: M
design: true
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

## Question

The fix the row's Next leaves is a loud refusal, and nothing settles
what a remap may refuse. `names/README.md` (N1–N6) says nothing about a
rewrite of a published name. ASSEMBLY.md's A4 lists the split's and the
inline's refusals (a severed edge, `TornGroup`, a declaring mate that
would start placing, a cut of unplaced material) and holds the pair to
"name-resolution identity"; it lists no refusal for a name the pair
cannot carry. A refusal here is a new A4 refusal, so the choice is
Ev's, not the lane's.

**Measurements (as of `b41d2fe38f`).**
- Suite reach: none (the row's instrumented run: no pair-boolean
  `EdgeCrossing` group of more than one vertex in editor-core).
- Node ids: `refactor::carry` inserts the carried nodes in document
  order, which is id order, except that `gauges_first` moves a gauge
  ahead of the nodes on it. A gauge mints no edge, so the node map
  keeps the order of every node a run line can name.
- Step ids: `carry` builds the step map by zipping the old program's
  ids with the new ones in program order, and the insert door mints
  them in program order. A `SetProgram` keeps an old step's id at any
  position and mints a new step's id after every older one
  (`edit::settle_step_ids`), so a program can hold its ids out of
  order (steps `a1, c3, b2`), and its re-minted ids then reorder `b`
  and `c`. Two run lines minted by one node and told apart by their
  steps (two pieces of one profile's extrude reached as both operands
  of a pair boolean, a pass-through placing one of them) can swap.
  Not reproduced end to end; this is a reading of `carry` and
  `settle_step_ids`.
- Read ids (`ReadMap`): mapped port by port onto the outputs each
  carried insert mints, so they follow node order.
- The public door, `refactor::remap_name`, takes any caller's maps,
  so it can swap any two lines (the reproduction maps 1 → 20, 2 → 10).

**Options.**
1. **Refuse.** `SegRewrite` gains a typed hook for an `Unrankable`
   (default: carry, so `emit_union`'s two citing rewriters keep their
   behaviour), `Remapping` answers it with a new error beside
   `Unmapped` (say `Unmapped::Unrankable { name }`, or a new
   `RemapError` wrapping both), and the split and the inline surface
   it as `SplitError`/`InlineError` variants naming the reference. A4
   gains the clause "a split or inline refuses a name it cannot carry
   ranked". Cost: one public error shape, two refusal variants, a
   reachable refusal of a split the author did not ask to fail.
2. **Keep name order in the maps.** Make `carry` mint (or map) a
   profile's steps so the step map keeps id order, which with the node
   and read maps above makes the split's and the inline's maps
   order-preserving on every line a run can name. The pair then never
   meets the case, and only `remap_name` with a caller's own map can;
   that door would still need option 1 or a documented precondition.
   Cost: the part's step ids stop following its program order, which
   A4 does not pin but the inline-of-split "up to minted ids and
   order" clause reads loosely.
3. **Carry, diagnosed.** Keep carrying, and have N5 say why the name
   vanished (a remap-specific `Diagnosis`). Not loud at the split; the
   reference fails at its next resolution.

Lane `remap` recommends 2 for the pair plus 1 for the public door:
the pair is pure and owns its maps, so it can avoid the case rather
than refuse an author's split; a caller's own map cannot be steered,
so that door refuses.
