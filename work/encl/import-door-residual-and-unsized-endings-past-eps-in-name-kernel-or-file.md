---
id: import-door-residual-and-unsized-endings-past-eps-in-name-kernel-or-file
kind: issue
title: geom-brep: at the import door past eps_in a definite residual or unsized refusal ends in KERNEL_OR_FILE_DEFECT_ENDING, not the door's one damaged-file refusal
status: closed
pr: 5454
opened: 2026-10-19
branch: encl/one-culprit-chooser
closed: 2026-10-20
---


(Filed by the ENCL implementer of PR 5349, from its review.)

## What

`[tolerance.recourse]` (iv) says a stored contradiction at the import door "is the file's: the import refuses typed, naming the file". PR 5349 lands that for sized decisions: past ε_in a definite reading on a `StoredContradiction` or `Unminted` side ends in `DAMAGED_FILE_ENDING` (`SizedDecision::words`, `crates/geom-brep/src/recourse.rs`).

Unsized decisions and residuals at the same door past ε_in still end in `KERNEL_OR_FILE_DEFECT_ENDING`:

- `crate::certify::recourse` (`crates/geom-brep/src/certify.rs`) sends `Ending::Residual` at `ReadAt::File` to `Unsized::residual_in_file`, whose fallback is `self.recourse(arm, ReadAt::File(file).reading())`, i.e. `Reading::AtRest`, and from there `defect_ending(AtRest)`. That is the ending for an `Unsized::Defect` miss past ε_in and for a definite `Unsized::Fit` miss past ε_in (which `Unsized::Fit`'s doc calls a stored contradiction).
- `Ending::Unsized` at `ReadAt::File` (`ChartImage`, `PlaneNurbsReportedTransversality`) goes to `Unsized::recourse(arm, AtRest)` and the same ending.

So the import door has two definite endings past ε_in, not the one damaged-file refusal the ruling describes.

## Why it was not routed on PR 5349

Routing `residual_in_file`'s fallback to `DAMAGED_FILE_ENDING` is not clearly right for every check that reaches it:

- `Unsized::Fit` covers misses where the kernel approximated (`MissSource::Fit`: "may be the kernel's own approximation"); a damaged-file ending would drop the kernel.
- `ChartImage` and `PlaneNurbsReportedTransversality` read a value the kernel derives at adoption, not one the file stores (the same question as `pcurve-span-refusals-name-a-damaged-file-for-a-kernel-derived-cache`).
- `Unsized::Defect` residuals over the file's own data (`EndpointStart`, `EndpointEnd`, `WitnessMidpoint`, `MappedSource`) are the case where "damaged file" is right.

## Repair shape

Split the import door's unsized endings by whose data the miss is (the file's, or the kernel's derivation at adoption), and end the file's in `DAMAGED_FILE_ENDING`. `a-defect-endings-culprit-has-two-choosers` is where that choice would live.

## Answer, 2026-10-20 (PR 5454)

The split finds no unsized check that reads the file's data alone at the import door, so all of them keep `KERNEL_OR_FILE_DEFECT_ENDING` past ε_in. That ending is now chosen by `recourse::culprit` for `Provenance::Derived` (`Unsized::recourse`, `crates/geom-brep/src/recourse.rs`). The repair shape above assumed the endpoint residuals were the file's; they are not:

- An ellipse edge's `t0`/`t1` are a scaled angle, not the nearest point (`work/interp/ellipse-edge-parameters-overstate-the-endpoint-miss.md`).
- A seam generator is a carrier the import mints through one rim vertex (`work/interp/minted-seam-generator-can-put-endpoint-end-two-eps-in-off.md`).
- A B-spline edge's parameters are pinned to its domain ends (`work/interp/partial-bspline-edges-are-refused-for-domain-pinned-parameters.md`).
- The witness is the carrier's mid-parameter point (`adopt.rs`), on such a carrier and at such parameters.

Only a sized decision's stored side names the file there (`SizedDecision::words`, `Provenance::Stored`).

Note (PR 5464, 2026-10-20): reversed. Ev ruled on PR 5464 that the import door names the file for every defect ending a bad file reaches, so these unsized endings past ε_in now end in `DAMAGED_FILE_ENDING` and `recourse::Provenance` is gone. (ENCL implementer)
