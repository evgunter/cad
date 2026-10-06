---
id: exchange-writers-take-a-body-no-at-rest-gate-read
kind: issue
title: The STEP writer refuses a null edge by hand and reads neither the rest of tier 2 nor orientation; the STL writer reads neither
status: open
opened: 2026-10-06
priority: P3
cost: M
---



Found by the sweep of CLEAVE's
`split-answers-an-inside-out-operand-with-two-inside-out-halves`
(branch `cleave/split-operand-gate`), whose class is a door that reads an
input body with a hand-written scaffolding check, or none.

- `step_export::step_string` / `write_step` take `&Body<f64>` and refuse
  a null edge by hand (`writer.rs`, the `CurveGeom::NullScaffold` arm →
  `StepExportError::NullScaffoldEdge`); the crate doc promises
  "mid-surgery bodies" refuse. Nothing reads the rest of tier 2 (a
  strut, an empty loop, a split shell) or orientation, so a strut-bearing
  or inside-out body would be written as a solid.
- The STL writer reads neither.

**Unmeasured**: what each writes for the slit dome of
`crates/sweep/tests/pole_slit_window.rs` and for the clockwise wedge of
`crates/topo/tests/split_operand_gate.rs`. The repair shape is the
Boolean's and the split's: take `AtRestBody`, or read tier 2 and check 7
at the door (`AtRestBody::gate_unverdicted`) and refuse typed, so the
hand-written null-edge arm becomes an invariant.
