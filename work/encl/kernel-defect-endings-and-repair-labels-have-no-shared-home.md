---
id: kernel-defect-endings-and-repair-labels-have-no-shared-home
kind: issue
title: geom-brep offset refusals spell the kernel-defect ending three ways and the repair two ways, with no shared home
status: open
opened: 2026-09-26
---


(ENCL implementer, filed from the fix pass of PR 3269 at its review's
request; the class, not one site.)

## What

A refusal the viewer shows ends in one of two shapes (the standard in
`work/chrome/error-and-check-text-overflows-its-region.md`, "The
standard a refusal is rewritten to"): a repair, or a plain statement
that there is none. Across the offset fit's carriers and the
checks window that summarises them, both halves are spelled by hand at
each site, and they disagree.

**The kernel-defect ending, three spellings:**

- `geom_brep::OffsetFitError::Elevation`'s `Display`
  (`crates/geom-brep/src/offset_fit.rs`): "There is no way through:
  this is a kernel defect; report the face's description".
- `topo::validate`'s `DEFECT` (`crates/topo/src/validate.rs`): "There
  is no way through: this is a kernel defect or a damaged file; report
  it".
- `geom_brep::patch_bound::PatchBoundError::note()`'s
  `RefinementFailed` and `DerivedKnots`
  (`crates/geom-brep/src/patch_bound.rs`): "…, which a valid face always
  allows: report the face's description", with no "There is no way
  through" marker, so `test_utils::refusal::recourse_markers` counts
  zero on them.

**The repair, two vocabularies:**

- `Recourse: …` after a sentence: every `OffsetFitError` arm that
  names a repair, and both `MeterError` repairs
  (`crates/geom-brep/src/offset_meters.rs`).
- A clause after a colon or dash, unlabelled: every
  `PatchBoundError::note()`, and the carriers the offset fit forwards
  whole (`geom::curves::fit::FitError`, `geom_core::spline::SplineError`:
  "— supply the missing samples; …").

## Why it matters

The checker the budget rows run counts markers, so an unlabelled repair
and a labelled one are different facts to it. A carrier whose repair is
unlabelled can therefore be forwarded under a wrapper that adds a
`Recourse:` of its own without the checker seeing two repairs, and a
kernel-defect ending without the marker reads to it as no ending at
all.

## Repair shape

`geom_core::predicate` already hosts the shared repair phrases
(`COINCIDENCE_RECOURSE`, `RANGE_RECOURSE`, `NO_DECLARATION_RECOURSE`,
`SPLIT_PLANE_RECOURSE`). A `KERNEL_DEFECT_RECOURSE` there (one
sentence, with the marker), used by the three sites above, and one
decision on whether a forwarded note carries its repair labelled, would
put both halves in one home. `predicate.rs` is PROPS's by
`work.py territory`, so the constant's landing touches their file.
