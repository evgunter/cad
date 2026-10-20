---
id: no-sized-decision-names-the-file-alone-at-the-import-door
kind: issue
title: recourse: no sized decision reads the file's own data at the import door, so DESIGN (iv)'s 'it is the file's' names no decision
status: open
needs_ev: true
opened: 2026-10-20
---


(Filed by the ENCL implementer of `pcurve-span-refusals-name-a-damaged-file-for-a-kernel-derived-cache`, branch `encl/sized-provenance`.)

## What

A sized decision's defect side now names its provenance (`RefusedSide::StoredContradiction(Provenance)`, `RefusedSide::Unminted(Provenance)`, `crates/geom-brep/src/recourse.rs`). Checked one by one, every defect side in the codebase is `Provenance::Derived` at the import door:

- `CertCheck::ParamSpan`, `CertCheck::ParamWinding` and a door's `CertifyError::StatedInterval` read the interval `step-import`'s `endpoint_params` (`crates/step-import/src/geometry.rs`) computes from the file's vertices. A spline edge's interval is the exception: `endpoint_params`'s `Curve3::Nurbs` arm returns the file's stored knot domain verbatim, a route that compares stored values only and never refuses, because a `KnotVector`'s domain is strictly forward (`param-span-provenance-is-per-decision-not-per-import-route`).
- `CertCheck::ImageSpan` and the `PcurveCheck` span and period gates (`PcurveCheck::routing`, `crates/geom-brep/src/pcurve_cache.rs`) read a chart image the kernel derives.
- `CertCheck::Transversality` and `CertCheck::TangentSecondOrder` refuse the adoption ladder's candidate description (`adopt_edges`, `crates/step-import/src/adopt.rs`), sampled at those derived parameters.

Every unsized decision was already `Derived` (PR 5454). So no production reading reaches `Provenance::Stored`, `StoredCulprit::File` or `DAMAGED_FILE_ENDING`.

## Question

`docs/DESIGN.md` `[tolerance.recourse]` (iv) says a stored contradiction "at the import door ... is the file's: the import refuses typed, naming the file", and that an unminted side "at the import door is the file's like any other". Its premise is a contradiction between a stored description and its stored geometry, which no import-door reading is today, so the code applies the clause rather than departing from it. But the clause now names a case no decision reaches.

Ev's call: should (iv) say the import door names the file only where the file's own values contradict, and the kernel beside the file where adoption derives a comparand? Or should the import door gain a decision that reads the file's values alone (for example, a check on stored vertices against a stored carrier before `endpoint_params` runs), so the file-alone ending has a reader? Until then, `Provenance::Stored` is exercised only by tests (`one_definite_reading_ends_one_way_whichever_arm_routes_it`, `recourse.rs`).
