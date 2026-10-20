---
id: param-span-provenance-is-per-decision-not-per-import-route
kind: issue
title: recourse: ParamSpan's Provenance is a per-decision constant, but whether its interval is derived or file-stated is a property of the import route
status: open
opened: 2026-10-20
---


(Filed by the ENCL implementer of `pcurve-span-refusals-name-a-damaged-file-for-a-kernel-derived-cache`, branch `encl/sized-provenance`, from review of PR 5464.)

## What

A sized decision's defect side carries its `Provenance` as a constant of the decision: `CertCheck::span` (`crates/geom-brep/src/certify.rs`) gives `CertCheck::ParamSpan` and `CertCheck::ParamWinding` `RefusedSide::Unminted(Provenance::Derived)`, and the import door's `culprit(at, provenance)` (`crates/geom-brep/src/recourse.rs`) reads it from there. Whether the interval a span check refuses is derived or file-stated is really a property of the import route that produced it:

- An analytic edge's interval is derived: `step-import`'s `endpoint_params` (`crates/step-import/src/geometry.rs`) projects the file's vertices, takes an eccentric anomaly or wraps an angle.
- A spline edge's interval is the file's stored knot domain verbatim (`endpoint_params`'s `Curve3::Nurbs` arm). That route compares stored values only; it is harmless today because a `KnotVector`'s domain is strictly forward, so it never refuses.
- A trimmed route is not supported: `Entities::curve` (`crates/step-import/src/entities.rs`, ~970) refuses `TRIMMED_CURVE` and `SURFACE_CURVE` as `StepImportError::UnsupportedEntity`.

## Why it matters

If `TRIMMED_CURVE` / `SURFACE_CURVE` support lands, the file states the trim parameters, so `ParamSpan`'s interval becomes file-stated on that route and a reversed or zero trim is the file's own contradiction. The per-decision `Provenance` cannot express that: `culprit(at, Provenance::Derived)` would keep naming the kernel beside the file for a contradiction of the file's own values. Whoever adds that route has to carry provenance per route (for example, on the refusal or on `ReadAt::File`) rather than per decision.
