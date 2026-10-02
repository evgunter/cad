---
id: the-chart-sweeps-first-order-box-reads-its-derivative-off-the-whole-span-cell
kind: issue
title: ssi: the chart sweep's first-order box reads its derivative off the whole span cell, so on a one-span wall it never tightens below the cell
status: open
opened: 2026-10-02
priority: P2
cost: M
---


Found by the lane on `plane-nurbs-tube-straddles-a-curved-dome-at-coarse-eps`
(branch `ssi/dome-tube`), 2026-10-02.

## What

`NurbsBoxes::deriv_box` (`crates/geom-brep/src/ssi/enclose.rs`) now
cuts each span cell's net to the rectangle before it reads the quotient
rule's hull (`CellNet::cut`, blossoming in certification arithmetic),
and meets that box with the whole cell's. So limb 3's chart tube sees
the wall's derivative over its own window.

The exhaustiveness sweep does not. `sweep_chart_plane`
(`ssi/exhaust.rs`) excludes cells with `NurbsBoxes::rect_box`, and the
seeding subdivision rides on that box. `rect_box` still reads its
`S_u` and `S_v` boxes off every touched cell whole
(`NurbsBoxes::cell_deriv_box`). Take a wall of one span, such as the
3×3 quadratic dome of `plane-nurbs-ssi-does-not-certify-a-curved-dome`.
There that is the whole patch's derivative hull at every cell size, so
the box never gets narrower than `|S_d|_patch · h`, however small the
cell is.

## Why it was not moved with limb 3

These were measured with `rect_box` on the cut box, at the default ε:

- `m5_pr7_ssi::an_unseeded_chart_run_refuses_typed_rather_than_receipting_an_unprovable_domain`
  fails its mode pin `out.seeds > 0`. Its fixture, `hull_slack_wall`,
  misses the plane by 2 mm under a net that crosses the plane by
  50 mm. It exists to keep cells alive down to the seed floor on the
  hull's slack, and the cut box excludes them above that floor. The
  fixture needs a near miss that the cut box still cannot separate.
- At ε 1e-9 the dome's tilt cuts still refuse limb 2 or limb 1, but
  the margins move a little: HullSup 5.07e-9 → 4.98e-9 at d = 1.5 and
  1.13e-8 → 1.19e-8 at d = 2, OnLocus 6.56e-9 → 7.25e-9 at d = 3. The
  sweep hands the marcher different seeds.

So the move changes seeding. That was outside that lane's fence while
`[ev]` PR 3862 rebuilds the plane × NURBS boundary handling.

## Next

- Call `deriv_box` from `rect_box`, and delete `cell_deriv_box`.
  `deriv_box` is never wider than `cell_deriv_box`, since it meets
  the two, so the sweep can only exclude more.
- Retune `hull_slack_wall` so the mode holds on the tighter box.
- Re-pin whatever moves.

Expect fewer examined cells on single-span walls.
