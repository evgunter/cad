---
id: rational-pieces-force-rows
kind: issue
title: The rational arm's 16 dyadic reading pieces of one degree-1 span each force a lattice row, so lofted rational walls carry ~2.3x the rows their whole-patch step asks for
status: open
opened: 2026-10-20
priority: P3
cost: M
---


## What

On a rational face, `v_cuts` includes the `RATIONAL_CERT_SPLITS = 16` dyadic reading pieces of a single degree-1 span, and `NurbsCellGrid::band_schedule` lands a row on every cut (`nvc = 1` each). On a lofted wall the whole-patch step asks for 7 rows, but the lane emits 16: about 2.3× the triangles, with no `v` gain to pay for them. `v_cuts` means two things: "where the bound is read" and "a mandatory row". Only the first is needed.

Found off-question by a designer on `lofted-rational-walls-refuse-at-some-deltas` (2026-10-20). Not verified by a lane yet.

## Fix direction

Let rows skip reading cuts (`NurbsCellGrid::cert` already handles boxes straddling cells), or coalesce pieces of equal bound before scheduling. Witness: the triangle count on a lofted rational wall at fixed δ falls, the falsifier stays green, and crease cuts (multiplicity ≥ degree) still force rows.

## Moved to TESSLATTICE (the NURBS lattice: its share, its rows and cells, and what it costs on swept and lofted walls) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
