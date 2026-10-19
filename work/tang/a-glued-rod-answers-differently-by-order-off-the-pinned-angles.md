---
id: a-glued-rod-answers-differently-by-order-off-the-pinned-angles
kind: issue
title: A rod glued in its bore still answers differently by operand order one step off the pinned angles, and coaxial nested equal rods do too
status: open
opened: 2026-10-15
priority: P3
cost: M
---

Found by PR 5188's review (off-target, pre-existing: identical at its merge-base).

`crates/sweep/tests/bisector_nodes.rs::a_rod_glued_in_its_bore_answers_alike_in_either_order` pins the orders alike at three angles of a family where it does not hold in general. The fixture is that row's: a unit rod against the brick `[−2, 2]³` bored to its radius, ∩ and ∪ in both orders.

## Measured (the reviewer's probe)

- **Default ε, 1e-7°:** both orders escalate on one margin, `1.3593e-9`, but at two decisions: `Containment { Loop(Boundary) }` rod-first and `Coincidence(VertexOnCurvedFace)` bore-first. The row holds this pose to the margin alone and cites this item.
- **Default ε, 3e-8°:** both orders escalate `WallRoots(AxisParallel)`, on different margins: `1.0472e-9` rod-first against `2.0944e-9` bore-first.
- **ε 1.2e-10:**
  - at 2e-9° and 3e-9°: `CurvedPierceUnsupported` rod-first, a `WallRoots` escalation bore-first;
  - at 5e-9°: `VertexOnCurvedFace` on `−1.36e-10` against `WallRoots` on `3.49e-10`.
- **ε 1e-12, 1e-10°:** `Containment` against `VertexOnCurvedFace`.
- **Coaxial nested equal-radius rods, 0 … 1e-8°:** ∩ builds A-first and refuses `FallbackExtentUnsupported` B-first; ∪ the reverse.

## The recourse

A rod glued in a bore of its own radius is the coincidence a shared construction decides. D4(i)'s "make the two one construction (D10)" fits it better than the escalation's "tighten the tolerance below m/K". Following that advice at ε 1.2e-10 just escalates elsewhere (`Containment` on `1.36e-10`), and at 1e-12 refuses `CurvedPierceUnsupported`.
