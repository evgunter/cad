---
id: door-arms-only-a-hand-split-operand-reached
kind: issue
title: Door arms only a hand-split operand reached have no at-rest input since tier 3's check 11
status: open
opened: 2026-10-08
priority: P4
cost: M
---


Tier 3's check 11 (`JoinableVertexAtRest`, Ev's PR 4251 ruling) makes a
body holding a vertex between two edges of one carrier construction
state, so no door takes it as an operand. Several door arms were
reached only by such operands — rims or seams split by hand
(`Body::split_edge`, `common::stations::cut_stations`) — and their rows
now assert the operand gate's refusal instead (PR D of step 3). No
at-rest input reaches these arms any more; each is either dead code to
retire or needs a witness no hand split supplies.

- **The blend's joined band** (PR 3701): two collinear links of one
  chain on the same two faces, carved as one band across the joint —
  `crates/sweep/src/blend/open/planar.rs:946` (`joined_blends`), its
  naming in `crates/editor-core/src/names/emit_blend.rs:146`, and the
  joint-foot clearance screen's `FaceClearanceUncertified` at a joint
  (`crates/sweep/src/blend/battery.rs:966`). A joint at rest is the
  join's, so every chain reaches the blend as one edge. Rows re-aimed:
  `review_3701_probes.rs`, `band_subdivided_side_walls.rs`
  (`a_split_rim_is_joined_before_it_blends`,
  `a_rim_band_is_cut_off_at_both_ends`),
  `blend_band_reach_chain_ends.rs`. **Dead code**: the blend doors
  (`fillet_edges`, `chamfer_edges`) take an `AtRestBody`, refused at
  `f64` by tier 3 and, at a dual, by the door's own operand gate
  (`AtRestBody::gate_unverdicted`, which reads check 11 since PR D:
  `unjoined_operand_at_a_dual.rs`), and the surgery plans its chains
  from that operand before anything is carved, so no construction-state
  caller reaches the arm at any scalar. Only the test-only meter
  (`band_reach_for_tests`) still reads a jointed chain.
- **The blend's half-band gate on a curved support carrying both arcs**
  (`crates/sweep/src/blend/surgery.rs:1333`): its row,
  `fillet_h5_r2_probes.rs`'s merged wall, holds the killed meridian's
  ends between two arcs of each rim, so it is construction state.
  Whether another at-rest operand reaches the gate is open; no
  construction-state caller does, through the same `AtRestBody` doors
  and their gates, at any scalar.
- **The shell's axial corners at a hand-split vertex**
  (`crates/topo/src/offset_axial.rs:1281`, "no profile constraint";
  `:1322`, "a line profile and a plane parallel to the axis"; and the
  line arm on a vertex whose only surface is a cylinder): the rows in
  `shell7_seam_corner.rs` say no door builds such a vertex, and the
  hand split that made one is construction state now. **Still
  reachable**: the public offset doors (`topo::offset_charts_together`,
  `topo::replace_face_offset`) take a `&mut Body`, so a construction-state
  caller still meets these arms, and `shell7_seam_corner.rs`'s rows
  read all three there (`a_corner_with_no_profile_constraint_refuses_typed_at_the_direct_door`,
  `a_line_profile_beside_one_meridian_cap_refuses_at_the_direct_door`,
  `the_line_arm_carries_a_hand_split_drum_seam_to_its_foot_at_the_direct_door`).
  The shell, which takes an `AtRestBody`, no longer does.
- **The boolean's single-operand fallback joining station vertices**:
  an at-rest operand holds none, so the fallback's join is a no-op on
  every input it can be given (`band_subdivided_side_walls.rs`,
  `a_stationed_prism_is_construction_state_until_joined`). Not dead code:
  the same output stage joins what the boolean's own cuts and merges
  leave, which is the join's real work; only the operand-borne case is
  gone.

- **The boolean's dual result gate's check 11**
  (`crates/topo/src/boolean/ops.rs`, `structural_gate`): no boolean
  output reaches it. The output stage ends with the join, which takes
  every vertex the same predicate reads, and nothing between the join
  and the gate makes a vertex (`sort_into_pieces` only sorts faces into
  solids). It is the output's postcondition, kept as the gate the
  `f64` path's tier 3 is; its witness is a hand-split body handed to
  the gate directly
  (`boolean::ops::tests::at_a_dual_the_result_gate_refuses_a_joinable_vertex`).
  Not on the remove list.

**The remove-or-keep list** is the blend's joined-band arm (with its
naming in `emit_blend.rs` and the joint arm of the clearance screen),
and, if no at-rest witness turns up, the half-band gate on a support
carrying both arcs of one rim.

The join chase those rows also exercised is kept: on the fixture itself
through `Body::join_edges` (`a_rim_joined_twice_reads_through_both_joins`)
and through the shell on an at-rest operand whose cuts carry rulings
(`emit_shell.rs`'s `a_rim_cut_at_three_points_joins_to_the_set_of_its_four_pieces`).
