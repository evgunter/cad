---
id: a-vertex-orbits-walk-order-and-run-rule-are-spelled-several-times
kind: issue
title: insert.rs spells a vertex orbit's walk order three times and the run that holds no third germ twice
status: closed
opened: 2026-10-05
priority: P1
cost: M
closed: 2026-10-05
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

## Since (the six-crossing pairing)

`b_runs` is a third run rule. A pair not adjacent in B's walk order
runs forward from its earlier position to its later, and its run then
holds another pair's run whole (a nested matching, six crossings or
more). So "the run that holds no third germ" now holds only in A, and
in B only for adjacent pairs. The one position type should carry the
nesting too: an interval of the walk, read from one origin.

## Built

- **One position order, `insert::walks_before`.** It counts entries from
  an origin entry, then uses `walks_after` within one entry. Each former
  spelling now reads it, and differs only in the origin it names:
  - `walk_order` reads from the orbit's first entry;
  - `held_cut` reads from the run's first entry, strut or fan, through
    one between-test;
  - `precedes` reads from its physical sector's first entry, which
    `edge_bound_entry` finds, the same walk as `next_edge_bound`'s.

  The hold predicates (`nests`, `holds_whole`, `nested`, `run_ends`,
  `strut_anchor`) read it through `precedes`. `holds_whole` keeps its
  own between-test, and a comment says why: a tie counts inside there,
  and reading from `lo` would wrap.
- **One run rule, `insert::walk_run`.** A and B read it alike, for every
  survivor count. Adjacent germs run the way that holds no third
  (`run_order`). For two survivors either way is a run, and
  `run_degenerates` breaks that tie. Other pairs run as an interval of the
  walk. Strut-ness is one helper, `is_strut`.
- **What stays, and why:**
  - `run_degenerates` in `reconcile_pass` asks whether a way round can
    mint at all, not which way to run.
  - `walk_faces_first` orders two germs in one entry by `strut_order`
    from the arrival edge; a unit row checks that it agrees with
    `walks_after`.
- **Not done:** one fixed origin, and `b_runs`' nesting read as an
  interval of the same walk. Both are filed as
  `a-vertex-orbits-position-has-one-comparator-but-no-fixed-origin`.
- **Behaviour:** byte-identical between main and head on
  `pierce_runs_battery`, both reflex batteries, all 84 `rc_wide` shards,
  `pinch_runs_battery` and `corner_pairs_battery`.
