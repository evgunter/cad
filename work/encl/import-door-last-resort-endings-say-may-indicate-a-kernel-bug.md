---
id: import-door-last-resort-endings-say-may-indicate-a-kernel-bug
kind: issue
title: recourse: at the import door a last-resort ending says the refusal may indicate a kernel bug, over a reading a bad file reaches
status: open
opened: 2026-10-20
---


(Filed by the ENCL implementer of PR 5464, `encl/sized-provenance`, from that PR's fix pass.)

## What

Ev ruled on PR 5464 that the import door names the file for every defect ending a bad file reaches, and `[tolerance.recourse]` (iv) now says so for defect endings. The last resort is not a defect ending, so the PR left it alone. At the import door (`ReadAt::File`) it still says "Recourse: loosen the tolerance, as a last resort; this refusal may indicate a kernel bug worth reporting" (`KERNEL_LIMIT_RECOURSE`, `crates/geom-core/src/predicate.rs`) over readings a bad file can reach:

- **`Unsized::Bound`** (`crates/geom-brep/src/recourse.rs`, `Unsized::recourse`). It ends in the last resort on every arm at every door. `CertCheck::TangentHull`, `PlaneNurbsHull`, `PlaneNurbsHullValue` and `AnalyticHull` are bounds. Past ε_in, or only partly within it, a bound at the door ends in `KERNEL_LIMIT_RECOURSE` (`a_certified_bound_refusal_is_the_certificates_limit`, `crates/geom-brep/src/certify.rs`).
- **`Unsized::Fit` on an undecided arm or a straddle** (`Unsized::recourse`). It ends in the last resort at every door but a build. `SurfaceResidual`, `PlaneNurbsOnLocus` and `AnalyticBreakResidual` are fits.
- **Within ε_in, a fit's miss** (`FileCoincidence::miss_recourse_in_file`, `MissSource::Fit`). It ends "...; this refusal may indicate a kernel bug worth reporting", as does a bound wholly within ε_in (`MissReading::Bound`).

A file whose geometry is off by more than the kernel's approximation can reach each of these. For example, a NURBS wall and a plane whose stated intersection is off makes `PlaneNurbsOnLocus`'s fit miss, and the import door then suggests a kernel bug.

## Finding

The last resort's kernel-bug note is honest about one of its two causes at the import door. D4 ¶1 (i) gives the last resort because no lever reaches a kernel approximation's limit. At the import door a bad file is a second cause, and the door cannot tell the two apart. The note names only the first.

## Options

- **A. Leave it.** The last resort names the approximation's limit and loosening ε, both true at the door. A bad file is not excluded, only unnamed.
- **B. Name the file beside the kernel at the import door.** For example, an import-door last resort ending "…this refusal may indicate a kernel bug or a damaged file". It would read the door through `ReadAt::culprit`, as the defect endings do.
- **C. Name the file alone at the import door, as the defect endings now do.** This drops the approximation's limit, which is still a live cause, so it says less than is known.
