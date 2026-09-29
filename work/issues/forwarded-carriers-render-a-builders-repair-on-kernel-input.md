---
id: forwarded-carriers-render-a-builders-repair-on-kernel-input
kind: issue
title: forwarded fit/spline carriers whose input the kernel supplied render a builder's repair labelled as a Recourse (step-import, ssi, skin, pcurve, compose)
status: open
opened: 2026-09-28
---


(ENCL implementer, from the review of the unit that gave the
kernel-defect ending one home, `geom_core::KERNEL_DEFECT_ENDING`. The
sites sit on several programs' ground, named per site; one row because
the repair is one shape.)

## What

`geom::curves::fit::FitError`, `geom_core::spline::SplineError` and
`KnotVectorIssue` label their repair `Recourse:`, and the repair is
addressed to a caller who SUPPLIED the samples, knots or points
("supply the missing samples", "ask on a domain whose ends are
finite…"). Where a refusal forwards one of them whole and the kernel,
not the user, supplied that input, the viewer shows a builder's repair
the user cannot act on, labelled as their recourse.
`geom_brep::OffsetFitError` had this shape on `Fit` and `Structure` and
now renders them as a kernel finding ending in `KERNEL_DEFECT_ENDING`,
with the carrier kept in `Debug`, the way its `Elevation` arm already did.

Every forwarded carrier whose input the kernel supplied:

- `crates/step-import/src/error.rs:448` `StepImportError::WallColumnStructure`
  (EXCH): forwards `{source}` mid-sentence, so its `Recourse:` label
  lands inside the sentence, then ends "No validated surface can be in
  that state, so the file is refused…": a repair and a dead end in one
  message, and the repair addresses a spline builder.
- `crates/geom-brep/src/ssi.rs:530` `SsiError::Fit` (SSI): "ssi: the
  fitting stack refused the marched trace: {e}"; the kernel marched the
  samples.
- `crates/sweep/src/skin.rs:224-226` `SkinError::Fit`, `KnotAlgebra`,
  `Structure` (CARVE): forwarded bare. The skin's own section checks
  (`TooFewSections`, `SectionShapeMismatch`) refuse the user's input
  first, so what reaches the fit is the kernel's compatible net.
- `crates/geom-brep/src/pcurve.rs:77-78` `PcurveError::Structure`, `Fit`
  (no owner): "pcurve: {e}"; the kernel sampled the edge.
- `crates/geom/src/curves/compose.rs:257-258` `ComposeError::Knots`,
  `Structure` (PROPS): the kernel computed the composed knot vector
  from curves that were valid.

## Repair shape

Render each as its own sentence naming what the kernel could not
build, ended in `geom_core::KERNEL_DEFECT_ENDING` (or
`KERNEL_OR_FILE_DEFECT_ENDING` for step-import, whose input is a
file), with the carrier kept in the variant's `Debug`, as
`OffsetFitError::Fit` does. Drop the stage prefixes (`ssi:`,
`pcurve:`) at the same time.
