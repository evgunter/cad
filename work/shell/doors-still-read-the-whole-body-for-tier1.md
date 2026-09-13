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

**The three whole-arena iterations, which the SHELL-10 PR's first
draft did not name.** Each door's decide phase visits every entity of
the body and filters with the scope's maps, so the ANSWER is scoped and
the ITERATION is not: the plane/chart sweep, the corner walk and the
edge walk (`crates/topo/src/offset_together.rs:229`, `:241`, `:262`;
`crates/topo/src/offset_axial.rs:409`, `:482`, `:507`), plus
`axial_frame`'s own `body.vertices()` sweep at `offset_axial.rs:816`.
The clone each door writes to is a fourth. Together with the tier-1
reads below they make a scoped call **linear in the whole body, not in
its scope**, and both reviewers measured it: the same one-solid move
set costs about 0.20, 0.30, 0.51 and 1.00 ms on bodies of one, two,
four and eight solids. Nothing pins that — no guard, no register — so
a regression in it is invisible.

**The larger tier-1 read: the asserting setters — TOPO's, not this
item's.** `Body::set_face_surface` and `Body::set_edge_curve` each run
a whole-body `validate(&self)` as a postcondition
(`crates/topo/src/attach.rs:92-97` and `:331-336`), and a door performs
one per moved face and one per re-described edge: **18** whole-body
tier-1 walks for a scoped planar call on a unit box (6 + 12), 16 for
the axial door, 90 for `shell_open` on the hollow-hollow-open body and
101 on box-beside-vessel opened. It is also a **panic, not a typed
refusal**, and this workspace's release profile sets
`debug-assertions = true`, so it is compiled there too. Both SHELL-10
reviewers ruled it `attach.rs`'s finding rather than the doors' — the
convention is the postcondition's, and every mutation door in the crate
pays it — so it carries on as TOPO's own item,
`attach-postconditions-validate-the-whole-body-and-panic`, filed by the
orchestrator at merge. It is named here only because it is what a
reader of THIS item will otherwise measure and misattribute, and
because it is why SHELL-10's structural-corruption row stops at the
scope walk: on a body whose out-of-scope solid is malformed the door
panics inside the first setter, before either the mint or the closure
check is reached (`crates/topo/src/shell10_r2_probes.rs`,
`r2_the_door_panics_in_the_first_setter_on_an_out_of_scope_malformed_solid`,
and the doc of `offset_together.rs`'s
`an_out_of_scope_solids_corruption_does_not_refuse_the_scope_walk`).

What would close this item: a tier-1 entry that takes a shell subset
and is the same passes restricted rather than a second implementation
— which means first separating the arena-global passes from the local
ones, a `validate.rs` change with its own evidence — and decide-phase
walks driven by the scope's own entities rather than by the arenas.
Neither is SHELL-10's. The setters' postcondition is not either, and
not free: it is the guard that made every mutation door's tier-1 claim
mechanical (`review_m1_pr5_internal`'s door table).
