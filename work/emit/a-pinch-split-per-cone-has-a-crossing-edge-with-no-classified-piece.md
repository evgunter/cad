---
id: a-pinch-split-per-cone-has-a-crossing-edge-with-no-classified-piece
kind: issue
title: Naming refuses Emission 'a crossing's edge has no piece the boolean classified at the crossing' on a union whose pinch split_cones splits per cone (plate and two leaning wedges, wedges first)
status: review
opened: 2026-10-07
priority: P0
cost: M
branch: emit/pinch-crossing-classified
pr: 4269
---


## What

Found by TANG's PR 4129 on main b475cf87. EMIT's #4203 ("every
crossing carries its sense", 704378c3) made `names/emit_topo.rs`'s
`sense_of` refuse a crossing whose edge has no classified piece at the
crossing. The witness's vertex is a pinch, which JOIN's
`zip::split_cones` (#4139) splits per cone before the zips. Whether
the two interact is not established; the title names the pose, not the
cause.

The witness is the union of the plate `[0, 3] × [0, 2] × [0, 1]` and
two leaning wedges whose footprints meet at (1.5, 1, 1)
(`topo::test_support::meeting::two_wedges`, through
`crates/editor-core/tests/union_pinch_member_order.rs`'s
`tilted_holes`). It is built in the member order wedges first, plate
last ([2, 1, 0]). editor-core refuses `Naming(Emission { "a crossing's
edge has no piece the boolean classified at the crossing" })`.

- topo builds every order of the same union sound (tiers 3 and 3′,
  closed-form volume, `corners_disjoint`;
  `crates/topo/tests/holes_meeting_at_a_vertex.rs`).
- Before #4203 (main 61edf137), editor-core named every order. PR 4129's
  rows `union_pinch_member_order::{wedges_meeting_at_a_vertex_build_one_body_in_every_member_order,
  holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_member_order,
  the_junction_where_the_wedges_meet_has_one_name_in_every_member_order}`
  passed there.
- With #4203 they refuse. The first refusals: two wedges [2, 1, 0];
  three wedges on one side [3, 2, 1, 0]; the junction row's [3, 2, 1, 0].
- Reproduced on bare main b475cf87 with only PR 4129's shared fixtures
  and that test file added, so the naming arm this PR adds is not
  involved at k = 2.

## Flip-back rows

These must build when this is fixed. Until then each row asserts the
refusal, by its exact text
(`union_pinch_member_order.rs`, `UNCLASSIFIED_CROSSING`), in the
member orders whose first fold step unions two angularly adjacent
wedges, and in no other order:

| row | fixture | adjacent pairs | refusing orders |
|---|---|---|---|
| `wedges_meeting_at_a_vertex_build_one_body_in_every_member_order` | two wedges | 1–2 | 2 of 6 |
| same | three wedges | every pair | 12 of 24 |
| same | four wedges | 1–2, 2–3, 3–4, 4–1 | 48 of 120 |
| `holes_with_a_reflex_sector_at_their_vertex_build_one_body_in_every_member_order` | three wedges on one side | every pair | 12 of 24 |
| same | an L and two wedges | 2–3 (the two wedges) | 4 of 24 |
| `the_junction_where_the_wedges_meet_has_one_name_in_every_member_order` | three wedges, plate last | every pair | 6 of 6 |

Member 0 is the plate. The regression came in with 704378c3 ("emit:
every crossing carries its sense; same-sense crossings rank alone"),
merged to main in #4203. On main 61edf137, before it, every one of
these orders built and named the junction.

## Done when

The flip-back rows build in every member order, with the naming of #4203 in place, and `every_order_but` and the junction row's refusal arm go.

## Cause

`names/emit_topo.rs`'s `fused_partners` read `vertex_merges` one hop
deep: each kept key got the keys fused straight into it, never those
fused into a key that a later row fused away in turn. On the first
fold step of two adjacent wedges (two wedges, [2, 1, 0]: A wedge 2,
B wedge 1), the reduction's null edges leave A's pinch vertex two
copies, and the zips fuse B's copy into the first (`7v3 → 20v1`) and
then the first into the second (`20v1 → 21v1`). Result vertex `21v1`
read only `20v1`, so `operand_vertex_keys` held A keys alone, and B's
leg edge, which the boolean classified at B's copies of that point,
had no class row among them. `sense_of` refused. Before #4203 nothing
read the senses there, so the short read was silent.

`fused_partners` now gathers every key fused into a vertex through any
number of rows (in the order they died), and `operand_vertex_keys`
closes over B-side welds and null copies together, both as two keys
of one point, so a chain of welds or a weld reached through a null copy
is read as well. topo records every class it owes here; the fix is in
the naming. The sense rule (N2 *Vertices*) is unchanged.
