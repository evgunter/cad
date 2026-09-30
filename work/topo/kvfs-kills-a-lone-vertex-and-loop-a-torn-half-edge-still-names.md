---
id: kvfs-kills-a-lone-vertex-and-loop-a-torn-half-edge-still-names
kind: issue
title: kvfs removes its Empty loop and lone vertex without proving no half-edge names them: a torn parent_loop or start leaves DanglingTopology through Ok
status: open
opened: 2026-09-30
priority: P3
cost: E
refs: [kill-ops-loop-anchor-on-an-unproven-next-step]
---


## What

Found by the fix pass of `kill-ops-loop-anchor-on-an-unproven-next-step`
(PR 3495), whose receipt walked every production `loops.remove`.

`Body::kvfs` (`crates/topo/src/euler_kill.rs`) removes a single-face
shell's outer loop, which it requires to be `Empty`, and that loop's
lone vertex. Its plan reads the loop's boundary and the vertex's key,
and nothing about the half-edges: it never proves that no half-edge
claims the loop (`parent_loop`) or starts at the vertex. `Empty`
says the loop has no member and the vertex no incidence, but a torn
`parent_loop` or start makes a half-edge of another shell name them
anyway, and the kill leaves that half-edge naming a dead loop or a
dead vertex.

This is the removal-side sibling of
`kef-and-mekr-kill-a-loop-whose-members-they-read-by-the-walk`, with no
walk: `kvfs` takes the loop's emptiness from its boundary rather than
from a cycle walk.

## Measured

At PR 3495's fix-pass head (dev, inside a surgery scope so the tier-1
postcondition does not answer first): an `mvfs` lone shell beside a
segment (`mvfs` plus `mev_line` at `MevSite::Lone`), then one tear and
`kvfs` on the lone shell's solid.

- The segment's plus half torn to claim the lone loop: `Ok`, and
  `validate` reports `DanglingTopology` (beside the planted
  `ParentLoopMismatch`).
- The segment's plus half torn to start at the lone vertex: `Ok`, with
  `DanglingTopology` (beside the planted `EmanatingStartMismatch`).

## The shape to give

`kvfs`'s plan proves that no half-edge claims the loop or starts at
the vertex it removes, bounded by the arena as the kill anchors'
`Lone` proof is (`Body::require_kill_anchors`), and refuses typed
otherwise. Pin one row per tear.
