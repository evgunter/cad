---
id: import-door-provenance-of-a-fit-residual-is-unsettled
kind: issue
title: geom-brep: at the import door a fit residual's provenance is Unsettled, though the carrier and surfaces it compares are the file's
status: closed
pr: 5454
opened: 2026-10-20
branch: encl/one-culprit-chooser
closed: 2026-10-20
---


(Filed by the ENCL implementer of `a-defect-endings-culprit-has-two-choosers`, from its provenance table.)

## What

`CertCheck::ending` (`crates/geom-brep/src/certify.rs`) gives every decision with no size a `recourse::Provenance`: whose data its contradiction is about at the import door. `recourse::culprit` reads it there, past ε_in: `Stored` ends in `DAMAGED_FILE_ENDING`, `Derived` and `Unsettled` in `KERNEL_OR_FILE_DEFECT_ENDING`.

The three `Unsized::Fit` residuals are `Unsettled`: `SurfaceResidual`, `PlaneNurbsOnLocus` and `AnalyticBreakResidual`. The routing reads the decision alone, and at a build the carrier it measures may be the kernel's fit. At the import door, though, the adoption ladder certifies the file's parsed carrier against the file's own surfaces (`crates/step-import/src/adopt.rs`, module docs). So a definite surface-residual miss past ε_in there is plausibly the file's data contradicting itself, which `[tolerance.recourse]` (iv) says names the file.

It was not routed on that PR because `Unsized::Fit` keeps its ending at every door. A damaged-file ending would drop the kernel's own evaluation: a foot-point search on a NURBS wall, and limb 2's composite at a break.

## Question

Can the import door tell a surface residual over the file's carrier (an exact evaluation of stored data) from one over a kernel approximation? For example, could the provenance be read per carrier kind, or per lane? If it can, the first becomes `Provenance::Stored`. If it cannot, `Unsettled` is the answer, and this row closes with that recorded.

## Answer, 2026-10-20 (PR 5454)

It cannot tell them apart. At the import door even an analytic carrier's parameters, and sometimes the carrier itself, are the kernel's: a scaled ellipse angle, B-spline domain ends, a minted seam generator (`work/encl/import-door-residual-and-unsized-endings-past-eps-in-name-kernel-or-file.md`, its answer). `Provenance::Unsettled` is retired. Every unsized check, these three fits included, reads `Provenance::Derived` there (`Unsized::recourse`, `crates/geom-brep/src/recourse.rs`), which names the kernel or the file.
