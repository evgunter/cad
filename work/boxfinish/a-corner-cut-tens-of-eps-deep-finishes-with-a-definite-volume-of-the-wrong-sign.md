---
id: a-corner-cut-tens-of-eps-deep-finishes-with-a-definite-volume-of-the-wrong-sign
kind: issue
title: A split cutting a corner 20 to 50 eps deep is refused at rest as NegativeVolume, a definite verdict that the corner piece is inside out
status: open
opened: 2026-11-19
priority: P2
cost: M
---

Found by `oneround/section-area`'s end-to-end probe (2026-11-19,
ε = 1e-9, `f64`), beside the section-area defect that unit fixes. The
cause is not traced: what follows is the payload. The outcomes were
read on main at `7b752a7a` and again on PR 7146's head; they are the
same on both except where a row says otherwise.

## The body and the cut

A plate through the public doors: `sweep::test_support::prism` over
`(0,0) → (10,0) → (10,1) → (0,1)`, the side from `(10,1)` to `(0,1)` an
arc of radius `R` bulging outward, extruded 1 along `+z`, finished at
rest. `topo::split` by the plane of unit normal `n = (1,1,1)/√3` through
`(10,1,1) − d·n`, which cuts off the corner where the arc wall, the
`x = 10` wall and the top cap meet: a tetrahedron `d` deep, of volume
about `d³`.

## Measured

| R | d | `split` returns |
|---|---|---|
| 1e4 | 50 ε | `Finish(ResultNotAtRest { side: Above, findings: [NegativeVolume { .. }] })` |
| 1e3 | 50 ε | the same (the review of PR 7146, 2026-11-20) |
| 20 | 20 ε, 50 ε | the same |
| 1e4, 20 | 200 ε and deeper | `Ok` |

- The corner piece is on the side its section's normal faces, a solid of
  positive volume about `1e-22`. `NegativeVolume` is a definite verdict
  that it is inside out.
- The split refuses, so no wrong body is returned: the defect is the
  stated reason.
- `R = 20` shows the wrong sign is not a far wall's alone.

At `R = 1e4`, `d = 20 ε` the split returns
`VolumeSignUnresolved { limit: Quadrature, volume: Some((-7.67e-13, 1.80e-14)) }`
(the bits on PR 7146's head: `-7.665647174464286e-13`,
`1.803049920254766e-14`; on `7b752a7a` they differ from the thirteenth
digits). The pair is a bracket `(lo, hi)`: it holds zero and the true
volume, and an unresolved sign is the right refusal for it. What it
shows is the bracket's width, ten orders above the solid's volume
(`work/boxfinish/a-cylinder-faces-flux-sums-two-terms-of-its-far-axis-origins-size.md`).

## Unmeasured

Which reading produces the sign (the aimed rays or the volume
quadrature).
