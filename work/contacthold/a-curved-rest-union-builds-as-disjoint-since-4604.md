---
id: a-curved-rest-union-builds-as-disjoint-since-4604
kind: issue
title: Since PR 4604 a frustum's support plates at δ = 0 union to a disjoint Assembly: a curved-on-curved rest is recorded but not read as a touch
status: closed
opened: 2026-10-11
priority: P0
closed: 2026-10-12
---


`sweep::all operand_gate_support_plates::plates_at_the_exact_support_of_a_frustum_and_a_torus_are_right_at_every_op`
(a slow-set row) fails on main. The INTENT orchestrator reproduced it at
ae9af7dd88:

```
[ε 1e-9] frustum full s 1e-3 dir0 … δ 0e0 a∪b: Body(Assembly): touching operands built as disjoint
```

The same failure shows for b∪a and for dir1, at
`crates/sweep/tests/operand_gate_support_plates.rs:283` (panic at `:335`).

A first-parent bisect (`ec191b4ade` good, `ae9af7dd88` bad) names merge
`758bd7acb4`, PR 4604 (`contacthold/at-rest-cell-pairs-census`). Its
parent `f0029af4f7` passes.

Likely cause, unconfirmed:
- PR 4604 made `ops.rs`'s `record()` infallible and removed the
  `BooleanError::CurvedRestUnrecorded` refusal. A curved edge resting on
  a curved face is now recorded instead of refused.
- The row skips a refusal (`let Ok(res) = out else { continue }`), so it
  used to pass while the union refused.
- The union now builds but classifies the result `Assembly` (disjoint),
  so whatever decides glued vs disjoint does not read a curved-on-curved
  rest as a touch.

First check: print `verdict(&out)` for "frustum full δ 0 a∪b" on
`f0029af4f7` and on main.

It reds every PR whose diff selects sweep's slow set. Lanes merge over
it with a PR comment citing this item.
