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
