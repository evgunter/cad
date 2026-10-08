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
