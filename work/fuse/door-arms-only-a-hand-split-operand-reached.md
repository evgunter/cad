---
id: door-arms-only-a-hand-split-operand-reached
kind: issue
title: Door arms only a hand-split operand reached have no at-rest input since tier 3's check 11
status: open
opened: 2026-10-08
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
  `blend_band_reach_chain_ends.rs`.
- **The blend's half-band gate on a curved support carrying both arcs**
  (`crates/sweep/src/blend/surgery.rs:1333`): its row,
  `fillet_h5_r2_probes.rs`'s merged wall, holds the killed meridian's
  ends between two arcs of each rim, so it is construction state.
  Whether another at-rest operand reaches the gate is open.
- **The shell's axial corners at a hand-split vertex**
  (`crates/topo/src/offset_axial.rs:1281`, "no profile constraint";
  `:1322`, "a line profile and a plane parallel to the axis"; and the
  line arm on a vertex whose only surface is a cylinder): the rows in
  `shell7_seam_corner.rs` say no door builds such a vertex, and the
  hand split that made one is construction state now.
- **The boolean's single-operand fallback joining station vertices**:
  an at-rest operand holds none, so the fallback's join is a no-op on
  every input it can be given (`band_subdivided_side_walls.rs`,
  `a_stationed_prism_is_construction_state_until_joined`).

The join chase those rows also exercised is kept: on the fixture itself
through `Body::join_edges` (`a_rim_joined_twice_reads_through_both_joins`)
and through the shell on an at-rest operand whose cuts carry rulings
(`emit_shell.rs`'s `a_rim_cut_at_three_points_joins_to_the_set_of_its_four_pieces`).
