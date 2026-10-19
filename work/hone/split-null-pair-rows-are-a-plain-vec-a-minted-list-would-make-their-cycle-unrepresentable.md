---
id: split-null-pair-rows-are-a-plain-vec-a-minted-list-would-make-their-cycle-unrepresentable
kind: issue
title: SplitNaming::vertex_pairs is a plain Vec, so a cycle names::pair_original must panic on is representable
status: open
opened: 2026-10-19
priority: P4
cost: M
refs: [split-and-boolean-copy-chains-are-chased-three-ways]
---



## Finding

From `wire/one-chain-chase` (2026-10-19), asking D9's row 0 of the
cycle `editor-core`'s `names::emit_topo::pair_original` guards.

`SplitNaming::vertex_pairs` (`crates/topo/src/splitting/finish.rs`) is
a `pub Vec<(VertexKey, VertexKey)>` of `(copy, original)` rows. The
split's finish writes it from the reduction's null-edge records
(`r.attr.copy_at(r.at_vertex)`), so each copy is a key its null edge
just minted, and the rows cannot cycle. The type does not say so.
`pair_original` follows the rows to their end through
`names::chase::to_root`, which panics when the walk outlasts its bound.

The Boolean already keeps the same rows as a list of mints:
`lineage::NullEdgeCopies` (`Minted<VertexKey>`), whose `push` refuses
a self row or a `new` that an earlier row names. A list built that way
cannot cycle, and `Minted::root` is total. If `vertex_pairs` were such
a list, `pair_original` would be `vertex_pairs.root(v)`, and the bound
and the panic would go.

## What the change costs

- `vertex_pairs` changes type, and that is a public field of
  `SplitNaming`. Its readers are `finish.rs`
  (`SplitResult::lineage`),
  `editor-core`'s `emit_topo`, and `topo/tests/join_whole_orbit_rows.rs`.
- `LineageRows::NullEdgeCopies` carries a Boolean `Operand`.
  `EdgeDivisions(Option<Operand>)` is the precedent for a split's list
  with no operand.
- The rows have to be pushed in mint order. A copy can be minted at an
  earlier copy, so its row has to come after the row that minted that
  copy. Whether the reduction's record order is already mint order has
  to be checked before the change is made.
