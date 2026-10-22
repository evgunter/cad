---
id: at-rest-in-chart-has-no-description-only-form
kind: issue
title: EdgeCurveSpec::at_rest_in_chart has no description-only form, so describe_face_rim_at_rest clones a carrier to keep a description
status: open
opened: 2026-10-20
---


(Filed by the ENCL lane on `work/encl/the-stored-curve-read-ladder-is-written-three-times.md`.)

## What

`describe_face_rim_at_rest` (`crates/sweep/src/swept.rs`) reads each rim once through `topo::Body::stored_curve`, then builds `curve.restated_spec().at_rest_in_chart(chart, false).description`. `EdgeCurve::restated_spec` clones the carrier into the spec, and `EdgeCurveSpec::at_rest_in_chart` (`crates/geom-brep/src/certify.rs`) only rewrites the description, so the clone is dropped unread.

A description-only form would remove the clone. The scaffold arm's mapping would live in it, and `at_rest_in_chart` would call it. That form belongs in `certify.rs`, which another lane held while this row was worked. The caller would then be `EdgeDescriptionSpec::at_rest_in_chart(curve.restated_description(), chart, false)` or similar. The ENCL lane did not re-spell the arm in sweep, because that arm mints the `declared_by` record.
