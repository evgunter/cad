---
id: completed-null-faces-are-carried-unremapped-across-later-cuts
kind: issue
title: bool_connect carries each completed null face across later cut_pair kills unremapped (unverified)
status: closed
opened: 2026-10-05
priority: P3
cost: E
closed: 2026-10-09
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

## Built — the invariant, stated and asserted

A completed null face is never a later kef's side. `chord_join.rs`
`ChordJoiner::cut_core` states it at the kef with its reason: the edge's
sides are the slivers its own polygon's chords walled off at its two
ends, and a completed face is bounded by another polygon's two copies,
which kemr left once that polygon's last null edge was cut. The joiner
records each face it completes (`completed`), and a `debug_assert!` at
the kef refuses one on either side. So any debug run that reaches the
case goes red there, before the key is read at quiescence.

## Measured

Before the assertion, a probe at the same point logged any kef with a
completed face on a side, and any kef with one face on both sides. It
logged nothing in:

- the sweep suite at ε 1e-9, 1e-6 and 1e-12 (2 460 tests each);
- `pierce_runs_battery` (4 536 lines), `pinch_runs_battery` (3 024) and
  seven `rc_wide` shards (0, 13, 27, 41, 55, 69 and 83 of 84, 480 lines
  each);
- the 323-pose search in the sibling row
  `the-forced-order-lanes-could-accept-a-ring-held-run`.

The split sweep shares the joiner and resolves its completed faces at
once, so only the boolean carries a completed key to quiescence. No
`fused_through`-style remap is built.
