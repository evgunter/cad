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
it closes stopped refusing and uncovered the next face; moved to SHELL,
whose producer mints the corners (below).

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
0.9750000000000251. The wall's pcurves, printed (PR 5696's review): the
rim's `IsoLine` has `p0 = (7.248e-30, 0.024999999999974973)`, 2.5e-14
below the row with a `u` offset of 7.2e-30, so its start is the third
corner; the fourth, `(1.0, 0.024999999999999998)`, is the `u = 1` seam
read at its cache's start, one ulp below the row. Neither corner is the
row and they are not each other's, so the polygon is a sliver off a
rectangle. The rectangle certificate needs exact corners (the
exact-structure read, the axis-aligned test at `:1368` and the shoelace
equality at `:1411`), so it refuses.

## What it blocks

The vase with its cap moved alone, end to end: the cap's volume share
is read, but the body's volume is not. Any spline wall a moved cap
trims to a row hits the same refusal.

## Whose it is

The producer's, so SHELL's (`crates/topo/src/replace_face.rs:1376`,
`replace_face_offset`). The moved plane is `z = 0.05` exactly, and the
loft's height is `2v`, so its row is `v = 0.05/2 = 0.025` exactly in
`f64`. The rim's `IsoLine` is minted 2.5e-14 below it, at
`v = 0.024999999999974973`, and off `u = 0` by 7.2e-30; the `u = 1`
seam's cache starts one ulp below it, at `0.024999999999999998`. The
patch lane's exact-rectangle certificate is doing its job: the minted
corners do not meet. Minting the rim's `IsoLine` on the exact row with
`p0.u` exactly `0`, and starting the seams' caches on that row, would let
the wall take the rectangle lane as it stands.

## For ISO

ISO's vase row (`iso/vase-cap-moves-alone`, the witness above) now gets
past the cap. The cap measures on `flux/rational-rim-vector-area`, and
the body stops at this wall instead.
