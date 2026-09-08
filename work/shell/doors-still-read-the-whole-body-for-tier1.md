---
id: doors-still-read-the-whole-body-for-tier1
kind: issue
title: the simultaneous doors' tier-1 reads are still whole-body: the closing closure check and the asserting setters
status: open
opened: 2026-09-08
---


SHELL-10 narrowed two of the three whole-body walks around a scoped
simultaneous offset (`shell-doors-still-walk-the-whole-body`): the
scope's construction and the closing pcurve pass. The third — the
closing `crate::validate::validate_closed(&work)` in
`crates/topo/src/offset_together.rs` and `crates/topo/src/offset_axial.rs`
— stays whole-body, and the spec's own STOP (`docs/SHELL-10-SPEC.md`
§3) is what it fired on: **tier 1 cannot be restricted to a shell
subset without a second implementation, and no per-shell machinery
exists to reach instead.**

What was measured. `validate::tier1` (`crates/topo/src/validate.rs`,
`fn tier1`) is thirteen passes over the raw arenas
(`body.solids.iter()`, `.shells`, `.faces`, `.loops`, `.half_edges`,
`.edges`, `.vertices`, plus the three geometry arenas), sharing
cross-arena counters. Five of them are **global by construction**, not
merely global by spelling — restricting them to a shell subset does not
narrow the check, it changes it:

- pass 1, reference resolution, and pass 7, ownership and
  back-pointers: both count owners across the whole arena, so a face
  owned by an out-of-subset shell reads as unowned;
- pass 3, the edge ↔ half-edge bijection: an edge's two halves can be
  reached only from the arena;
- pass 8, orphan geometry: refcounts over the point/curve/surface
  arenas, so every surface an out-of-subset face wears reads as an
  orphan;
- pass 10, edge-adjacency shell coherence: its subject is the relation
  between shells.

`validate.rs` exposes no per-shell entry (`pub fn`s: `validate`,
`validate_closed`, the tier-3 family, `validate_pseudomanifold`) and no
private helper takes a `ShellKey`. The spec forbids a second validator,
so the check stayed whole-body and the doc sentences say so.

There is a second, larger whole-body tier-1 read the original item did
not name, found while pinning SHELL-10's acceptance row 2: **the
asserting setters.** `Body::set_face_surface` and `Body::set_edge_curve`
each run `validate(&self)` as a postcondition
(`crates/topo/src/attach.rs:93`, "set_face_surface postcondition: result
is not tier-1 valid (kernel bug)"), and a door performs one per moved
face and one per re-described edge — so a scoped call on an N-solid body
pays O(faces + edges) whole-body tier-1 walks, not one. It is also a
**panic, not a typed refusal**, and this workspace's release profile
sets `debug-assertions = true`, so it is compiled in release too. That
is why SHELL-10's structural-corruption row stops at the scope walk: on
a body whose out-of-scope solid is malformed the door panics inside the
first setter, before either the mint or the closure check is reached
(`crates/topo/src/offset_together.rs`, `mod scope_walks`,
`an_out_of_scope_solids_corruption_does_not_refuse_the_scope_walk`,
whose doc records it).

What would close this: a tier-1 entry that takes a shell subset and is
the same passes restricted rather than a second implementation — which
means first separating the arena-global passes from the local ones, a
`validate.rs` change with its own evidence — and a decision about the
setters' postcondition (a scoped postcondition, or one paid once per
door rather than once per write). Neither is SHELL-10's, and neither is
free: the setters' assert is the guard that made every mutation door's
tier-1 claim mechanical (`review_m1_pr5_internal`'s door table).
