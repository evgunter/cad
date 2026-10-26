---
id: an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane
kind: issue
title: An analytic operand edge against the other operand's NURBS face refuses CurvedBooleanUnsupported at the crossing layer: the contact klein wall 3, lily wall 8 and teapot wall 3 reach once the edge gate is gone
status: open
opened: 2026-10-26
priority: P0
cost: H
design: true
refs: [delete-the-boolean-operand-edge-gate, nurbs-edge-crossing-rung-is-the-ring-composite, a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm, nurbs-plane-section-has-no-component-arm, 5723]
---


Filed by the SPLICE orchestrator, from the gate lane's measurement on
PR 5723 (`splice/gate`, the operand edge gate deleted).

## What

With `gate_operand_edges` gone, all three demo joins refuse **at their
contact**, and at the same site:
`BooleanError::CurvedBooleanUnsupported { operand: B, face, kind: Nurbs }`.
- **klein wall 3**: the bulb's outer neck-cylinder seam LINE (edge 2v1
  on face 4v1, r = 0.275, z ∈ [2.58, 3.0]), which ends on the z = 3 rim
  that the loop's top cap meets, is read against loop face 3v3. That
  face is one of the loop's four NURBS walls and rises from that rim.
- **lily wall 8**: the blade's base-section rim circle, against a
  sheath NURBS wall rising from the same rim.
- **teapot wall 3**: a belly latitude circle (sphere | sphere), against
  a spout NURBS wall.

So the first thing each join needs is an analytic edge (line, circle)
of one operand read against a NURBS face of the other, and no crossing
layer reads a spline face. It is not
`nurbs-edge-crossing-rung-is-the-ring-composite`: that rung composes a
face's implicit residual along a NURBS EDGE (`spline::compose`, f ∘ C),
and a NURBS face has no implicit residual to compose. It is not
SECT's `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`
either: that is the face × face section the klein and teapot pierces
need after the contact.

Lily wall 8 was this track's first demo witness on the premise that it
needs no curved section. It still needs this row.

## Open (designers first)

Where and how the crossing is found has more than one viable answer:
- a certified curve × NURBS-surface root lane in the crossing layer
  (the edge's points on the surface, by subdivision of the surface's
  net against the edge's carrier);
- the crossing read off the section of the edge's own faces with the
  NURBS face, as SECT's ratified section-first design reads chords
  (`work/sect/plan.md`, "The design"). C5's rung 3 has plane × NURBS,
  and has no cylinder × NURBS or sphere × NURBS arm;
- or something the designers find.

The contact here is also a coincidence: the edge lies on the rim the
NURBS wall rises from. Whether that coincidence is decided at the
crossing layer, or by the `[coincidence]` door before it, is part of
the question. One Opus and one Fable designer weigh it
(`memories/orchestration-model.md`) before a lane builds.

Reachable once PR 5723 merges.
