---
id: split-gate-approx-arm-has-no-whole-split-row
kind: issue
title: The split gate's Approx arm is read at the gate only: no public door builds a body with an Approx face to split whole
status: open
opened: 2026-10-02
priority: P3
cost: M
---


Filed by the split-gate lane (PR 3843).

`splitting/classify.rs` `gate_operand` clears an `Approx` face behind
its padded reach box, the fit's control net (`FaceBoxRule::ControlNet`
on `Approx::fit`). The only row that reads that arm is
`topo/tests/split_gate_per_face.rs`, at the gate: the unit cube with its
top face's surface swapped for an offset `Approx`, through
`split_reduce` and `vertex_sides`. No whole split runs through an
`Approx` face, because no public construction builds a body carrying
one (`mesh/tests/d9_mesh_goldens.rs` module docs say the same of the
mesher's goldens).

**When** a public door mints an `Approx` face on a solid (the offset or
shell lanes are the likely first), add a `sweep` row splitting that
solid with a plane clear of the face, checked against an independent
oracle through both halves at the three ε, and one with a plane that
may meet it refusing at the gate.
