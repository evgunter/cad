---
id: blend-scaffolding-arms-behind-the-operand-gate
kind: issue
title: blend: the host gate, the half-band gate's slit arm and the screen's strut and lone-vertex arms read operand scaffolding the door's at-rest gate now refuses
status: open
opened: 2026-10-07
priority: P3
cost: E
---



`sweep::blend::fillet_edges` and `chamfer_edges`
(`crates/sweep/src/blend/build.rs`) take `&AtRestBody` since
`blend-doors-answer-an-inside-out-operand-with-an-inside-out-body`, and
read `AtRestBody::gate_unverdicted` at the door where no verdict rides
(a dual). So tier 2 refuses every scaffolding state at every scalar
before the battery runs. Four arms were reached only by rows that
built scaffolding operands, and those rows now pin that the operand
does not finish:

- **The face-clearance screen's lone-vertex arm** (`battery.rs`,
  `screened_loop`, "a support face carries a lone-vertex cycle") —
  tier 2's `ScaffoldingEmptyLoop`.
- **The screen's uncertified-carrier arm** (`battery.rs`, same
  function, "a support face's boundary edge carries no certified
  carrier") — reached by a null strut, tier 2's `ScaffoldingStrutVertex`
  and `NullEdgeAtRest`. Rows:
  `band_clearance_screen_reads_every_feature::*_does_not_finish`.
- **The hostless host gate** (`surgery.rs`, "a hostless-crossing rim's
  host face carries edges outside the requested chain in its outer
  cycle") — reached by a strut spur in the cap's outer cycle. Row:
  `fillet_h5_r2_probes::a_host_with_a_strut_spur_in_its_outer_cycle_does_not_finish`.
- **The half-band gate's curved single host** (`surgery.rs`,
  `HostSide`'s "One shape this door does not serve", "a curved support
  does not carry exactly its own rim arc") — reached by killing a sphere
  wall's seam meridian, which leaves the pole a strut tip. Row:
  `fillet_h5_r2_probes::a_curved_single_face_carrying_both_arcs_over_a_slit_does_not_finish`.

The other uncertified-carrier reads in `surgery.rs` may be reached by
the surgery's own mid-construction bodies, so they are not listed here.

Owed: for each arm, find a finished operand (or a mid-surgery body)
that reaches it, and row it; or retire it as unreachable and say so at
the site.
