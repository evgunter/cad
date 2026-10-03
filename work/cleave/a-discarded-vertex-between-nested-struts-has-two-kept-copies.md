---
id: a-discarded-vertex-between-nested-struts-has-two-kept-copies
kind: issue
title: A discarded vertex between two nested struts has two kept copies on one twin face: kept_end refuses JoinDesync
status: open
opened: 2026-10-03
priority: P1
cost: M
---

## What

Three dangling null edges nested at the cube's corner: the
block's (Out), the inner lens's (In), and a notch's (Out) inside it.
The witness is `two_dangling_null_edges_with_one_segment_ending_in_the_cubes_face`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`), with `y` the
lens-in-a-lens less a notch, with or without a lens in the notch. Both
∩ build and pass 3′. `y ∪ cube`, `cube ∪ y` and `cube ∖ y` refuse
`JoinDesync`: "a section vertex's null-edge copies have not exactly
one kept end". Pinned there as it stands.

`y` is a boolean output that the next boolean refuses, so this may
be P0 on FUSE's ruling of 2026-10-02.

## Cause (measured)

The refusal comes from `finish.rs` `discarded`, in its `kept_end`
closure (`crates/topo/src/boolean/finish.rs`). In the cube, the
nesting leaves the chain `v —block— t —lens— w —notch— tip`. `w` is
the lens's In region, discarded by ∪ and by `cube ∖ y`. Its copies
(`NullCopies::of`, transitive) take in both its neighbours: `t`, the
region outside both pieces, and the notch's tip. Both are kept, and
both lie on the twin of the section face `across`, so the twin filter
leaves two.

An instrumented run of `cube ∪ y`, notched:
- `v = 28v1`, `across = 13v1`.
- `copies = [27v1, 29v1, 1v1]`, `kept = {27v1, 29v1}`, both on the
  twin `8v9`.

By `finish`, the null edges are gone (neither half of `w`'s two null
edges resolves to a face). So which neighbour borders the stretch
along `across` is no longer readable off them.

## Owed

Pick the kept copy across the null edge that bounded `across`'s
polygon, read before the null edges go. One option is to record, per
section face, the null edges its polygon ran through. Then flip the
pin's ∪ and `cube ∖ y` to building.
