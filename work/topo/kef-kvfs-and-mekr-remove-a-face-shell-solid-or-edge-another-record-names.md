---
id: kef-kvfs-and-mekr-remove-a-face-shell-solid-or-edge-another-record-names
kind: issue
title: kef, kvfs and mekr remove a face, shell, solid, edge or ring loop on the strength of its boundary or anchor, not proving no other record names it: DanglingTopology through Ok
status: review
pr: 3570
branch: topo/kill-proves-removals
opened: 2026-09-30
refs: [mekr-and-kvfs-remove-an-empty-loop-or-lone-vertex-another-record-names, kvfs-kills-a-lone-vertex-and-loop-a-torn-half-edge-still-names]
priority: P3
cost: M
---

## What

Found by the review of the walk-proofs unit (PR 3511). That unit
proves the half-edges a plan walks, moves and removes, and the loops
and vertices it kills. Its second pass covered only
`loops.remove` and `vertices.remove`. The removals of faces, shells,
solids and edges were never swept. Each reads the record's
emptiness from its boundary or anchor (`face.rings`, `shell.faces`,
`solid`'s shell list, `edge.he_plus` / `edge.he_minus`), and none proves
that no other record still names it:

- `Body::kef` (`crates/topo/src/euler_kill.rs`, the kill phase,
  `self.edges.remove(edge)` and `self.faces.remove(f1)`, ~`:1479`,
  `:1483`): a half-edge outside the edge's two halves whose `edge` is
  torn to the killed edge, or a loop outside `f1`'s boundary whose
  `face` is torn to `f1`, is left naming the dead record.
- `Body::kvfs` (`crates/topo/src/euler_kill.rs`, the kill phase,
  `self.faces.remove(face)`, `self.shells.remove(shell)`,
  `self.solids.remove(solid)`, ~`:623`-`:632`): a loop of another shell
  torn to name the lone face, a face torn to name the lone shell, or a
  shell torn to name the lone solid is left naming it.
- `Body::mekr` at `MekrSite::Cycles` (`crates/topo/src/euler_ring.rs`,
  `mekr_finish`, `face_data.rings.retain` then
  `self.loops.remove(ring_loop)`, ~`:1723`): only the ring's own face
  drops it from its `rings`. Another face whose `rings` is torn to list
  the ring is left naming a dead loop.

Unmeasured siblings of the same shape: the edge removals in `Body::kev`
(`euler_kill.rs`, ~`:1189`) and `Body::kemr` (`euler_ring.rs`, ~`:570`)
also trust `edge.he_*`.

## Measured

The review's probe `walkr_c8_other_removals`, re-run at the fix
pass's head (dev, each call inside a surgery scope, validator
`DanglingTopology` counts before and after). Each call returns `Ok`,
with 0 dangling before and 1 after:

| call | tear | left dangling |
| --- | --- | --- |
| `kef` (declined cube) | a third loop's `face` torn to the dying face | `Loop → Face` |
| `kef` (declined cube) | a third half-edge's `edge` torn to the killed edge | `HalfEdge → Edge` |
| `kvfs` (segment solid beside a lone solid) | the segment's face's `shell` torn to the lone shell | `Face → Shell` |
| `kvfs` (same) | the segment's shell's `solid` torn to the lone solid | `Shell → Solid` |
| `kvfs` (same) | the segment's loop's `face` torn to the lone face | `Loop → Face` |
| `mekr_chord(Cycles)` (holed box) | a box face's `rings` torn to list the top face's ring | `Face → Loop` |

## The shape to give

Each removal proves, before it mutates, that no record outside the
removed set names what it removes. That is an arena scan per record
kind, bounded as `Body::require_run_of`'s `Whole` proof is. The kill
refuses a typed corruption variant naming the record, and one row is
pinned per case above. The owner decides whether to add a new variant
per record kind or reuse the loop and orbit variants. If they add one,
it ends in the defect ending (see
`euler-op-corruption-refusals-end-in-a-tag`).
