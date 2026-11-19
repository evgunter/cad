---
id: a-corner-cut-tens-of-eps-deep-finishes-with-a-definite-volume-of-the-wrong-sign
kind: issue
title: A split cutting a corner 20 to 50 eps deep is refused at rest as NegativeVolume, and on a 10 km wall its volume bracket reads -7.7e-13 for a solid of about 1e-23
status: open
opened: 2026-11-19
priority: P2
cost: M
---

Found by `oneround/section-area`'s end-to-end probe (2026-11-19, merge
base `7b752a7a`, ε = 1e-9, `f64`), beside the section-area defect that
unit fixes. The cause is not traced: what follows is the payload.

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
| 1e4 | 20 ε | `Finish(ResultNotAtRest { side: Above, findings: [VolumeSignUnresolved { limit: Quadrature, volume: Some((-7.665647174464286e-13, 1.803049920254766e-14)) }] })` |
| 1e4 | 50 ε | `Finish(ResultNotAtRest { side: Above, findings: [NegativeVolume { .. }] })` |
| 20 | 20 ε, 50 ε | `Finish(ResultNotAtRest { side: Above, findings: [NegativeVolume { .. }] })` |
| 1e4, 20 | 200 ε and deeper | `Ok` |

- The corner piece is on the side its section's normal faces, a solid of
  positive volume about `1e-23` (d = 20 ε) to `1e-22`. `NegativeVolume`
  is a definite verdict that it is inside out.
- At `R = 1e4` the bracket is centred `-7.7e-13` with a half-width of
  `1.8e-14`: ten orders above the solid's own volume, and of the wrong
  sign. It does not hold the true volume, so it is not an enclosure.
- The split refuses either way, so no wrong body is returned: the
  defect is the stated reason and a bracket that excludes the truth.
- `R = 20` shows the wrong sign is not the far wall's alone. Whether the
  two share a cause is not known.

## Unmeasured

Which reading produces the sign (the aimed rays or the volume
quadrature), and which face's term carries the `-7.7e-13`.

