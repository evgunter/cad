---
id: member-space-look-through-stops-at-splits-containment-and-fragmented-merges
kind: issue
title: A member-space declaration resolves through merges only: a face consumed by a split, by containment, or inside a fragmented merged row is still order-shaped
status: open
opened: 2026-09-07
refs: [DOCM-8, 2073]
---

## What

DOCM-8 built the look-through for MERGES: a member-space name that is
no row at its step is rewritten to the accumulation's flat `Merged`
row that lists it (`crates/editor-core/src/eval/wire.rs`,
`look_through_merges`). Three other ways the fold consumes a member
face leave a declaration naming it order-shaped, measured by both
DOCM-8 reviews on head `6d433b6f`:

- **Split by a later member.** R1's fixture (`a` and `c` flush along
  x; `s` sits on `a`'s top cap, its footprint strictly inside it, so
  folding `s` in fragments the cap): of the six orders, `[a, c, s]`
  and `[c, a, s]` fuse; `[a, s, c]` and `[s, a, c]` refuse
  `DeclareResolve { Vanished { a.cap(End) } }` — the cap is neither a
  row nor in any merged row's set; `[c, s, a]` and `[s, c, a]` refuse
  the pre-existing `Emission("seam vertex parentage underdetermined
  from incident edges")` (`two-emitter-refusals-a-legal-declared-union-reaches`).
  The base refused six of six, so the look-through is a strict
  improvement; pinned as a measurement by
  `docm8_flat_merged::a_member_face_split_by_a_later_member_is_still_order_shaped`.
  R1's three-neighbour star (`c` between `w` and `e` along x, `t`
  stacked on its y-walls) is the same class: 8 of 24 orders `Vanished`
  on `c`'s end cap, 10 the seam-vertex `Emission`.
- **Consumed by containment.** R2's `r2_p7`: `a`'s x = 1 wall inside
  `big`, declared against `far`'s wall. Two orders `Vanished`, four
  `ContactContradicted` (the wall is a row at the step the pair is fed
  to and the kernel then contradicts the carrier claim).
- **A merged row later fragmented.** A member face inside
  `[Merged(set), Fragment(q)]` is in no searched set: that row is a
  fragment, not a merge, and `look_through_merges` reads bare
  `[Merged(set)]` rows only. Unreachable today: the emitter refuses
  the shape first (`crates/editor-core/src/names/emit.rs`,
  `unique_shared_edge` — R2 NOTE-4, `r2_p4`).

## Where it contradicts what is written

Nothing now. The three prose sites (`wire.rs` `route_declarations`,
`node.rs` `Node::Union`, `names/role.rs` `RoleSeg::FromMember`) and
N3 state the bound — merges only — and name this file.

## What a look-through into fragmented rows would need

A membership test cannot answer it: a fragment row's set names the
faces the MERGE listed, and which fragment a member face's material
ended in is a geometric question (the fragments' discriminators are
`SideOf`/`OrderAlong` verdicts against the cutting partners, N2), so
the rewrite would have to choose among the fragments by re-measuring
the member face against the partners — the re-measurement DOCM-8's
rule forbids at the routing step. The split case is the same
question one step earlier (which fragment of `a.cap` does the pair
mean?), and containment has no row to rewrite to at all. Any of the
three is a design ruling on what a member-space declaration means for
a face that is no longer one face.

