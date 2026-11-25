---
id: the-rest-of-a-junction-edges-span-refuses-its-near-root-at-1e-12
kind: issue
title: Past a junction end's window, the Line × Torus root door refuses the near root a neutral point puts at √(12δ), at ε = 1e-12 from about 5.3√ε to 15√ε
status: open
opened: 2026-11-24
priority: P2
cost: M
---


Found by `heldrecords/belt`'s design scan, 2026-11-24. The measurements
were taken on main, without the belt's change.

## Measured

The scan uses `editor-core` `a_pipe_unions_with_its_elbow`'s
`neutral_off_the_seam(δ)` pose, the union at the run's ε, `K = 10`, in
quarter-decade steps of `δ/√ε`:
- at `ε = 1e-12`, `δ` from `5.3√ε` to `15√ε` (5.3e-6 … 1.5e-5) refuses
  `CurvedPierceUnsupported`, at the pipe's seam ruling (operand A)
  against the bend's torus face;
- at `ε = 1e-6` and `1e-9` the same band units build.

## The cause, as far as traced

Off a neutral point by `δ`, the seam ruling meets the bend's carrier
again at `s = √(12δ)` from its rim end. The end-root door
(`geom_brep::curve_face::end_root::end_window`) reads order 2 there, with
a window ending short of `√(12δ)`. So that simple root is the rest of
the span's root door's to find and place (Line × Torus,
`reduce::curved_face_arm`'s root lane). Its separation is not small at
the band: between the end and the root the composite peaks at `δ²/2`,
which is `30…100·Kε` across the strip. Not traced: which reading in the
Line × Torus lane refuses, or why only at `1e-12`.

## Done when

The pose builds at the closed form across the strip at `ε = 1e-12`.
The root at `√(12δ)` is found and placed off the bend's face, and there
is a row at `δ = 1e-5`.
