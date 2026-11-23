---
id: a-spline-crossing-at-a-dual-is-not-built
kind: issue
title: A spline crossing at a dual refuses not-implemented: run the certified search on the value channel and lift each root by the implicit-function tangent
status: open
priority: P2
cost: H
opened: 2026-11-23
---


Ruled on PR 7565 (Ev, 2026-11-23): "a not implemented error (of the correct type so it can be found later) is fine at runtime". U4d builds the typed not-implemented refusal at a dual; this row is the real meaning.

## What it is

- A spline arm of `curve_face::cross` (`composite::crossings`; `spline_face::isolate`/`settle`) has a meaning at a dual. The certified search runs on the dual's value channel (`T::Value`), which is bit-identical to the f64 run, so its decisions, count and roots are the f64 run's.
- Each admitted root keeps that value exactly and carries its tangent by the implicit-function theorem:
  - `dt/dp = −(∂F/∂p)/(∂F/∂t)` for a spline curve against an analytic face;
  - `−J⁻¹·∂R/∂p` with `J = [C′ | −S_u | −S_v]` for a span against a spline chart.
- The arms already certify what the theorem needs: `composite` a strictly one-signed slope, `spline_face` a Krawczyk-invertible Jacobian.
- At `Dual<Interval>` the tangent is evaluated over the root's certified box.

## Needs

- `Decide`'s `Value: Certifies` stated as a bound.
- A dual half of the admit door. This changes DL5.2's sentence "so a dual cannot reach it", which is ratified text and needs Ev's sign-off.
- Suite rows checking the tangents against finite differences (fork 293's outcome).

## Settles when

A spline arm at a dual answers the f64 run's meetings with tangents, and the not-implemented refusal is gone.

## Related

The general-rung march (`AtRestPolicy::ssi_lane`, `NoMarch` at every scalar but f64) is the same class. Without it, a dual spline boolean still refuses one step later at the plane × spline section.

Signed: (GERM orchestrator)
