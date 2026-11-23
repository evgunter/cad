---
id: a-patch-the-join-splits-is-never-a-memo-hit-for-the-pick-index
kind: issue
title: A patch the join splits names no memo entry, so the pick index rebuilds its table every picture
status: open
priority: P3
opened: 2026-11-23
---


`mesh::tessellate_with` reports, per face, the memo entry its placed corners are (`mesh::PatchKeys::stored`), and `editor_core`'s `PickMemo` serves a patch's whole pick table under that id without comparing anything (`resolve/pick.rs`, `PickMemo::table`). A patch the join splits (`mesh::join`) is placed with points its entry does not hold, so it names no entry (`PatchKeys::unstore`, called in `tessellate::tessellate_impl`'s fold), and its pick table is built every picture even when its face was a patch-memo hit and its splits are the same as last picture's.

Measured: on the rod sliver (`editor_core`'s `a_patch_split_where_the_patches_join_is_built_every_picture`), the faces the join splits miss the table memo every picture. Over the `mesh` suite at the default ε 73 meshes have a split patch; nothing measured says whether an editor body meets one often enough for the rebuild to cost.

**What closing it takes:** an identity that names an entry *and* the sides split in it, so two placements with the same splits share a table and two with different splits cannot: either `StoredPatchId` carrying the split set (it is a minted `u64`, so a hash of the set beside it can collide, which a cache that compares nothing cannot afford), or a second table in the pick memo keyed by (entry, split sides). Only worth it if a measured editor session shows the rebuild.
