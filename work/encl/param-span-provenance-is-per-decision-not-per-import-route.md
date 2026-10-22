---
id: param-span-provenance-is-per-decision-not-per-import-route
kind: issue
title: recourse: ParamSpan's Provenance is a per-decision constant, but whether its interval is derived or file-stated is a property of the import route
status: closed
opened: 2026-10-20
pr: 5464
branch: encl/sized-provenance
closed: 2026-10-20
---


(Filed by the ENCL implementer of `pcurve-span-refusals-name-a-damaged-file-for-a-kernel-derived-cache`, branch `encl/sized-provenance`, from review of PR 5464.)

## What

A sized decision's defect side carries its `Provenance` as a constant of the decision: `CertCheck::span` (`crates/geom-brep/src/certify.rs`) gives `CertCheck::ParamSpan` and `CertCheck::ParamWinding` `RefusedSide::Unminted(Provenance::Derived)`, and the import door's `culprit(at, provenance)` (`crates/geom-brep/src/recourse.rs`) reads it from there. Whether the interval a span check refuses is derived or file-stated is really a property of the import route that produced it:

- An analytic edge's interval is derived: `step-import`'s `endpoint_params` (`crates/step-import/src/geometry.rs`) projects the file's vertices, takes an eccentric anomaly or wraps an angle.
- A spline edge's interval is the file's stored knot domain verbatim (`endpoint_params`'s `Curve3::Nurbs` arm). That route compares stored values only; it is harmless today because a `KnotVector`'s domain is strictly forward, so it never refuses.
- A trimmed route is not supported: `Entities::curve` (`crates/step-import/src/entities.rs`, ~970) refuses `TRIMMED_CURVE` and `SURFACE_CURVE` as `StepImportError::UnsupportedEntity`.

## Why it matters

If `TRIMMED_CURVE` / `SURFACE_CURVE` support lands, the file states the trim parameters, so `ParamSpan`'s interval becomes file-stated on that route and a reversed or zero trim is the file's own contradiction. The per-decision `Provenance` cannot express that: `culprit(at, Provenance::Derived)` would keep naming the kernel beside the file for a contradiction of the file's own values. Whoever adds that route has to carry provenance per route (for example, on the refusal or on `ReadAt::File`) rather than per decision.

## Closed

2026-10-20, PR 5464: moot. Ev ruled on that PR that the import door names the file for every defect ending a bad file reaches, whether the comparand is stored or derived. `Provenance` is removed, so no per-decision provenance is left to be wrong per route. A `TRIMMED_CURVE` / `SURFACE_CURVE` route needs nothing extra: a reversed or zero trim it states already ends in `DAMAGED_FILE_ENDING` at the import door (`ReadAt::culprit`, `crates/geom-brep/src/recourse.rs`).
