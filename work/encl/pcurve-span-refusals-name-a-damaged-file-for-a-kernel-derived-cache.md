---
id: pcurve-span-refusals-name-a-damaged-file-for-a-kernel-derived-cache
kind: issue
title: geom-brep: a PcurveCheck span refusal at rest ends in KERNEL_OR_FILE_DEFECT_ENDING, naming a damaged file for a cache the kernel derives
status: open
opened: 2026-10-14
---


(Filed by the ENCL implementer of `a-lever-decisions-definite-arm-at-rest-drops-its-note`, from the PR 5101 review. This predates that PR.)

## What

`PcurveCheck::recourse` (`crates/geom-brep/src/pcurve_cache.rs`) routes `PcurveCheck::ParamSpan` to `CertCheck::ParamSpan`, and `AzimuthPeriod`/`TubePeriod` to `CertCheck::ParamWinding`. Both are `StoredDefinite::Contradiction`. So a definite refusal read at rest ends in `KERNEL_OR_FILE_DEFECT_ENDING` ("this is a kernel defect or a damaged file"), and since PR 5101 it does so on either route. ParamSpan's stored `no_size` note names the file as well.

A pcurve is a cache the kernel derives from the edge and its faces, not data a file stores. A span or period refusal on it is a defect of the kernel's own derivation, so "or a damaged file" may over-name the file.

## Question

Should a pcurve check's at-rest ending name the kernel alone (`KERNEL_DEFECT_ENDING`, the build ending), for example through its own `SizedDecision` with `StoredDefinite::Lever`-like semantics or a reading fixed at `Reading::Build`? Or is a pcurve read back from a file (STEP p-curves) in some path? If it is, the file is the right thing to name there. First check which readers end a `PcurveCertifyError` at rest (`PcurveCertifyError::ending`).
