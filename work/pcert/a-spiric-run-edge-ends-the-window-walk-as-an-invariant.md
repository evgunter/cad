---
id: a-spiric-run-edge-ends-the-window-walk-as-an-invariant
kind: issue
title: A spiric run edge ends the window walk as SectionInvariant, though it is the torus's own curve and only the walk's lane for it is unwritten
status: open
opened: 2026-10-25
---


Filed by PCERT unit E1 (`pcert/lane-choices`), from the orchestrator's Q11 on PR #5500.

## What

`crates/topo/src/chord_join.rs`, `face_azimuth_images`: each run edge's image comes from `chart_pcurve_over`, and its azimuth extent is read with `chart_azimuth_range`, which is `Pcurve::closed_form_span_box` (`crates/geom-brep/src/pcurve_cache.rs`). That box exists only for `Harmonic` and `FocalSection`. When it is `None`, the walk reads the image kind:

- **`Projected` or `General`** (a circle's lane choice on a sphere, or a spline's only image): `SplitJoinError::RunImageNotYet`, which ends lever-only "not yet".
- **Every other image**: `SplitJoinError::SectionInvariant` ("curved-section invariant"). The only such image `chart_pcurve_over` returns is a spiric carrier's exact `Pcurve::Spiric`: a torus wall's plane section, on the torus or on the plane cap.

A spiric edge is one of the chart's own curves, imaged exactly, so nothing about it breaks an invariant. The window walk's lane for it (a certified azimuth box of the spiric image over a span) is just not written. Ending that as `SectionInvariant` names a kernel break where there is a capability gap, which is the same category error D3's lane-choice clause removes for class readings.

## Done when

Either:
- the walk reads a spiric image's azimuth extent, from a certified span box of `Pcurve::Spiric`, and a torus face bounded by a plane section divides again; or
- until then, a spiric run edge refuses as a capability gap that ends "not yet", with its own variant or with `RunImageNotYet` re-worded so it does not claim the edge is "none of its own curves".

Either way, a row cuts a torus twice through one wall face and pins the outcome at `f64` and at `Interval`.
