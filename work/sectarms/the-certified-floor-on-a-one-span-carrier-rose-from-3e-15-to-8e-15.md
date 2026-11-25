---
id: the-certified-floor-on-a-one-span-carrier-rose-from-3e-15-to-8e-15
kind: issue
title: The certified envelope floor on a one-span carrier reads 8e-15 m where it read 3e-15: the one image function's chain there is looser than the foot schedule's was
status: open
opened: 2026-11-25
---


Filed by the `sectarms/one-image-function` lane (PR 7537), which moved
the pin.

## What

`crates/geom-brep/tests/r1_pxn_probes.rs`, `ENVELOPE_FLOOR`: the plane
lane's certified `hull_sup` for a carrier lying exactly on both operands
(the quarter cylinder's ruling, a one-span cubic, amplitude 0) reads
4.108e-15 m, where it read 1.443e-15 m while the wall's image was the
degree-1 interpolant of the 33-foot schedule. The constant went from
3e-15 to 8e-15 with it, and `RATIO_ARM_FROM` from 1.2e-13 to 3.2e-13, so
the `a = 1e-13` rung left the additive arm's reach.

The carrier's image is now the cubic on the carrier's own knots: one
span, four feet, where the schedule's image had 32 spans. The envelope's
noise over the wall's rational control net is read through that one
cubic span, and is 2.8 times looser. Nothing certifies wrongly: the true
sup is zero and both readings bound it.

## Done when

Either the one-span reading is shown to be the enclosure's honest floor
for this image (the composite's own width at one span, with the
arithmetic that gives 4.1e-15), or limb 2 reads a one-span image at a
finer cut and the pin returns toward 3e-15.
