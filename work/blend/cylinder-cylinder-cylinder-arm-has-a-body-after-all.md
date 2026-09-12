---
id: cylinder-cylinder-cylinder-arm-has-a-body-after-all
kind: issue
title: The CylinderCylinderCylinder arm's 'no body yet' is falsified by the extrude door: the arm is reachable and has no row
status: open
opened: 2026-09-09
---


`crates/sweep/README.md`'s FILLET-H7 paragraph closes with:

> The `CylinderCylinderCylinder` consumer — two parallel cylinders
> unioned at a common ruling — has no body yet: the union refuses at
> the boolean's curved-pierce door, and so does a block ∪ cylinder at
> its join lane; that is the boolean's ground, not the band's.

The first clause is true about the UNION and false about the arm. The
extrude door reaches the same shape, and the arm carves on it.

## Measured

A profile of two flatted circles fused at a common chord — `R = 0.5`,
each flat at `0.3` from its own centre (the 3-4-5 point, so both major
arcs have bulge exactly `2.0`), centres `0.6` apart — extruded `L = 2`:

| reading | value |
|---|---|
| census | `v=4 e=6 f=4`, tier 3 OK |
| volume | `2.694297435588181` against the closed form `2·L·(πR² − R²(φ − sinφ·cosφ))` with `cos φ = 0.6` = `2.69429744` |
| battery over the two straight creases | `BlendArm::CylinderCylinderCylinder` on both, `ChainClosure::Open`, 4 transverse caps |
| `fillet_edges(&body, &creases, 0.1, tol)` | Ok, tier 3 OK, 6 faces |
| ΔV | `+0.0026920065` — material ADDED (the concave side) |
| ΔV against an independent grid integration of the added section | `0.000673003` per crease per unit length vs `0.000673002` measured (2.5e-6, grid-limited) |

The union clauses re-measured on the same head, with cylinders built
the way the kernel's own fixtures build them (an EXTRUDED circle, one
cylindrical wall — a revolve of an axis-touching profile gives
seam-split walls and refuses `NonMaximalFaces` for an unrelated
reason):

| operation | refusal |
|---|---|
| parallel cylinder ∪ cylinder | `CurvedPierceUnsupported { operand: A, face: FaceKey(3v1), edge: EdgeKey(1v1) }` |
| cylinder ∪ box | `CurvedPierceUnsupported { operand: B, face: FaceKey(3v1), edge: EdgeKey(12v1) }` |
| block ∪ a sunk rod | `CurvedSectorSideUnsupported` (already filed: `work/bool/slab-cut-cylinder-refuses-sector-side`) |

So the boolean sentence stands; the "no body yet" sentence does not.

## What the taker owes

1. Correct the README clause: the arm has a consumer through the
   extrude door, and only the *union* spelling of the shape refuses.
2. A row. The arm ships with no end-to-end fixture — the H7 suite's
   rod rows are all `CylinderPlaneCylinder` — so its band derivation,
   its two trimlines and its transverse cut-off are carried by no test
   on any body. The fixture above is cheap, closed-form on both the
   source volume and the carve, and exercises the CONCAVE material
   side, which no ruled row does today.

Refs FILLET-H7 (#1736), `crates/sweep/tests/fillet_h7_transverse_cap.rs`.
