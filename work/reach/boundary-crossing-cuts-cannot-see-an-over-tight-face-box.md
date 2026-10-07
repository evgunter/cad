---
id: boundary-crossing-cuts-cannot-see-an-over-tight-face-box
kind: issue
title: Only crest cuts and direct box rows can see a face box tighter than its face; boundary-crossing cuts cannot
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Found by the REACH review of PR 4123 (Q3; executed with the mutant M2,
`lo + 0.05` on the sphere rectangle's latitude).

A cut that crosses a face's boundary refuses at the split gate however
tight that face's box is, because the cut also meets the face's
boundary edges, which the gate reads on their own. So a row built from
boundary-crossing cuts passes over a box TIGHTER than its face, which
is the unsound direction. Under M2 the PR's into-row
(`crates/sweep/tests/reach_split_gate_azimuth.rs`
`every_cut_into_a_partial_turn_sphere_face_refuses_at_the_gate`)
stayed green until interior-crest cuts were added to `planes`. The
reviewer's own split probe also stays green under M2 at `s = 1`.

Two kinds of row see an over-tight box today:

- **interior crest cuts**, `δ` into the face along its normal at
  points inside it, which meet the face in a small circle and no edge
  (`reach_split_gate_azimuth.rs` `planes`; the 1e-7 cuts of
  `reach_split_gate_window.rs`
  `a_cut_into_the_faces_window_refuses_at_the_gate`);
- **direct box rows** against a dense sample of the face
  (`boxes.rs` `sphere_rect_rows`, `the_*_arms_box_is_exactly_the_construction_its_rule_states`);

**The unit:** any new box rule, or tightening of one, ships a crest-cut
row or a dense-sample box row of its own, not only boundary-crossing
cuts. Sweep the existing per-kind box rows (cone, torus, cylinder) for
a crest or dense-sample row each, and add one where it is missing.
