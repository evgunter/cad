---
id: the-edge-certificates-check-4-reads-at-a-computed-station
kind: issue
title: The edge certificate's check 4 reads its per-sample residuals at a station computed at the scalar, so a steep carrier's box carries the station's width
status: open
opened: 2026-11-12
priority: P3
cost: E
---


Filed by `pcert/cone-image-fine-band` (PCERT), from its class sweep.

## What

`certify.rs` `run_checks`, check 4, takes `let t = sample_param(t0, t1, i)`
at the certifying scalar and reads each description's per-sample
residual there: on an intersection, `implicit_residual(surf, carrier.eval(t))`
for both surfaces. At `Interval` an interior station is a few ulps wide,
so the carrier's box carries that width times the carrier's speed. A
spline carrier with dense knots also takes the station's width through
each de Boor level, scaled by its control coordinates. The implicit
residual's enclosure is then about as wide as that box.

The SSI certificate's limb 1 reads the same residual at held stations
(`ssi/certify.rs`, `held_station`). Its comment says this is so that
an interior station stays a point. Check 4 is the same read without
that hold.

The same shape is in `topo/src/boolean/contact_verify.rs`
(`let t = schedule_param(t0, t1, i, samples)`, then
`implicit_residual(s1, p)`).

## Evidence

PCERT's stated route had the identical defect on check 3, and a
correct cone image ended in its lever at ε 1e-12. A 3-ulp station gave
the carrier's point 3.7e-13 of width (`analytic_image_rows.rs`
`a_cone_image_at_a_steep_sample_certifies`). PCERT fixed only its own
route, with `Stations::Held`. Check 4 has not been seen red, so this
is P3.

## Fix direction

`run_checks` holds `CertifiedBounds`, so
`held_station(outer_ends((t0, t1)), (t0, t1), i, CERT_SAMPLES)` is
available there. The mid sample then becomes the `f64` middle entered
as a point, not `mid_param` at the scalar; check what reads that
equality (`schedule_param`'s doc: "the middle sample is the parameter
`Curve3::mid_point` evaluates at").
