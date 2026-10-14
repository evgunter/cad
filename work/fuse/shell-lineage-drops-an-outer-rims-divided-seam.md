---
id: shell-lineage-drops-an-outer-rims-divided-seam
kind: issue
title: Shelled::lineage states no fate for an outer rim's divided seam: its seam_pieces self row hides the seam from lineage::roots
status: open
opened: 2026-10-13
priority: P2
cost: E
refs: [lineage-lists-are-five-newtypes-over-one-core]
---

Found by `fuse/one-lineage-list` (the `lineage-lists-are-five-newtypes-over-one-core`
unit) while measuring whether `RimNaming::seam_pieces` could go on a
checked mint list. **Suspected, not yet pinned by a test.**

`seam_rim` (`crates/topo/src/shell.rs`, the `SeamedRim` it returns)
writes `seam_pieces: vec![(seam, source_seam), (top, source_seam)]`.
With `side` `Outer` the first piece keeps the divided edge's key, so
`seam == source_seam` and the first row maps the seam to itself. A
probe over the sweep and editor-core shell tests (`verbs_shell`,
`shell_node`, `emit_shell`, `edge_piece_locality`: 90 tests) saw 7 rims
written, 4 of them with that self row.

`Shelled::lineage` (`crates/topo/src/shell.rs`, the loop over
`roots(&rim.seam_pieces)`) states the divided seam's fate from
`crate::lineage::roots`, which returns each divided-from cell that no
row lists as new. With rows `[(S, S), (T, S)]`, `S` is listed as new
(the self row), so `roots` returns nothing and the loop never states
`S`'s fate: the outer rim's divided seam is left to whatever `lineage`
says of a cell it never touched, not `opened([S, T], corners)`. The void
rim's rows (`(twin, …)`, no self row) are not affected.

What is owed: a row that builds an outer seamed rim and asserts the
divided seam's fate (red today if the suspicion holds), then the
reader fixed, for instance by reading `seam_pieces` as the map it is
(group the pieces by their second column) rather than through
`roots`/`Descent`.
