---
id: product-refuses-split-halves-as-roots-when-a-tie-narrows-to-unique
kind: unit
title: product falsely refuses Naming when the two halves of one split are taken as two Part roots over an N2 tie the plane separates
status: review
opened: 2026-09-24
priority: P0
cost: H
refs: [product-refuses-naming-when-one-instance-is-placed-under-two-roots]
branch: gather/split-halves-tie-merge
pr: 3256
---

Found by PR 3142's review (MAJOR 1), filed by the GATHER two-roots lane.
It predates PR 3142, which neither causes nor fixes it.

**The false refusal.** A document whose two roots are
`Part(Above)` and `Part(Below)` of ONE split places no body twice, and
the gather still refuses it `ProductError::Naming` whenever the split's
plane separates the candidates of an N2 tie. The same split, taken as
the only root, gathers, and the product holds one `Entry::Tied` row.

**Repros**, both pinned as measurements in
`crates/editor-core/tests/gather_placed_under_two_roots.rs`, to be
flipped when this is fixed:

- `split_halves_as_roots_over_a_one_one_tie_falsely_refuse_at_the_carry`
  (~580): a 4×4×4 block less a U-cutter with prongs at y ∈ [1, 1.5] and
  [2.5, 3], split at y = 2. It refuses from the per-root carry,
  `Naming { node: <Below's Part>, name.node: <subtract> }`.
- `split_halves_as_roots_over_a_one_two_tie_falsely_refuse_at_the_flush`
  (~601): the same block less an E-cutter with prongs at y ∈ [0.5, 1],
  [1.75, 2.25] and [3, 3.5], split at y = 1.5. It refuses from the tie
  flush, `Naming { node: <subtract>, name.node: <subtract> }`, naming
  the minter and neither root.

**Mechanism.** A `Part`'s table is its input's projected onto one
output body (`crates/editor-core/src/names/table.rs`, `NameTable::project`
~461). That projection narrows a tie through `narrow_into`
(`names/defer.rs` ~156), so a half holding ONE candidate writes the tied
name as `Unique`. The gather's carry (`names/defer.rs`,
`CarriedRows::carry` ~254) decides whether to defer by the SOURCE
entry's tie bit. A `Unique` row therefore goes straight to
`insert_ref`, and a `Tied` row is deferred:

- one candidate in each half: both rows are strict, and the second
  collides in `carry_names` (`product.rs` ~893);
- one in one half and several in the other: the strict row meets the
  deferred tie at `tie_rows.finish` (`product.rs` ~918;
  `CarriedRows::finish`, `names/defer.rs` ~313).

A split ROOT does not hit this, because the split's own table keeps the
tie across both output bodies and every row arrives deferred.

**What a fix has to decide** (not decided here): whether the gather
should recover the tie across sources whose tables narrowed it (it
would need to know that the `Unique` rows under one name descend from
one tie upstream), or whether `Part`'s projection should keep a
cross-body tie marked as a tie. Either way, the fix is right when both
rows above gather into one `Entry::Tied` row, with the same shape the
split root gets.

## The fix

The gather recovers the tie. `NameTable` marks a `Unique` row that is
one piece of a tie separated across output bodies upstream:
`NameTable::project` marks a tie it narrows to one candidate and keeps
a mark its input already carried, a split's intact pass-through keeps
the mark (`names::defer::pass_through`), and a `Transform` shares its
input's table whole. `CarriedRows::carry` defers a marked row as it
defers a tied one, so the pieces merge at the flush into the one
`Entry::Tied` the split's own table holds. A lone `Part` publishes the
same `Unique` row as before, and a lone `Part` root gathers it `Unique`.

The two measurements are now passing rows asserting the gather, and
each compares the two-halves product table with the split-root
product table, row for row (`gather_placed_under_two_roots`,
`split_halves_as_roots_over_a_one_{one,two}_tie_gather_one_tie`).
`a_separated_piece_merges_through_a_transform_and_a_second_split`
covers the other verbatim edges.
