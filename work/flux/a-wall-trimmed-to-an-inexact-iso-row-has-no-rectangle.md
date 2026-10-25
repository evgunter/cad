---
id: a-wall-trimmed-to-an-inexact-iso-row-has-no-rectangle
kind: issue
title: A NURBS wall trimmed to an iso-row its rim reaches only to rounding refuses the rectangle lane
status: open
opened: 2026-10-25
priority: P2
cost: M
---


Filed by FLUX's `flux/rational-rim-vector-area` lane, when the moved cap
it closes stopped refusing and uncovered the next face.

## Measured

`crates/sweep/tests/encl_curved_loft_shell.rs`'s
`shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_moves_alone_onto_a_wall_row`
moves the vase's bottom cap 0.05 into the body. The cap now measures
(`geom_brep::props::rational_rim`). Tier 3 then refuses at one of the
two spline walls, at every ε row (1e-9, 1e-12, 1e-6):

`VolumeUncomputable { source: Face { face: <a wall>, source: QuadratureUnsupported { what: "the NURBS-face boundary does not traverse its UV rectangle exactly once (shoelace ≠ ±rectangle area) …" } } }`

The wall's traversal polygon, as `nurbs_face`
(`crates/topo/src/props/quad_lane.rs:1285`) reads it off its half-edges'
lifted images, is

```text
[(1.0, 1.0), (0.0, 1.0), (7.248337464615123e-30, 0.024999999999974973), (1.0, 0.024999999999999998)]
```

with shoelace 0.9750000000000125 against a rectangle area of
0.9750000000000251. The rim's `IsoLine` sits on the row
`0.024999999999999998`, but the seam's image read at its cache's end
parameter reaches `v = 0.024999999999974973` and `u = 7.2e-30`: an end
of the seam that the rim reaches only to rounding. The rectangle
certificate needs exact corners (the exact-structure read, the
axis-aligned test at `:1368` and the shoelace equality at `:1411`), so it
refuses.

## What it blocks

The vase with its cap moved alone, end to end: the cap's volume share
is read, but the body's volume is not. Any spline wall a moved cap
trims to a row hits the same refusal.

## Directions

Either the producer (`topo::replace_face_offset`'s rim minting) lands
the seam's end exactly on the rim's row, or the rectangle lane reads a
row its corners reach within their certified envelope. The second is
the cut-loft class's trimmed lane, which already reads `General`
images.
