---
id: the-area-pass-never-refines-beside-a-converged-volume
kind: issue
title: The rational patch lane's area pass is fixed-resolution and never refines, so a face reported converged on its volume carries a percent-wide area
status: open
opened: 2026-10-21
priority: P2
cost: M
---


Found off-question by a designer weighing `quad2-rational-max-rounds-dial-decision`
(2026-10-21, on `quad/check7-sign-walk`); not yet reproduced by the filer.

The reporting target meters the FLUX only. In `crates/geom-brep/src/props/quad/`,
the area pass of the patch lanes runs at a fixed resolution
(`AREA_CELLS_PER_AXIS`) and never refines with the rounds. Measured on the
quarter torus: the area bracket is 0.47 m² wide (about ±2.4% of π² m²) on every
round, and it is reported beside a volume the lane calls converged. `area_pad`
is honest about the width; the "converged" label is not a statement about the
area.

Owed: either the area refines under its own target (and its own refusal or
bracket, per however the dial fork is ruled), or the result stops calling the
measurement converged where only the volume is. Read the fork's ruling first:
one designer's recommendation makes the bracket the primary value, which would
settle the label half.
