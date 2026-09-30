---
id: line-seam-boundary-row-refusal-discarded-as-iso-unsupported
kind: issue
title: pcert: the LINE-seam arm discards boundary_iso_u's refusal as IsoUnsupported
status: open
opened: 2026-09-29
---


(TRIM, from the `boundary-iso-doors-panic-before-they-can-refuse` lane's
sweep of `boundary_iso_*` consumers.)

## What

In `crates/geom-brep/src/pcurve_cache.rs`, the LINE-seam arm of the
iso certifier (the `boundary_iso_u(payload, end).map_err(|_| …)` call
near :3801) discards the door's `SplineError` and answers
`PcurveCertifyError::IsoUnsupported { what: "the chart's boundary row
failed to re-wrap as a curve (corrupt chart structure)" }`.

`IsoUnsupported` is documented as a *typed and permanent unsupported
class* ("never a runtime fallback (C5)"); a corrupt chart is not a
class, and the sibling arms in the same function family carry the
payload through `PcurveCertifyError::ChartRow { source }` (the
`boundary_iso_v` call near :3406, the `boundary_iso_u` call near
:3904, the one near :4041). Since the doors now refuse a net whose
length disagrees with its knots (`ControlCountMismatch` /
`WeightCountMismatch`) instead of panicking, the discarded payload is
a real one on every arm.

## Repair shape

`.map_err(|source| PcurveCertifyError::ChartRow { source })`, as its
siblings; S394 (closed) converted sites of this shape in this file,
and this one was not among them.
