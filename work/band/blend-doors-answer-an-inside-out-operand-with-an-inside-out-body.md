---
id: blend-doors-answer-an-inside-out-operand-with-an-inside-out-body
kind: issue
title: fillet_edges and chamfer_edges take an operand no at-rest gate read: an inside-out prism comes back as an inside-out blended body
status: open
opened: 2026-10-06
priority: P1
cost: M
---



Found by the sweep of CLEAVE's
`split-answers-an-inside-out-operand-with-two-inside-out-halves`
(branch `cleave/split-operand-gate`), which makes the split's doors take
`AtRestBody`, as the Boolean's already do.

`sweep::blend::fillet_edges` and `chamfer_edges`
(`crates/sweep/src/blend/build.rs`) take `&Body` and read nothing about
the operand's finish before the battery: no tier-2 read, no orientation.

Measured on main `575b309d`, `Tol::witness()`: `prism_z` over the
clockwise triangle (0,0), 190°, 80° (unit radius), z ∈ (0.5, 1), volume
−0.2349 (inside-out; tier 3's check 7 refuses it `NegativeVolume`).
Requesting all nine edges at size 0.02:

- `fillet_edges` returns `Ok`, volume −0.23355; the counterclockwise
  wedge's fillet is +0.23355. `validate_geometric` refuses the inside-out
  result (one finding).
- `chamfer_edges` returns `Ok`, volume −0.23331 (counterclockwise:
  +0.23331), refused by `validate_geometric` likewise.

So each door answers for the complement and ships a body tier 3 refuses.
A single edge refuses in both orientations (the run-out arm), which is
why the shell row that first named these doors
(`work/shell/shell-answers-for-the-complement-of-an-inside-out-operand.md`)
could not measure them.

On the slit dome of `crates/sweep/tests/pole_slit_window.rs` (`slits`,
a valence-1 strut tip at the pole), filleting each edge refuses with
unrelated reasons (a valence-2 corner, tangential supports, a chart-seam
vertex): none says the operand is not finished.

What closes it: the blend doors take `AtRestBody` (their own adoption
unit, per `work/reach/boolean-door-adopts-the-finished-body-type.md`), or
read tier 2 and check 7 per solid at the door where no verdict rides
(`AtRestBody::gate_unverdicted`, shared by the Boolean and the split).
