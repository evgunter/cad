---
id: a-pair-merged-face-group-reads-no-cutters-though-its-seams-cite-it
kind: issue
title: A pair boolean's merged face group reads as not seam-bounded, though its seams now cite its Merged parent
status: open
opened: 2026-10-18
---


## Finding

`crates/editor-core/src/resolve/mod.rs` `SeamParent::of` reads a pair
boolean's group only when its head is `[From { read, of }]`, and
`group_cutters`' doc lists a pair's merged face among the bases that are
`GroupCutters::NotSeamBounded`. That was the only reading available while
the pair's seam pass spelled a seam beside a merged face by one of its
constituents. Since `wire/seam-cites-parent`, a pair seam beside a merged
face cites the face's `Merged` parent as that side
(`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_edges`'s
`merged_side`; N3 in `crates/editor-core/src/names/README.md`), so the
seams that divide a merged parent held as several faces are on record by
the parent's name, as they are for an operand face's `From` group.

Shape of a repair: a `[RoleSeg::Merged(_)]` head in a pair reads as
`SeamParent::Pair { parent: base, .. }`, its side being the side opposite
the seam's operand face (`a_side` from which seam side the parent holds,
not from `from`). Not built: the reading is doctail's ground, and nothing
refuses today; a merged group's divider report says "not seam-bounded"
where it could say which cutters came and went.

Signed (WIRE lane `wire-seamparent`).
