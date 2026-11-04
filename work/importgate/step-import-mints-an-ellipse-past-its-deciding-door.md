---
id: step-import-mints-an-ellipse-past-its-deciding-door
kind: issue
title: step-import mints ELLIPSE through Ellipse::new, past Curve3::ellipse's refusal of a non-positive semi-axis
status: open
opened: 2026-11-03
---


Found by ONECOPY `onecopy/angle-reads`.

`crates/step-import/src/entities.rs`'s `ELLIPSE` arm mints
`Curve3::Ellipse(geom::Ellipse::new(..))`, which is unchecked;
`Curve3::ellipse` is the deciding door that refuses a non-finite or
non-positive semi-axis (`EllipseInvalid`). A file stating
`ELLIPSE('',#25,0.5,0.)` was refused only by accident: `Conic::angle_of`
divided by the semi-axis and `endpoint_params`' finiteness check caught
the `0/0`. The angle-reads unit retired `angle_of` for `Conic::azimuth`,
which reads `(y·a, x·b)` with no division, so that route now refuses
later, at adoption (`StatedInterval`, a zero-length interval:
`step-import/tests/probe_refusals.rs`
`a_conic_self_loop_with_a_nonfinite_angle_refuses_typed` pins it there).

## Fix

Mint the ellipse through `Curve3::ellipse` and map `EllipseInvalid` to a
typed refusal naming the `ELLIPSE` record; move the probe's zero
semi-axis route onto that refusal.
