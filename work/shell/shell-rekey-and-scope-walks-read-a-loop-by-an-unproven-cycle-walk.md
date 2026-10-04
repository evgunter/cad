---
id: shell-rekey-and-scope-walks-read-a-loop-by-an-unproven-cycle-walk
kind: issue
title: shell's loop re-key, offset_together's scope and replace_face's boundary take a loop's edges from a next walk no plan proves claims the loop
status: open
opened: 2026-10-04
priority: P4
cost: E
---


## What

Found by the receipt of TOPO's row-walk unit (PR 4016), which routed
the pcurve rows walk and the Euler site-row walks through
`Body::loop_cycle_of` (`crates/topo/src/body.rs`). That walk refuses a
member that does not claim the loop, so a torn `next` cannot hand
another loop's half-edges to a reader. Three of this program's
readers take a loop's edges from `Body::loop_cycle` with no
`parent_loop` proof. Each decides a write on a staged clone of an
operand that no entry gate validates:

- `shell::loop_rekeyed` (`crates/topo/src/shell.rs`): the walked
  edges' specs go to `Body::set_edge_curve` through
  `rename_loop_surface` and to `Body::set_face_surfaces_describing`.
- `offset_together::Scope::walk`
  (`crates/topo/src/offset_together.rs`): the walked starts and edges
  decide which vertices `offset_planes_together` moves and which
  edges it re-describes (reached from `shell` too).
- `replace_face::boundary_edges_into`
  (`crates/topo/src/replace_face.rs`): `group_boundary` decides which
  edges get re-derived curves.

A diverted walk re-describes or moves another face's records. Each
door's closing tier check (`validate_geometric` in `shell`,
`validate_closed` in `offset_together` and `replace_face`) refuses
most such results after the fact, so the expected outcome is a
refusal that names the result rather than the torn input. Not
measured.

## The shape to give

`Body::loop_cycle_of(first, loop)` in place of `loop_cycle(first)` at
the three sites, refusing each door's corrupt-input variant where it
answers `None`.
