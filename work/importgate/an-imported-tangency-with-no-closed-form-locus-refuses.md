---
id: an-imported-tangency-with-no-closed-form-locus-refuses
kind: issue
title: An imported tangent edge whose pair has no closed-form tangent locus (a spline wall, an off-axis pose) refuses not-yet-supported
status: open
opened: 2026-11-18
---



Found by `importgate/build` (PR 7128).

## What

An imported tangent edge's carrier is built as the pair's tangent locus:
`geom_brep::tangent_locus` (a plane and a cylinder, two parallel
cylinders) or `geom_brep::coaxial_tangent_circle` (the parallel of a
coaxial pair). A tangency of any other pose or pair, and a tangency
against a spline wall (which the file band reads from the section's
`Tangent` verdict), has no construction, and refuses
`StepImportError::NotYetSupported` naming this row (`TANGENT_LANE` in
`crates/step-import/src/build.rs`). No file of the committed corpus
reaches it at PR 7128's head.

## What would close it

C7 tangent-locus constructions for the poses a file states: the
off-axis quadric pairs, and spline × implicit and spline × spline
contact curves.
