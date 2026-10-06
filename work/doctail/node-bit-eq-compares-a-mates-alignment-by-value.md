---
id: node-bit-eq-compares-a-mates-alignment-by-value
kind: issue
title: Node::bit_eq compares a mate's alignment datum by value, so a -0.0 and a 0.0 frame are one mate to every D7 comparator
status: closed
opened: 2026-09-29
priority: P3
cost: E
closed: 2026-10-03
pr: 3961
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

## Closed

Fixed by PLACE's mate-frame-offset unit (PR 3961). `Alignment::bit_eq`
(`crates/editor-core/src/mate.rs`) compares both frames by
`MateFrame::bit_eq` (base, then `Placement::bit_eq`), the primitive's
lengths and the rider by bits. `Node::bit_eq` reads it for a mate, and
reads `Placement::bit_eq` for a gauge's placement and an instance's
offset beside the transform's it already read. Python's
`Alignment.__eq__` and `MateFrame.__eq__` ask the same comparators.
Pinned by `place_mate_frame_offset::a_mates_alignment_compares_by_bits`.
