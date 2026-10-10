---
id: dual-operand-read-passes-an-orientation-it-cannot-measure
kind: issue
title: At a dual, gate_unverdicted passes an inside-out operand whose check 7 needs the quadrature lane, and every tier-3-only finding
status: open
opened: 2026-10-07
priority: P3
cost: M
---


Found by the review of PR 4252 (the blend doors take a finished
operand); its fix pass confirmed the second half below.

`topo::AtRestBody::gate_unverdicted` (`crates/topo/src/validate.rs`)
is what the three operand-gated doors read where no verdict rides (a
dual, whose `AtRestPolicy` runs no at-rest gate): the Boolean
(`reduce::gate_unverdicted_operand`), the split, and since PR 4252 the
fillet and chamfer (`sweep::blend::build::operand_gate`). It is weaker
than tier 3 in two ways, so a body `f64` refuses at the gate reaches
each door at a dual.

- **Orientation it cannot measure passes.** The read calls
  `wound_negative(&body, band, tol, T::quad_lane())`. At a dual
  `quad_lane()` is `None`, and `wound_negative` keeps only
  `NegativeVolume` from check 7's errors (`plus_v_by_sign`), dropping
  `VolumeUncomputable`. So an inside-out solid whose volume needs the
  quadrature lane (a face outside the closed-form inventory) passes
  the dual door, where `f64` takes the lane, decides the sign negative
  and refuses. Tier 3 also refuses `VolumeUncomputable` itself; the
  dual read passes it. Reasoning by inspection; no witness was built.
- **Tier-3-only findings are not read at all.** By design the read is
  tier 2, checks 7 and 10, and check 11 (which answers at every scalar
  the join reads at), so every other tier-3 finding passes.
  Measured: a full-revolve cylinder whose wall is merged into one face
  by plain `kef` on one seam meridian leaves the survivor a slit, which
  tier 3 refuses (`DescriptionNotAdjacent`, "an edge whose halves bound
  one face is that face's wrap edge"), so it does not finish at `f64`.
  At a dual it passes `gate_unverdicted` and reaches the blend's
  half-band gate. The `kef_describing` spelling (the survivor restated
  as the wall's wrap edge) no longer finishes at `f64` either: each
  rim's two arcs meet at the killed meridian's end, which tier 3's
  check 11 refuses
  (`fillet_h5_r2_probes::a_curved_single_face_carrying_both_arcs_is_construction_state`),
  and `gate_unverdicted` reads check 11 at a dual as well
  (`unjoined_operand_at_a_dual.rs`).

Owed: decide what a dual door owes an operand it cannot certify. One
answer: a sign the dual cannot measure refuses, as tier 3 refuses it,
rather than passing. The alternative is to say at `gate_unverdicted`,
and in each door's contract, that the dual's read is tier 2 plus a
best-effort orientation read, and to name the gap there. Row the
first case with a dual operand whose check 7 needs the lane.
