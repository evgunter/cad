---
id: blend-scaffolding-arms-behind-the-operand-gate
kind: issue
title: "blend: the host gate is reached by a finished pinched host; the screen's strut and lone-vertex arms are unreachable through the doors"
status: closed
opened: 2026-10-07
priority: P3
cost: E
closed: 2026-10-07
pr: 4252
---



`sweep::blend::fillet_edges` and `chamfer_edges`
(`crates/sweep/src/blend/build.rs`) take `&AtRestBody` since
`blend-doors-answer-an-inside-out-operand-with-an-inside-out-body`, and
read `AtRestBody::gate_unverdicted` at the door where no verdict rides
(a dual). So tier 2 refuses every scaffolding state at every scalar
before the battery runs. Three arms were reached only by rows that
built scaffolding operands; each is now settled:

- **The hostless host gate** (`surgery.rs`, "a hostless-crossing rim's
  host face carries edges outside the requested chain in its outer
  cycle") — reached by a finished operand: a coplanar triangle cut into
  the repaired cylinder's base disc at a rim vertex pinches the disc's
  outer cycle, and that body finishes. Row:
  `fillet_h5_r2_probes::a_finished_pinched_host_refuses_at_the_hostless_gate`.
- **The face-clearance screen's lone-vertex arm** and **its
  uncertified-carrier arm** (`battery.rs`, `screened_loop`) —
  unreachable through the doors: `LoopBoundary` and `CurveGeom` have
  two variants each, and tier 2's check 1 refuses every `Empty` loop and
  check 4 every `NullScaffold` curve, at every scalar, before the
  battery runs. Kept as defence for direct `run_battery` callers, which
  read a raw body, and said so at the site. Rows pinning that the
  scaffolding operands do not finish:
  `scaffolding_on_a_blend_support_face_does_not_finish::*`.

The half-band gate's curved single host was never on this list: a
cylinder wall merged into one face over its wrap edge
(`kef_describing`, either meridian) finishes and reaches it
(`fillet_h5_r2_probes::a_finished_curved_single_face_carrying_both_arcs_refuses_at_the_half_band_gate`).
