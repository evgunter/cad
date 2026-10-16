---
id: inside-out-operand-reading-has-two-homes-and-four-spellings
kind: issue
title: The inside-out operand reading has two loops, four spelled-out recourse sentences and convention-held unreachable arms
status: open
opened: 2026-10-12
priority: P3
cost: E
---


Left by PR 4725's final re-review (style notes, none gating).

- The recourse "build it with its faces pointing outward, or revert it" is written out four times: `validate.rs` (~L3442, ~L3798, ~L9228) and `solid_contain.rs` (~L645). Give it one home. RESTFRONT's `an-inside-out-findings-end-on-the-kernel-defect-ending` is the neighbouring class.
- `reduce.rs` `gate_sliver_orientation` and `shell_witness::inside_out_operand` both loop over solids reading `SolidFaces::of(..).read_as_given()` through `point_in_solid_faces`: two homes for one reading.
- `census::point_hull` is shared but lives inside the census, one of its consumers.
- The census `unreachable!` on `InsideOut` holds only because no census selection sets `as_given`, and nothing enforces that. The gate's `unreachable!` on `OnBoundary` likewise rests on the probe point's 1 m margin.
- In the gate, `let Ok(sel) = SolidFaces::of(..) else { continue }` can never fail, because the keys come from `body.solids()`. That makes it a silent skip that never runs.
- `inside_out_operand` could return `InsideOutOperand { errors: [] }` if the whole-body read refuses while no single solid does. Not reached.
- The gate's refusal now precedes more specific ones. A valid thin tube cut across used to refuse `CurvedPierceUnsupported` and now refuses `Containment` from the gate. Nothing pins which story wins.
