---
id: the-depth-gaps-least-point-is-placed-over-axis-dot-n-uncharged
kind: issue
title: depth_gap::least_point places an axis's pierce of a plane at (o − origin)·n / (axis·n) from two stored origins, uncharged: rounding |o − origin|·u over the cosine
status: open
opened: 2026-10-19
priority: P3
cost: E
---



Found by the sweep of `restread/ef-pierce-charge` (census closed forms
read from a stored origin). Suspected, not measured.

## What

`crates/topo/src/census/depth_gap.rs` `least_point` places an axis
depth's least point on a plane at `origin + axis·s`,
`s = (o − origin)·n / (axis·n)`: `o` the plane's stored origin and
`origin` the axis's, both possibly metres from where the axis meets the
plane. The numerator rounds to about `u·|o − origin|`, and `axis·n` is
the cosine between the axis and the plane's normal, small where the axis
lies nearly in the plane. So the point is off by about
`u·|o − origin|/|axis·n|` along the axis, uncharged. `least_on` (its
one caller) reads the point through `face_placement` and keeps the
cover's boundary reading only where the face reads it `Out`, so a point
moved across the face's boundary turns which reading the depth gap
takes.

## Owed

Measure: an axis a few `Kε` off parallel to a face whose stored origin
is metres from the pierce, at ε = 1e-12, `f64` against `Interval`. If
the placement can cross the face's boundary, charge it before
`face_placement` reads it, or read `s` from a point of the axis near the
plane.
