---
id: a-vertex-orbits-walk-order-and-run-rule-are-spelled-several-times
kind: issue
title: insert.rs spells a vertex orbit's walk order three times and the run that holds no third germ twice
status: open
opened: 2026-10-05
priority: P1
cost: M
---


## What

Found by PR 4036's dual review (r1 S1, S2; r2 Q1).

**Three spellings of "position round the vertex"**, all in
`crates/topo/src/boolean/insert.rs`:
- `walk_order` orders survivors by entry index, then by `walks_after`
  within one entry, and refuses a tie.
- `precedes` compares two cuts relative to the first edge-bounded
  entry at or before the first cut, then uses `walks_after` within one
  entry. It returns `None` on a tie. Its origin moves with its first
  argument, so it is not a total order across physical sectors, and
  `walk_order` cannot call it.
- `held_cut` has its own `rel` (entries relative to a run's first end).

Around them sit the file's other order and hold predicates: `nests`,
`holds_whole`, `nested`, `tied_held` and `run_ends`. That is about
twelve in all for one vertex.

**Two spellings of "the run that holds no third germ".**
- `run_order` takes it from the walk order when a pair has more than
  two survivors.
- `run_degenerates` takes the forward run unless it swallows the whole
  orbit, when there are two survivors.

## Owed

One position type per vertex orbit: an entry, and a direction within
it, against one fixed origin. Have the walk order, the cut order and the
hold tests read it, and read the run rule off it for every survivor
count. The two survivors' tie-break may stay; it is the one place where
either way is a run.
