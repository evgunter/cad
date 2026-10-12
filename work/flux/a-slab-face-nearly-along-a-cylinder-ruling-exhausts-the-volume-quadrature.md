---
id: a-slab-face-nearly-along-a-cylinder-ruling-exhausts-the-volume-quadrature
kind: issue
title: A cylinder ∩ a slab whose face runs nearly along a ruling exhausts the volume quadrature, ending two ways
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## What

Found by the lever check of
`work/cleave/a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join.md`
(probe `sweep/examples/section_end_census_probe.rs`, family `bool`,
`walltilt t1e-2`). The unit cylinder (`0 ≤ z ≤ 1`) ∩ the slab
`[1 + dd, 3] × [−2, 2] × [0.25, 0.75]` turned `+1e-2` rad about `y`
through `(1 + dd, 0, 0.5)`, its inner face nearly along the seam ruling
and `dd` inside it, refuses in both orders at every ε row (`−1e-2`
builds, as do ∪ and ∖):

- `dd ∈ {−50ε, −1000ε}` at ε = 1e-9 and 1e-12, and `dd = −1e-3` at
  ε = 1e-6: the volume backstop, `VolumeUnmeasured { Face {
  QuadratureBudget { width_len ≈ 3.1e-9, target_len 1.024e-9, rounds:
  13 } } }` at 1e-12, with the last-resort ending;
- `dd = −5e-5` at ε = 1e-6: the at-rest gate, `ResultInvalid {
  VolumeUncomputable { Face { QuadratureBudget { width_len
  0.0024576868015994116, target_len 0.001024, rounds: 13 } } } }`, ending
  "There is no way through yet".

## Owed

Say whether this face's quadrature should converge (it is a thin
wall sliver between an elongated ellipse arc and the slab's rims), and
give the one budget refusal one ending at both readers.
