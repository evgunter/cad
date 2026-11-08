---
id: analytic-by-spline-edges-refuse-as-unimplemented-at-resolve
kind: issue
title: a non-plane analytic surface paired with a spline refuses at resolve as Unimplemented, whatever its carrier, because the chart tube is written for a plane only
status: open
opened: 2026-10-13
priority: P3
cost: M
---


Split out of `the-spline-pair-edge-certificates-refuse-a-closed-form-carrier`
by both designers who weighed it (2026-10-13): it is a gap in which
surface pair is certified, not in which carrier.

## What happens

`geom_brep::certify::run_checks`'s `resolve` closure answers a non-plane analytic surface
(cylinder, cone, sphere, torus) paired with a spline as
`CertifyError::Unimplemented`, for a spline carrier as well as a closed
form. The uniqueness tube a spline pair uses is the chart tube, written
for a plane only: `ssi::certify::CHART_TUBE_NEEDS_PLANE`, which
`certify_branch` and `ssi`'s search both raise.

## What closes it

Either a chart tube for an analytic operand, or a named, typed refusal
for the pair at `resolve` (with the tube's limit as its reason) in
place of `Unimplemented`, which reads as "nobody wrote this".

## The tube half is built (2026-11-02)

The chart tube now reads a cylinder, a sphere and a ring torus as well as
a plane (`ssi::enclose::ChartPhi`, SECTARMS's NURBS × implicit stage 3a),
and `CHART_TUBE_NEEDS_PLANE` is gone: `certify_branch` raises
`CHART_TUBE_NEEDS_FORM` only for a partner with no form it reads (a cone,
a spindle torus, or one not exact at `f64`). The refusal at `resolve`
stands as written: `resolve` still refuses the spline side of a
non-plane analytic × spline pair before any tube is read, so what closes
this row now is a `Resolved` arm for that pair beside `PlaneNurbs`, or
the typed refusal.
