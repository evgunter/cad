---
id: lily-wall-8s-leaf-wall-lifts-no-closed-trim-at-the-pierce-pass
kind: issue
title: Lily wall 8's union refuses HeldPierceUnbuilt (placed: false): the leaf's other NURBS wall reads no closed lift of its loop, so its trim cannot say whether it holds the pierce point
status: open
opened: 2026-11-25
priority: P0
cost: M
refs: [a-nurbs-face-against-a-torus-has-no-section-arm, the-chart-accounting-clears-a-thin-torus-only-in-sub-millimetre-cells, a-spline-wall-cannot-say-whether-it-holds-a-pierce-point]
---

Filed by SSIARITH (`ssiarith/thin-torus-exclusion`), which cleared the
cell budget lily wall 8's torus × spline section stopped at
(`work/ssiarith/the-chart-accounting-clears-a-thin-torus-only-in-sub-millimetre-cells.md`).

## What

With that section accounted, lily wall 8 (`demos/tour/src/lily.rs`,
`wall_probes`: the lower stem's torus tube unioned with the long leaf)
reaches the pierce pass and refuses there, at ε 1e-9 and 1e-6 and the
tour's own scalar, after about 30 s and 20 s (`ci-tour` profile):

> `HeldPierceUnbuilt { operand: B, face: FaceKey(4v1), pierced: FaceKey(3v1), kind: Nurbs, pierced_kind: Torus, placed: false }`

The held face is the leaf's other NURBS wall (not the ridge wall the
germ section reads), and the pierced face is the lower stem's torus
wall. The wall's pin now names this stop.

## Where it stops (measured)

A temporary probe in `topo::spline_region::read_capped` and
`topo::spline_trim::Trim::of`, on that run:

- `boolean::vtxfac::held_reads` reaches
  `contain::placement_unheld` → `spline_region::spline_face_placement`,
  whose reading is `Unsettled(None)`, so the placement is `Trim(None)`
  and the pass refuses with `placed: false`.
- Four of the reads end at `Trim::of` returning `None`, every one at
  `TrimChart::chain` refusing `SplineTrimGap::NoClosedLift`. The other
  ten end their carrier walk with no foot (off the carrier).

`NoClosedLift` is any of three things (`spline_trim.rs`, `chain`): the
lift is empty, a row lacks a joint element, or the joints wind. Which
of the three it is was not measured.

## What would close it

Read which of the three it is, and make the leaf's wall's trim say
whether it holds the point; then re-measure wall 8 and re-pin it.

Banded P0 because lily wall 8 is what
`sectarms/a-nurbs-face-against-a-torus-has-no-section-arm` (P0) waits
on (`work/README.md`, Priority: a blocker inherits the band of what
waits on it). — (SSIARITH orchestrator)

