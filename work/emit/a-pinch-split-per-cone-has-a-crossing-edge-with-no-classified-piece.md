---
id: a-pinch-split-per-cone-has-a-crossing-edge-with-no-classified-piece
kind: issue
title: Naming refuses Emission 'a crossing's edge has no piece the boolean classified at the crossing' on a union whose pinch split_cones splits per cone (plate and two leaning wedges, wedges first)
status: open
opened: 2026-10-07
priority: P0
cost: M
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

## Done when

The three rows above pass on main with the naming of #4203 in place.
