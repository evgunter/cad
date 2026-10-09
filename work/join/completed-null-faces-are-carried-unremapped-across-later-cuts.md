---
id: completed-null-faces-are-carried-unremapped-across-later-cuts
kind: issue
title: bool_connect carries each completed null face across later cut_pair kills unremapped (unverified)
status: dispatched
opened: 2026-10-05
priority: P3
cost: E
branch: join/three-small-join-rows
---

## What (unverified)

`boolean/join.rs` `bool_connect` collects each completed pair as an
`UnresolvedPair { a_face, a_outer, a_ring, .. }` and carries it across
every later `cut_pair` (`chord_join.rs` `cut_core`), which may `kef` a
face out of `slivers`. A completed null face is minted by a chord
`mef`, so it sits in `slivers`, and nothing removes it there: a later
interior null edge with that face on one side could pick it as the
`kef` victim. The keys are read at quiescence
(`resolve_roles_geometric`, `set_null_face_pair`) with no remap; a miss
answers typed (`JoinDesync("completed section loop no longer
resolves")`, or the `get_face` miss), never silently.

Found by the carried-keys sweep of the REST lane's segment-end remap
(`work/topo/segment-ends-are-not-remapped-across-the-rest-lanes-strut-undo.md`),
by reading only: whether a completed null face can neighbour a later
null edge is not established.

## Direction

Establish whether a completed face can be a later `kef`'s victim (a
row, or the invariant stated at `cut_core`); if it can, carry the
completed rows through the kills, as `fused_through` does for the zip.
