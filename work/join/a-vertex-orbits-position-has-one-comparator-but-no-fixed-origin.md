---
id: a-vertex-orbits-position-has-one-comparator-but-no-fixed-origin
kind: issue
title: insert.rs reads a vertex orbit's position through one comparator from three origins, and b_runs' nesting is a separate integer reading
status: open
opened: 2026-10-05
priority: P3
cost: M
branch: join/insert-one-walk-order
---


## What

The residue of `a-vertex-orbits-walk-order-and-run-rule-are-spelled-several-times`,
which PR 4061 carried. PR 4061's review (S1, S5) found it, and it is
disclosed there.

PR 4061 folded the position readings into one comparator,
`insert::walks_before(sectors, origin, p, q)`, and the run rules into
`walk_run`. Two things the consolidation row owed are not done:
- **One fixed origin.** The comparator takes its origin as a parameter,
  and three remain: the orbit's first entry (`walk_order`), a run's
  first entry (`held_cut`), and a physical sector's first entry
  (`precedes`, `holds_whole`). `precedes`' origin still moves with its
  first argument and is never checked against the second's sector. All
  its callers compare cuts of one physical sector, but nothing asserts
  that. A total order from one fixed origin, with each caller's
  interval read off it, would make the origin a property of the orbit
  rather than of the call.
- **The nesting as an interval of the walk.** `b_runs` reads B's runs
  as integer intervals of the survivors' walk positions (`spans`,
  `interval`, `holds`). It is a separate reading from the cut order the
  hold tests use (`held_cut`, `holds_whole`, `nests`).

Also, `crates/topo/src/boolean/vtxfac.rs` steps a sector array back by
`(k + n - 1) % n` at `germ_of((run.0 + n - 1) % n)`, `germ(.., start_germ)`
and `sectors[(runs[i].0 + n - 1) % n].arm`. Whether those readings are
the same position order as `insert.rs`' is unchecked. `zip.rs`' `within`
is `walks_before`'s cross-entry formula verbatim, but it stays out
while PR 4057's design question is open.

## Built (branch `join/insert-one-walk-order`)

- **One fixed origin.** `insert::walks_before(sectors, p, q)` reads the
  orbit forward from its first entry and takes no origin. Entry 0 opens
  a physical sector (`sectors::build_sectors` emits each corner's
  edge-bounded entry first), so no physical sector wraps past it, and
  the three former origins (the orbit's first entry, a run's, a
  physical sector's) read one order. The order refuses an orbit whose
  first entry opens no physical sector, so the precondition is checked
  rather than assumed.
- **`precedes` is gone.** `leads`, `strut_anchor`, `walk_faces_first`
  and `holds_whole` call `walks_before`. Its origin no longer moves
  with an argument, so "both cuts of one physical sector" no longer
  needs checking for the order to be right.
- **One interval reading.** `insert::on_arc(before, [lo, hi], x)`
  reads where `x` lies on the arc from `lo` to `hi`, off any one total
  order, wrapping past the origin where `hi` comes before `lo`.
  `held_cut` reads it over cuts, and `arc_holds` (below) over cuts or
  walk positions. A plan's walk positions are `walk_order`'s ranks of
  the same order.
- **`vtxfac.rs`' `(k + n − 1) % n` steps.** These read the same entry
  array in the same direction, but they compare no positions. Each one
  steps from a run's first bound to the sector before it, an adjacency
  between a bound and a sector, so there is no comparator to fold in.
  `recl.rs`' `flankers` and `reflank` are the same kind of step.
- **`zip.rs`' `within`** is no longer in the tree, so there is nothing
  to fold.

## Measured (release, main 047d10d5 against branch head 8b03369e)

0 lines moved, across 89 battery runs:

| battery | lines |
|---|---|
| `pierce_runs_battery` | 4 537 |
| `pinch_runs_battery` | 3 025 |
| `corner_pairs_battery` | 16 381 |
| `join1_r1_reflex_battery` | 1 153 |
| `j3r2_r1_reflex_battery` | 1 153 |
| `rc_wide_battery`, all 84 shards | 40 320 `RCW` lines |

Every run exited 0 on both sides. The whole workspace suite (debug,
12 951 tests) passed on head except one wall-clock row. That row reads
the same time on main and is filed as
`a-names-alike-bound-sits-at-the-debug-builds-own-time`.
