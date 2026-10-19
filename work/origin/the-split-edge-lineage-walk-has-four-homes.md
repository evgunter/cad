---
id: the-split-edge-lineage-walk-has-four-homes
kind: issue
title: The SplitEdge lineage walk is written four times, each with its own stop, bound and cycle refusal
status: open
opened: 2026-09-30
priority: P1
cost: M
refs: [a-split-half-loses-the-lineage-of-a-twice-crossed-edge, 5086]
---


## What

Four loops chase an edge's `Provenance::SplitEdge` records parent to
parent:

- `topo::Body::split_root` (`crates/topo/src/provenance.rs`): one body,
  a caller's stop predicate, bounded by the edge count, refusing
  `SplitLineageCycle`;
- `editor-core` `names::borders` (the obstacle chase, ~`borders.rs:216`):
  one body, falling back to the boolean's `dead_split` rows for a key
  with no record, with its own bound and `Emission` refusal;
- `topo::boolean::discard` (~`discard.rs:99`): one body, collecting
  the whole chain, detecting a cycle by `chain.contains`;
- `editor-core` `names::emit_topo::chase_split_edge_to_table`: a
  split's two halves, reading each hop's record from whichever half
  holds the key.

Each loop is the same walk with a different place to look up a key's
record, a different point to stop at and a different refusal. When
the records do not all live in one body, each loop has to find that
out for itself. That is how the split's naming lane came to stop at a
key the other half held
(`a-cylinder-split-refuses-missing-upstream-once-its-pieces-rank`), and
`a-split-half-loses-the-lineage-of-a-twice-crossed-edge` is the same
gap in the readers still left.

## The fix

Add one walk in `topo` that takes a record-lookup closure and a stop
predicate, with one bound and one cycle refusal. `split_root` becomes
the case that looks up its own body, and the other three become calls
to it. Priced M: `discard.rs` wants the whole chain, not the root, so
the walk has to yield the chain or visit each hop.

## The copies the walks read beside the records (`fuse/split-hop-lineage-at-append`, 2026-10-14)

Three naming tables copy `SplitEdge` records out of a body before a
kill drops them:
- `SplitNaming::joined_lineage`, written by `SplitNaming::record_joins`
  (`crates/topo/src/splitting/finish.rs`);
- `BooleanNaming::killed_lineage`, written by `edge_join::join_stage`
  (`crates/topo/src/boolean/edge_join.rs`) and, for the edges the
  merge kills, by `boolean::ops::finish_output`;
- `DiscardRow::boundary_chains` (`crates/topo/src/boolean/discard.rs`).

The Split's `joined_lineage` and the Boolean's `killed_lineage` take
their records the same way (`edge_join::split_records`) and share one
writer, which asserts that an edge never gets two parents
(`edge_join::record_killed_lineage`).
None of the three is a list of mints, since each is written in kill order and a
join can kill a child before its parent; each is as acyclic as the
records it copies, and the argument is stated once, on
`Provenance::SplitEdge` (`crates/topo/src/provenance.rs`). So the
cycle refusals left in the readers belong to this row, and the one walk
it asks for should take the copies as another record lookup. As
filed (the next section says what they do now):

- `emit_topo::chase_edge_to_table` (~`emit_topo.rs:99`), which steps
  through the chains' hops and the boolean's `killed_lineage`
  (`KilledSplits`) and refuses
  `SplitLineageCycle` past its hop bound;
- `emit_topo::chase_split_lineage` (~`emit_topo.rs:191`), which steps
  through `joined_lineage` where a half holds no record, and refuses
  `SplitLineageCycle` past its edge-count bound;
- `borders.rs`'s `Obstacles::up_lineage` (~`borders.rs:280`), which
  reads `split_from` (each body's records merged with the chains' hops
  and with each boolean's `killed_lineage`, in `Obstacles::record`)
  and refuses `bug("a piece edge's split lineage is cyclic")`.

Inherited from `join-desync-is-the-catch-all-for-cyclic-lineage-records`, beyond the readers: nothing checks the `SplitEdge` records where `Body::split_edge` writes them, and the three copies (`SplitNaming::joined_lineage`, `BooleanNaming::killed_lineage`, `DiscardRow::boundary_chains`) are `pub` fields a caller can write. The fix checks acyclicity where the records are written, or says at the write site why it cannot. It also decides whether the copies stay writable from outside the kernel. Until both hold, the readers' bounds rest on the writer-access argument rather than a check; the `editor-core` readers now panic on them (below), and `Body::split_root` and `discard.rs` still refuse typed.

## The editor-core readers share one walk now (`wire/one-chain-chase`, 2026-10-19)

`emit_topo::chase_edge_to_table`, `emit_topo::chase_split_lineage` and
`borders.rs`'s `Obstacles::up_lineage` now chase through one helper,
`names::chase::to_root`. It takes a step closure and a bound, and so
does `emit_topo::pair_original` over a split's null-pair rows. When a
walk outlasts its bound, the helper panics (D9 row 4) and does not
refuse typed. The emitters read only rows the kernel's own split or
boolean wrote, so in `editor-core` a cycle is a kernel bug. The panic
message names the rows and why they cannot cycle.

`Body::split_root`'s own `SplitLineageCycle` still passes through
`chase_edge_to_table` and `chase_b` as `NamingError::SplitLineage`.
That is `topo`'s published refusal and stays with this row. When the
`topo` walk this row asks for lands, `to_root`'s callers can take it
as their step.
