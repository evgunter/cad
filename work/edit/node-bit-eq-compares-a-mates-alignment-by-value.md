---
id: node-bit-eq-compares-a-mates-alignment-by-value
kind: issue
title: Node::bit_eq compares a mate's alignment datum by value, so a -0.0 and a 0.0 frame are one mate to every D7 comparator
status: open
opened: 2026-09-29
priority: P3
cost: E
---


Found by the placement unit's sweep (`edit/placement-type`), which
fixed two instances of the class in `Node::bit_eq`
(`crates/editor-core/src/node.rs`): a transform's literal steps and an
explicit placement rule's listed frames were compared through the
derived `PartialEq`, which reads `0.0 == -0.0`, and now compare by
bits.

The third instance is `Node::Mate`'s `alignment`
(`crate::mate::Alignment`, `crates/editor-core/src/mate.rs`): two
`MateFrame`s of bare `[f64; 3]`s, the primitive's authored lengths
(`MatePrimitive::authored_lengths`) and the optional clocking rider.
`Node::bit_eq` reaches them only through `self != other`, so two mates
differing only in a signed zero are one node to the D7 comparator,
while the content key (`feed_alignment` in `eval/mod.rs`) feeds the
same fields by bits and calls them two. The two disagree about one
node.

The fix is a bit comparator for `Alignment` (its home is `mate.rs`,
MSOLVE's ground; the caller is `Node::bit_eq`, EDIT's), read by the
`match` the placement unit added to `Node::bit_eq`. Filed rather than
fixed there because the comparator belongs beside the type it reads.
