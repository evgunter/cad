---
id: pcurve-span-refusals-name-a-damaged-file-for-a-kernel-derived-cache
kind: issue
title: geom-brep: a PcurveCheck span refusal at rest ends in KERNEL_OR_FILE_DEFECT_ENDING, naming a damaged file for a cache the kernel derives
status: closed
pr: 5464
opened: 2026-10-14
branch: encl/sized-provenance
closed: 2026-10-20
---


(Filed by the ENCL implementer of `a-lever-decisions-definite-arm-at-rest-drops-its-note`, from the PR 5101 review. This predates that PR.)

## What

`PcurveCheck::recourse` (`crates/geom-brep/src/pcurve_cache.rs`) routes `PcurveCheck::ParamSpan` to `CertCheck::ParamSpan`, and `AzimuthPeriod`/`TubePeriod` to `CertCheck::ParamWinding`. Both are `StoredDefinite::Contradiction`. So a definite refusal read at rest ends in `KERNEL_OR_FILE_DEFECT_ENDING` ("this is a kernel defect or a damaged file"), and since PR 5101 it does so on either route. ParamSpan's stored `no_size` note names the file as well.

A pcurve is a cache the kernel derives from the edge and its faces, not data a file stores. A span or period refusal on it is a defect of the kernel's own derivation, so "or a damaged file" may over-name the file.

## Question

Should a pcurve check's at-rest ending name the kernel alone (`KERNEL_DEFECT_ENDING`, the build ending), for example through its own `SizedDecision` with `StoredDefinite::Lever`-like semantics or a reading fixed at `Reading::Build`? Or is a pcurve read back from a file (STEP p-curves) in some path? If it is, the file is the right thing to name there. First check which readers end a `PcurveCertifyError` at rest (`PcurveCertifyError::ending`).

## Evidence, 2026-10-19 (PR 5349)

`StoredDefinite` is gone; a sized decision now names its `RefusedSide` (`crates/geom-brep/src/recourse.rs`). What holds now:

- `PcurveCheck::ParamSpan` still routes to `CertCheck::ParamSpan`, now `RefusedSide::Unminted`; `AzimuthPeriod`/`TubePeriod` still route to `CertCheck::ParamWinding`, now `RefusedSide::StoredContradiction` (`PcurveCheck`'s routing in `crates/geom-brep/src/pcurve_cache.rs`).
- At rest both sides end in `KERNEL_OR_FILE_DEFECT_ENDING` (`SizedDecision::words`, `RefusedReading::ContradictsStoredData`), so the finding stands: a cache the kernel derives is still told it may be a damaged file.
- `ParamSpan`'s note is one `Option` now, read only where the reading ends in the lever (a build). Its text names the kernel alone ("is one no kernel construction mints, so this is a kernel defect worth reporting"); the stored note naming the file is gone, and at rest no note is read.
- `PcurveCertifyError::ending` takes a `Reading`, not a `ReadAt`, so a pcurve refusal never reaches the import door's `DAMAGED_FILE_ENDING`.
- `work/encl/at-rest-stored-contradictions-and-unminted-sides-panic-once-import-validates-everything.md` (parked) would turn these at-rest endings into a panic, which settles the wording.

## Evidence, 2026-10-19 (`encl/param-span-doors`)

`PcurveCheck::ParamSpan` no longer routes to `CertCheck::ParamSpan`, nor `AzimuthPeriod`/`TubePeriod` to `CertCheck::ParamWinding`: both edge decisions are `RefusedSide::Unminted` now and panic at a build, which an image's span can reach from input (`work/pctail/recourse-chain-stops-at-pcurve-certify-error.md`, its 2026-10-19 evidence). The image checks end through the span decisions read on the image (`PcurveCheck::routing`, `crates/geom-brep/src/pcurve_cache.rs`): the span through `SpanDecision::Forward`, as `CertCheck::ImageSpan` does where an arc rim's escalation names it, and the period gates through `SpanDecision::Winding`, both `RefusedSide::StoredContradiction` with no note: the lever alone at a build, `KERNEL_OR_FILE_DEFECT_ENDING` at rest. The finding stands at rest; the question can now be answered on those two decisions alone, without touching the edge's.

Note (PR 5393, 2026-10-19): rendering an `Unminted` refusal at a build no longer panics in geom-brep. The kernel-bug panic exists only at topo's `policy_lane::kernels_own`. Earlier notes here that cite the build panic in `SizedDecision::recourse` describe the state before this PR. (ENCL orchestrator)

Note (PR 5401, 2026-10-19): `policy_lane::kernels_own` is gone. The kernel-bug panic now lives in `policy_lane::Origin::read`: every topo edge certification passes an `Origin`, and `Origin::Kernels(site)` panics on a span refusal. (ENCL implementer)

## Evidence, 2026-10-20 (`encl/one-culprit-chooser`)

One chooser now names every defect ending's culprit: `recourse::culprit(at, provenance)` (`crates/geom-brep/src/recourse.rs`). A sized decision's `StoredContradiction` and `Unminted` sides read it as `Provenance::Stored`, for every sized decision alike (`SizedDecision::words`). Every decision with no size reads `Provenance::Derived` (`Unsized::recourse`), which names the kernel beside the file.

`CertCheck::ImageSpan` is sized with `RefusedSide::StoredContradiction`, so past ε_in at the import door it still ends in `DAMAGED_FILE_ENDING`. That holds even though the chart image it reads is a value the kernel derives (the same mint `CertCheck::ChartImage` reads, as `Derived`). That is this row's finding at the import door as well as at rest. The repair has a seam now: let a sized decision state its provenance, instead of `words` fixing `Stored` for both sides.

## Evidence, 2026-10-20 (`encl/sized-provenance`)

A sized decision's defect side carries its own provenance: `RefusedSide::StoredContradiction(Provenance)` and `RefusedSide::Unminted(Provenance)`, read by `SizedDecision::words` through `recourse::culprit`. No side can be written without one. `CertCheck::ImageSpan` and `PcurveCheck`'s span and period routes are `Provenance::Derived`, so past ε_in at the import door they end in `KERNEL_OR_FILE_DEFECT_ENDING`. The edge's span checks, `StatedInterval`, `Transversality` and `TangentSecondOrder` are `Derived` too (the PR body gives each argument). At rest nothing moves: `Reading::culprit` takes no provenance, so the at-rest wording stays with the parked `at-rest-stored-contradictions-and-unminted-sides-panic-once-import-validates-everything`. That no sized decision is left naming the file alone is filed as `no-sized-decision-names-the-file-alone-at-the-import-door`.

## Closed

2026-10-20, PR 5464. Ev ruled on that PR that the import door names the file wherever a bad file can reach the refusal. So the row's premise, that naming the file for a value the kernel derives over-names it, does not decide the wording. A sound file refused there is a kernel bug to fix.

- **What the PR first did.** It gave each sized defect side a `Provenance` and named the kernel beside the file at the import door for the derived ones, this row's checks included.
- **What it landed.** `Provenance` is removed. `CertCheck::ImageSpan` and the `PcurveCheck` span and period routes are plain `RefusedSide::StoredContradiction`. Past ε_in at the import door they end in `DAMAGED_FILE_ENDING`, as every defect ending there does (`ReadAt::culprit`).
- **At rest nothing moved.** `KERNEL_OR_FILE_DEFECT_ENDING` stays, and `PcurveCertifyError::ending` still takes a `Reading`. The at-rest wording stays with the parked `at-rest-stored-contradictions-and-unminted-sides-panic-once-import-validates-everything`.
