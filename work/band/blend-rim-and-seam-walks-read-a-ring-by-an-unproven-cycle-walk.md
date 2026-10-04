---
id: blend-rim-and-seam-walks-read-a-ring-by-an-unproven-cycle-walk
kind: issue
title: the blend surgery's rim split and wall seam take a ring's members from a next walk no plan proves claims the ring, with no tier gate in release
status: open
opened: 2026-10-04
priority: P3
cost: E
---


## What

Found by the receipt of TOPO's row-walk unit (PR 4016), which routed
the pcurve rows walk and the Euler site-row walks through
`Body::loop_cycle_of` (`crates/topo/src/body.rs`). That walk refuses a
member that does not claim the loop. The blend surgery
(`crates/sweep/src/blend/surgery.rs`) takes a ring's members from its
own `loop_walk`, a wrapper over `Body::loop_cycle` with no
`parent_loop` proof. Two writes follow from it:

- `rim_phase` runs `split_fragment` → `Body::split_edge` on the
  meridian at each walked ring vertex.
- `wall_seam` chooses the host and mate seam edges the annulus carve
  cuts. It cross-checks them against the vertex's edge orbit, not
  against `parent_loop`.

Both are reached by `fillet_edges` and the chamfer door through
`blend_surgery`. Neither door gates its operand at entry, and the
closing tier-2 check is a `debug_assert`, so a release build adopts
whatever the diverted walk wrote. Not measured.

## The shape to give

`loop_walk` over `Body::loop_cycle_of(first, loop)` (crate-internal
today; the blend needs a public door onto it, or `loop_walk` checks
each member's `parent_loop` itself), refusing the blend's corrupt-input
variant where it answers `None`.
