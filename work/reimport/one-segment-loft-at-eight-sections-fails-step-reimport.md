---
id: one-segment-loft-at-eight-sections-fails-step-reimport
kind: issue
title: an 8-section degree-2 one-segment loft exports to STEP and fails re-import: edge #109 (the strut) has no intensional description that certifies
status: open
opened: 2026-10-10
priority: P2
cost: M
---

(NURBS lane, from the delta review of PR 4479. Measured, cause not
traced.)

## Finding

`sweep::loft_body` over a one-segment circle through 8 equally spaced
sections at degree 2 (`z = 0, 1, …, 7`) builds and certifies, exports
with `step_export::step_string`, and fails `step_import::import_step`:

```text
step import: edge #109: no intensional description certifies (a lever
applies to the model the file was exported from, which is then
re-exported)
```

Edge #109's curve is `#108`, the wall's wrap strut (a rational-form
`B_SPLINE_CURVE_WITH_KNOTS` of degree 2 with knots
`(-1, -0.7857…, -0.6428…, -0.5, -0.3571…, -0.2142…, 0)`, multiplicities
`(3, 1, 1, 1, 1, 1, 3)`). The same fixture at 4 sections (degree 1),
`[0, 1, 3]` (degree 1) and 2 sections round-trips.

The reviewer reports the failure identical on the head before PR 4479,
where the strut was stated on `[0, 1]` with rounded mirrored knots, so
the negated domain is not its cause.

## Evidence

The probe `review_probe_one_segment_loft_step_round_trip`
(`crates/step-import/tests/review_4479_probe.rs` on branch
`nurbs/review-4479-delta-k3`, commit `d771fd593`) runs the four lofts
and prints each export's B-spline lines. Re-run it there, or copy the
file into `crates/step-import/tests/` with a `mod` line in `all.rs`.

## Where to look

The importer's description ladder for a wrap edge (`step-import`'s
`adopt.rs`, the IsoCurve rung and its neighbours): which rung the
strut reaches at 8 sections and why it does not certify — the section
count changes the strut's knot vector (and the wall's `knots_v`) from
the dyadic or few-knot cases that round-trip.

