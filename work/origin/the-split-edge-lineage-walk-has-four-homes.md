---
id: the-split-edge-lineage-walk-has-four-homes
kind: issue
title: The SplitEdge lineage walk is written four times, each with its own stop, bound and cycle refusal
status: open
opened: 2026-09-30
priority: P1
cost: M
refs: [a-split-half-loses-the-lineage-of-a-twice-crossed-edge, a-cylinder-split-refuses-missing-upstream-once-its-pieces-rank]
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
